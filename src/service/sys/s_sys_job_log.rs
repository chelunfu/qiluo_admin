use crate::model::sys::model::msys_job_log::{JobLogSearch, SysJobLogModel};
use crate::service::prelude::*;

pub async fn list(
    VQuery(arg): VQuery<PageParams>,
    VQuery(search): VQuery<JobLogSearch>,
) -> impl IntoResponse {
    let rlist = SysJobLogModel::list(arg, search).await;
    ApiResponse::from_result(rlist)
}
pub async fn edit() -> impl IntoResponse {}
pub async fn add() -> impl IntoResponse {}
pub async fn delete() -> impl IntoResponse {}
