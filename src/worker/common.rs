use crate::common::error::Result;
use serde::Serialize;

pub mod enqueue_opts;
pub mod job;
pub mod periodic;
mod processor;
mod scheduled;
pub mod unit_of_work;
pub mod worker;
pub mod worker_opts;

pub use enqueue_opts::{opts, EnqueueOpts};
pub use job::Job;
pub use processor::{Processor, WorkFetcher};
pub use scheduled::Scheduled;
pub use unit_of_work::UnitOfWork;
pub use worker::{Worker, WorkerRef};
pub use worker_opts::WorkerOpts;

pub async fn perform_async(class: String, queue: String, args: impl Serialize) -> Result<()> {
    opts().queue(queue).perform_async(class, args).await
}

pub async fn perform_in(
    duration: std::time::Duration,
    class: String,
    queue: String,
    args: impl Serialize,
) -> Result<()> {
    opts().queue(queue).perform_in(class, duration, args).await
}
