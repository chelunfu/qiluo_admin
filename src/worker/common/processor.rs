use super::Result;
use super::{periodic::PeriodicJob, Job, Scheduled, UnitOfWork, Worker, WorkerRef};
use crate::cache::CacheManager;
use std::collections::BTreeMap;
use std::sync::Arc;
use tokio::select;
use tokio::task::JoinSet;
use tokio_util::sync::CancellationToken;
use tracing::{debug, error};
use chrono::{DateTime, Duration, Utc};
#[derive(Clone, Eq, PartialEq, Debug)]
pub enum WorkFetcher {
    NoWorkFound,
    Done,
}

#[derive(Clone)]
pub struct Processor {
    queues: Vec<String>,
    periodic_jobs: Vec<PeriodicJob>,
    workers: BTreeMap<String, Arc<WorkerRef>>,
    cancellation_token: CancellationToken,
    num_workers: u16,
}

impl Processor {
    pub fn new(queues: Vec<String>, num_workers: u16) -> Self {
        Self {
            workers: BTreeMap::new(),
            periodic_jobs: vec![],
            queues: queues
                .iter()
                .map(|queue| format!("queue:{queue}"))
                .collect(),
            cancellation_token: CancellationToken::new(),
            num_workers,
        }
    }

    async fn fetch(&mut self) -> Result<Option<UnitOfWork>> {
        let cache = CacheManager::instance().await;
        let response: Option<(String, String)> = cache.brpop(self.queues.clone(), 2).await?;

        if let Some((queue, job_raw)) = response {
            let job: Job = serde_json::from_str(&job_raw)?;
            return Ok(Some(UnitOfWork { queue, job }));
        }

        Ok(None)
    }

    pub async fn process_one(&mut self) -> Result<()> {
        loop {
            if self.cancellation_token.is_cancelled() {
                return Ok(());
            }

            if let WorkFetcher::NoWorkFound = self.process_one_tick_once().await? {
                continue;
            }

            return Ok(());
        }
    }

    pub async fn process_one_tick_once(&mut self) -> Result<WorkFetcher> {
        let work = self.fetch().await?;
        if work.is_none() {
            tokio::task::yield_now().await;
            return Ok(WorkFetcher::NoWorkFound);
        }
        let mut work = work.expect("polled and found some work");

        if let Some(worker) = self.workers.get_mut(&work.job.class) {
            let worker = worker.clone();
            match worker.call(work.job.args.clone()).await {
                Ok(_) => {
                  
                }
                Err(err) => { 
                    let mut job = work.job.clone();
                    job.error_message = Some(format!("{:?}", err));
                    let retry_count = job.retry_count.unwrap_or(0) + 1;
                    if job.retry_count.is_some() {
                        job.retried_at = Some(Utc::now().timestamp() as f64);
                    } else {
                        job.failed_at = Some(Utc::now().timestamp() as f64);
                    }
                    job.retry_count = Some(retry_count);
 
                    let max = worker.max_retries();
                    if retry_count > max {
                        error!({
                        "status" = "fail",
                        "class"  = &job.class,
                        "queue"  = &job.queue,
                        "jid"    = &job.jid,
                        "err"    = &job.error_message
                    }, "Max retries exceeded, will not reschedule job");
               
                    } else { 
                        let at = Self::calc_next_retry_at(retry_count);
                        Self::schedule_retry(&job, at).await?;
                    }
                }
            }
        } else { 
            error!({
            "status" = "fail",
            "class"  = &work.job.class,
            "queue"  = &work.job.queue,
            "jid"    = &work.job.jid
        }, "Worker not found, re-enqueue immediately");
            work.reenqueue().await?;
        }

        Ok(WorkFetcher::Done)
    }
    fn calc_next_retry_at(retry_count: usize) -> chrono::DateTime<Utc> {
        let exp = retry_count.min(10) as u32;
        let delay_secs = (1_i64 << exp) * 5;
        let delay_secs = delay_secs.min(3600);
        Utc::now() + Duration::seconds(delay_secs)
    }
 
    async fn schedule_retry(job: &Job, at: DateTime<Utc>) -> Result<()> {
        let cache = CacheManager::instance().await;
        let payload = serde_json::to_string(job)?;
        let score = at.timestamp_millis() as f64;

        cache.zadd("retry", payload, score).await?;
        Ok(())
    }
    pub fn register<
        Args: Sync + Send + for<'de> serde::Deserialize<'de> + 'static,
        W: Worker<Args> + 'static,
    >(
        &mut self,
        worker: W,
    ) {
        self.workers
            .insert(W::class_name(), Arc::new(WorkerRef::wrap(Arc::new(worker))));
    }

    pub fn get_cancellation_token(&self) -> CancellationToken {
        self.cancellation_token.clone()
    }

    pub(crate) async fn register_periodic(&mut self, periodic_job: PeriodicJob) -> Result<()> {
        self.periodic_jobs.push(periodic_job.clone());

        periodic_job.insert().await?;

        Ok(())
    }

    pub async fn run(self) {
        let mut join_set: JoinSet<()> = JoinSet::new();
        for i in 0..self.num_workers {
            join_set.spawn({
                let mut processor = self.clone();
                let cancellation_token = self.cancellation_token.clone();

                async move {
                    loop {
                        if let Err(err) = processor.process_one().await {
                            error!("Error leaked out the bottom: {:?}", err);
                        }

                        if cancellation_token.is_cancelled() {
                            break;
                        }
                    }

                    debug!("Broke out of loop for worker {}", i);
                }
            });
        }

        join_set.spawn({
            let cancellation_token = self.cancellation_token.clone();
            async move {
                let sched = Scheduled::default();
                let sorted_sets = vec!["retry".to_string(), "schedule".to_string()];

                loop {
                    select! {
                        _ = tokio::time::sleep(std::time::Duration::from_secs(5)) => {}
                        _ = cancellation_token.cancelled() => {
                            break;
                        }
                    }

                    if let Err(err) = sched.enqueue_jobs(chrono::Utc::now(), &sorted_sets).await {
                        error!("Error in scheduled poller routine: {:?}", err);
                    }
                }

                debug!("Broke out of loop for retry and scheduled");
            }
        });

        join_set.spawn({
            let cancellation_token = self.cancellation_token.clone();
            async move {
                let sched = Scheduled::default();

                loop {
                    select! {
                        _ = tokio::time::sleep(std::time::Duration::from_secs(30)) => {}
                        _ = cancellation_token.cancelled() => {
                            break;
                        }
                    }

                    if let Err(err) = sched.enqueue_periodic_jobs(chrono::Utc::now()).await {
                        error!("Error in periodic job poller routine: {}", err);
                    }
                }

                debug!("Broke out of loop for periodic");
            }
        });

        while let Some(result) = join_set.join_next().await {
            if let Err(err) = result {
                error!("Processor had a spawned task return an error: {}", err);
            }
        }
    }
}
