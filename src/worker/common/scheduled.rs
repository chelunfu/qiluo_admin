use super::Result;
use super::{periodic::PeriodicJob, UnitOfWork};
use crate::cache::CacheManager;
use tracing::info;
#[derive(Default)]
pub struct Scheduled {}

impl Scheduled {
    pub async fn enqueue_jobs(
        &self,
        now: chrono::DateTime<chrono::Utc>,
        sorted_sets: &Vec<String>,
    ) -> Result<usize> {
        let mut n = 0;
        let cache = CacheManager::instance().await;
        for sorted_set in sorted_sets {
            let jobs: Vec<String> = cache
                .zrangebyscore_limit(
                    sorted_set,
                    f64::NEG_INFINITY,
                    now.timestamp() as f64,
                    0,
                    100,
                )
                .await?;

            n += jobs.len();

            for job in jobs {
                if cache.zrem(sorted_set, job.clone()).await? {
                    let work = UnitOfWork::from_job_string(job)?;

                    work.enqueue_direct().await?;
                }
            }
        }

        Ok(n)
    }

    ///定时作业
    pub async fn enqueue_periodic_jobs(&self, now: chrono::DateTime<chrono::Utc>) -> Result<usize> {
        let cache = CacheManager::instance().await;
        let periodic_jobs: Vec<String> = cache
            .zrangebyscore_limit(
                "periodic",
                f64::NEG_INFINITY,
                now.timestamp() as f64,
                0,
                100,
            )
            .await?;
        for periodic_job in &periodic_jobs {
            let pj = PeriodicJob::from_json_string(periodic_job)?;
            if pj.update(periodic_job).await? > 0 {
                let job = pj.into_job().await;
                let work = UnitOfWork::from_job(job);

                work.enqueue_direct().await?;
            } else {
                info!("update periodic job failed");
            }
        }

        Ok(periodic_jobs.len())
    }
}
