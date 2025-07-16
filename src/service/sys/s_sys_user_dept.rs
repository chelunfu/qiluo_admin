use crate::model::sys::model::msys_user_dept::SysUserDeptModel;
use crate::service::prelude::*;

pub async fn user_dept_name_list(userinfo: UserInfo) -> impl IntoResponse {
    let r = SysUserDeptModel::user_dept_name_list(userinfo.uid,userinfo.did).await;
    ApiResponse::from_result(r)
}