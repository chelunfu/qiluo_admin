use crate::model::sys::model::msys_user_role::SysUserRoleModel;
use crate::service::prelude::*;

pub async fn user_role_name_list(userinfo: UserInfo) -> impl IntoResponse {
    let r = SysUserRoleModel::user_role_name_list(userinfo.uid, userinfo.rid).await;
    ApiResponse::from_result(r)
}
