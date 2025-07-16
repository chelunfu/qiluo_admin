use crate::model::sys::model::msys_role::{
    RoleAddReq, RoleEditReq, RoleReq, RoleSearch, SysRoleModel,
};
use crate::service::prelude::*;

pub async fn list(
    VQuery(arg): VQuery<PageParams>,
    VQuery(search): VQuery<RoleSearch>,
) -> impl IntoResponse {
    let rlist = SysRoleModel::list(arg, search).await;
    ApiResponse::from_result(rlist)
}
pub async fn tree() -> impl IntoResponse {
    let rlist = SysRoleModel::tree().await;
    ApiResponse::from_result(rlist)
}
pub async fn menu() -> impl IntoResponse {
    let r = SysRoleModel::menu().await;
    ApiResponse::from_result(r)
}

pub async fn get_role_menus(VQuery(arg): VQuery<RoleReq>) -> impl IntoResponse {
    let r = SysRoleModel::get_role_menus(arg.role_id).await;
    ApiResponse::from_result(r)
}

pub async fn edit(VJson(arg): VJson<RoleEditReq>) -> impl IntoResponse {
    let r = SysRoleModel::edit(arg).await;
    ApiResponse::from_result(r)
}

pub async fn add(VJson(arg): VJson<RoleAddReq>) -> impl IntoResponse {
    let r = SysRoleModel::add(arg).await;
    ApiResponse::from_result(r)
}

pub async fn delete(VQuery(arg): VQuery<RoleReq>) -> impl IntoResponse {
    let r = SysRoleModel::delete(arg).await;
    ApiResponse::from_result(r)
}

pub async fn get_role_depts(VQuery(arg): VQuery<RoleReq>) -> impl IntoResponse {
    let r = SysRoleModel::get_role_depts(arg.role_id).await;
    ApiResponse::from_result(r)
}
