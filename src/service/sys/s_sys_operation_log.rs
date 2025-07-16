use crate::model::sys::model::msys_oper_log::{SysOperLogAdd, SysOperLogModel,SysOperLogSearch};
use crate::service::prelude::*;

pub async fn list(VQuery(arg): VQuery<PageParams>,
VQuery(search):VQuery<SysOperLogSearch>)->impl IntoResponse{
    let rlist= SysOperLogModel::list(arg,search).await;
    ApiResponse::from_result(rlist)
}
pub async fn edit()->impl IntoResponse{
    
}
pub async fn add() -> impl IntoResponse {}
pub async fn delete() -> impl IntoResponse {}


pub async fn add_oper_log(arg: SysOperLogAdd) {
    let _ = SysOperLogModel::add(arg).await;
}
