use crate::cache::CacheManager;
use crate::common::error::Result;
use rand::Rng;
use sha2::{Digest, Sha256};

use super::Job;

#[derive(Debug)]
pub struct UnitOfWork {
    pub queue: String,
    pub job: Job,
}

impl UnitOfWork {
    #[must_use]
    pub fn from_job(job: Job) -> Self {
        Self {
            queue: format!("queue:{}", &job.queue),
            job,
        }
    }

    pub fn from_job_string(job_str: String) -> Result<Self> {
        let job: Job = serde_json::from_str(&job_str)?;
        Ok(Self::from_job(job))
    }

    pub async fn enqueue(&self) -> Result<()> {
        self.enqueue_direct().await
    }

    pub async fn enqueue_direct(&self) -> Result<()> {
        let mut job = self.job.clone();
        job.enqueued_at = Some(chrono::Utc::now().timestamp() as f64);
        let cache = CacheManager::instance().await;
        if let Some(ref duration) = job.unique_for {
            let args_as_json_string: String = serde_json::to_string(&job.args)?;
            let args_hash = format!("{:x}", Sha256::digest(&args_as_json_string));
            let redis_key = format!(
                "enqueue:unique:{}:{}:{}",
                &job.queue, &job.class, &args_hash
            );
            if cache
                .set_nx_ex(&redis_key, "", duration.as_secs() as usize)
                .await?
            {
                return Ok(());
            }
        }

        cache.sadd("queues", &[job.queue.as_str()]).await?;

        cache
            .lpush(&self.queue, serde_json::to_string(&job)?)
            .await?;
        Ok(())
    }

    pub async fn reenqueue(&mut self) -> Result<()> {
        if let Some(retry_count) = self.job.retry_count {
            let cache = CacheManager::instance().await;
            cache
                .zadd(
                    "retry",
                    serde_json::to_string(&self.job)?,
                    Self::retry_job_at(retry_count).timestamp() as f64,
                )
                .await?;
        }

        Ok(())
    }

    fn retry_job_at(count: usize) -> chrono::DateTime<chrono::Utc> {
        let seconds_to_delay =
            count.pow(4) + 15 + (rand::rng().random_range(0..30) * (count + 1));

        chrono::Utc::now() + chrono::Duration::seconds(seconds_to_delay as i64)
    }

    pub async fn schedule(&mut self, duration: std::time::Duration) -> Result<()> {
        let enqueue_at = chrono::Utc::now() + chrono::Duration::from_std(duration).unwrap();
        let cache = CacheManager::instance().await;
        let _ = cache
            .zadd(
                "schedule",
                serde_json::to_string(&self.job)?,
                enqueue_at.timestamp() as f64,
            )
            .await;
        Ok(())
    }
}
