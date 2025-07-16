use crate::model::sys::model::msys_role_api::{
    RoleApiCheckInfo, RoleApiSearch, RoleApiTransferListIdReq, RoleApiTransferReq, SysRoleApiModel,
};
use crate::service::prelude::*;

pub async fn list(
    VQuery(arg): VQuery<PageParams>,
    VQuery(search): VQuery<RoleApiSearch>,
) -> impl IntoResponse {
    let rlist = SysRoleApiModel::list(arg, search).await;
    ApiResponse::from_result(rlist)
}
pub async fn edit() -> impl IntoResponse {}
pub async fn add() -> impl IntoResponse {}

pub async fn role_permission_list(uinfo: UserInfo) -> impl IntoResponse {
    let rlist = SysRoleApiModel::role_permission_list(uinfo.rid).await;
    ApiResponse::from_result(rlist)
}

pub async fn role_api_transfer_list(VQuery(arg): VQuery<RoleApiTransferReq>) -> impl IntoResponse {
    let rlist = SysRoleApiModel::role_api_transfer_list(arg).await;
    ApiResponse::from_result(rlist)
}

pub async fn add_many_role_api_transfer(
    VJson(arg): VJson<RoleApiTransferListIdReq>,
) -> impl IntoResponse {
    let rlist = SysRoleApiModel::add_many_role_api_transfer(arg).await;
    ApiResponse::from_result(rlist)
}
pub async fn delete() -> impl IntoResponse {}

pub async fn check_api_permission(rid: i64, api: &str, method: &str) -> bool {
    if APPCOFIG.system.super_role.contains(&rid) {
        return true;
    }
    let arg = RoleApiCheckInfo {
        role_id: rid,
        api: api.to_owned(),
        method: method.to_owned(),
    };
    SysRoleApiModel::check_api(arg).await
}
