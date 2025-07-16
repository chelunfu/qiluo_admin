use crate::common::error::Result;
use crate::service::sys::s_sys_job;
use crate::worker::common::{Worker, WorkerOpts};
use crate::worker::AppWorker;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
#[derive(Deserialize, Serialize, Clone, Default)]
pub struct JobMsg {
    pub job_id: i64,
}

#[derive(Clone)]
pub struct JobWorker {}

impl AppWorker<JobMsg> for JobWorker {
    fn new() -> Self {
        Self {}
    }
}
#[async_trait]
impl Worker<JobMsg> for JobWorker {
    fn opts() -> WorkerOpts<JobMsg, Self> {
        WorkerOpts::new().queue("default")
    }

    async fn perform(&self, _: JobMsg) -> Result<()> {
        s_sys_job::update_job().await;

        Ok(())
    }
}
