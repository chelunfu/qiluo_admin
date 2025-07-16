use crate::common::error::Result;
use crate::model::sys::model::{
    msys_job::SysJobModel,
    msys_job_log::{JobLogAdd, SysJobLogModel},
};
use crate::service::sys::{s_sys_api_permission, s_sys_white_jwt};
use crate::worker::common::{Worker, WorkerOpts};
use crate::worker::AppWorker;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use tracing::info;
#[derive(Deserialize, Serialize, Clone, Default, Debug)]
pub struct InvokeFunctionMsg {
    pub job_id: Option<i64>,
    pub callfun: String,
    pub parmets: String,
}

#[derive(Clone)]
pub struct InvokeFunctionWorker {}

impl AppWorker<InvokeFunctionMsg> for InvokeFunctionWorker {
    fn new() -> Self {
        Self {}
    }
}
#[async_trait]
impl Worker<InvokeFunctionMsg> for InvokeFunctionWorker {
    fn opts() -> WorkerOpts<InvokeFunctionMsg, Self> {
        WorkerOpts::new().queue("default")
    }

    async fn perform(&self, arg: InvokeFunctionMsg) -> Result<()> {
        info!("InvokeFunctionWorker perform: {:?}", arg);
        let message = match arg.callfun.as_str() {
            "updateapi" => s_sys_api_permission::update_all_api(arg.parmets).await,
            "clearuserinfo" => s_sys_white_jwt::clear_user_info().await,
            _ => Ok("未找到对应的方法".to_string()),
        };
        if let Some(job_id) = arg.job_id {
            let runcount = SysJobModel::updata_run_count(job_id).await.unwrap_or(0);
            let mut jog_add = JobLogAdd {
                job_id,
                run_count: runcount,
                ..Default::default()
            };
            match message {
                Ok(r) => {
                    let txt = r;
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
        }

        Ok(())
    }
}
