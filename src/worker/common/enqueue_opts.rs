use crate::common::error::Result;
use crate::GID;
use serde::Serialize;
use serde_json::Value as JsonValue;

use super::{Job, UnitOfWork};

#[must_use]
pub fn opts() -> EnqueueOpts {
    EnqueueOpts {
        queue: "default".into(),
        retry: true,
        unique_for: None,
    }
}

pub struct EnqueueOpts {
    pub queue: String,
    pub retry: bool,
    pub unique_for: Option<std::time::Duration>,
}

impl EnqueueOpts {
    #[must_use]
    pub fn queue<S: Into<String>>(self, queue: S) -> Self {
        Self {
            queue: queue.into(),
            ..self
        }
    }

    #[must_use]
    pub fn retry(self, retry: bool) -> Self {
        Self { retry, ..self }
    }

    #[must_use]
    pub fn unique_for(self, unique_for: std::time::Duration) -> Self {
        Self {
            unique_for: Some(unique_for),
            ..self
        }
    }

    async fn create_job(&self, class: String, args: impl Serialize) -> Result<Job> {
        let args = serde_json::to_value(args)?;
        let args = if args.is_array() {
            args
        } else {
            JsonValue::Array(vec![args])
        };

        Ok(Job {
            queue: self.queue.clone(),
            class,
            jid: GID().await,
            created_at: chrono::Utc::now().timestamp() as f64,
            enqueued_at: None,
            retry: self.retry,
            args,
            error_message: None,
            failed_at: None,
            retry_count: None,
            retried_at: None,
            unique_for: self.unique_for,
        })
    }

    pub async fn perform_async(self, class: String, args: impl Serialize) -> Result<()> {
        let job = self.create_job(class, args).await?;
        UnitOfWork::from_job(job).enqueue().await?;
        Ok(())
    }

    pub async fn perform_in(
        &self,
        class: String,
        duration: std::time::Duration,
        args: impl Serialize,
    ) -> Result<()> {
        let job = self.create_job(class, args).await?;
        UnitOfWork::from_job(job).schedule(duration).await?;
        Ok(())
    }
}
