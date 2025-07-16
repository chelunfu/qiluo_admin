use crate::config::APPCOFIG;
use crate::worker::AppWorker;
use tracing::trace;

use super::common::Processor;
use super::{InvokeFunctionWorker, JobWorker, LoginInfoWorker, MailerWorker, RequestUrlWorker};

pub const DEFAULT_QUEUES: &[&str] = &["default", "mailer", "logininfo"];

async fn init_process() -> Processor {
    let config = APPCOFIG.clone();
    let queues = get_queues(&config.workers.queues);
    let num_workers = config.workers.num_workers;
    trace!(
        queues = ?queues,
        "registering queues (merged config and default)"
    );
    Processor::new(
        DEFAULT_QUEUES
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>(),
        num_workers,
    )
}

pub fn get_queues(config_queues: &Option<Vec<String>>) -> Vec<String> {
    let mut queues = DEFAULT_QUEUES
        .iter()
        .map(ToString::to_string)
        .collect::<Vec<_>>();

    if let Some(config_queues) = config_queues {
        for q in config_queues {
            if !queues.iter().any(|aq| q == aq) {
                queues.push(q.to_string());
            }
        }
    }

    queues
}

pub async fn processor_job() -> Processor {
    let mut p = init_process().await;
    p.register(MailerWorker::new());
    p.register(LoginInfoWorker::new());
    p.register(JobWorker::new());
    p.register(InvokeFunctionWorker::new());
    p.register(RequestUrlWorker::new());

    trace!("done registering workers and queues");
    p
}
