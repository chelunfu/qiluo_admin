use crate::common::error::Result;
use crate::model::sys::model::{
    msys_job::SysJobModel,
    msys_job_log::{JobLogAdd, SysJobLogModel},
};
use crate::worker::common::{Worker, WorkerOpts};
use crate::worker::AppWorker;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use tracing::info;
#[derive(Deserialize, Serialize, Clone, Default)]
pub struct RequestUrlMsg {
    pub url: String,
    pub job_id: i64,
    pub task_type: String,
    pub job_name: String,
    pub job_group: String,
}

#[derive(Clone)]
pub struct RequestUrlWorker {}

impl AppWorker<RequestUrlMsg> for RequestUrlWorker {
    fn new() -> Self {
        Self {}
    }
}
#[async_trait]
impl Worker<RequestUrlMsg> for RequestUrlWorker {
    fn opts() -> WorkerOpts<RequestUrlMsg, Self> {
        WorkerOpts::new().queue("default")
    }

    async fn perform(&self, arg: RequestUrlMsg) -> Result<()> {
        info!("request url: {}", arg.url);
        let url = arg.url;
        let resp = reqwest::get(url.as_str()).await;
        let runcount = SysJobModel::updata_run_count(arg.job_id).await.unwrap_or(0);
        let mut jog_add = JobLogAdd {
            job_id: arg.job_id,
            run_count: runcount,
            ..Default::default()
        };
        match resp {
            Ok(r) => {
                let txt = r.text().await.unwrap_or_default();
                jog_add.job_message = Some(if txt.len() > 2048 {
                    let truncated = &txt[..2000];
                    format!("{truncated} ...数据太长，不记录完整内容。")
                } else {
                    txt
                });
                jog_add.status.clone_from(&"Sucess".to_owned());
            }
            Err(e) => {
                let txt = e.to_string();
                jog_add.job_message = Some(if txt.len() > 2048 {
                    let truncated = &txt[..2000];
                    format!("{truncated} ...数据太长，不记录完整内容。")
                } else {
                    txt
                });
                jog_add.status.clone_from(&"Failed".to_owned());
            }
        };
        let _ = SysJobLogModel::add(jog_add).await;
        Ok(())
    }
}
