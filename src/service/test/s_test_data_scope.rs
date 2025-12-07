use crate::model::test::model::mtest_data_scope::*;
use crate::service::prelude::*;
pub async fn list(
    VQuery(arg): VQuery<PageParams>,
    VQuery(search): VQuery<TestDataScopeSearch>,
    userinfo: UserInfo,
) -> impl IntoResponse {
    let rlist = TestDataScopeModel::list(arg, search, userinfo).await;
    ApiResponse::from_result(rlist)
}
pub async fn edit(userinfo: UserInfo, VJson(arg): VJson<TestDataScopeEdit>) -> impl IntoResponse {
    let r = TestDataScopeModel::edit(arg, userinfo).await;
    ApiResponse::from_result(r)
}
pub async fn add(userinfo: UserInfo, VJson(arg): VJson<TestDataScopeAdd>) -> impl IntoResponse {
    let r = TestDataScopeModel::add(arg, userinfo).await;
    ApiResponse::from_result(r)
}
pub async fn delete(
    userinfo: UserInfo,
    VQuery(arg): VQuery<TestDataScopeDel>,
) -> impl IntoResponse {
    let r = TestDataScopeModel::del(arg, userinfo).await;
    ApiResponse::from_result(r)
}
