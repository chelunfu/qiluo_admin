use crate::worker::common::{Worker, WorkerOpts};
use crate::worker::AppWorker;
use crate::common::error::Result;
use async_trait::async_trait;  

use crate::model::sys::model::msys_login_info::LoginInfoMsg;
use crate::service::sys::s_sys_login_info::update_login_info;
 
#[derive(Clone)]
pub struct LoginInfoWorker {}

 
impl AppWorker<LoginInfoMsg> for LoginInfoWorker {
    fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl Worker<LoginInfoMsg> for LoginInfoWorker {
    fn opts() -> WorkerOpts<LoginInfoMsg, Self> { 
        WorkerOpts::new().queue("default")
    }

    async fn perform(&self, arg: LoginInfoMsg) ->Result<()> { 
        update_login_info(arg).await;
        Ok(())
    }
}
