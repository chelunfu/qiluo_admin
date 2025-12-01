use crate::model::test::model::mtest_api::{
    TestApiAdd, TestApiDel, TestApiEdit, TestApiModel, TestApiSearch,
};
use crate::service::prelude::*;
pub async fn list(
    VQuery(arg): VQuery<PageParams>,
    VQuery(search): VQuery<TestApiSearch>,
) -> impl IntoResponse {
    let rlist = TestApiModel::list(arg, search).await;
    ApiResponse::from_result(rlist)
}
pub async fn edit(VJson(arg): VJson<TestApiEdit>) -> impl IntoResponse {
    let r = TestApiModel::edit(arg).await;
    ApiResponse::from_result(r)
}
pub async fn add(VJson(arg): VJson<TestApiAdd>) -> impl IntoResponse {
    let r = TestApiModel::add(arg).await;
    ApiResponse::from_result(r)
}
pub async fn delete(VQuery(arg): VQuery<TestApiDel>) -> impl IntoResponse {
    let r = TestApiModel::del(arg).await;
    ApiResponse::from_result(r)
}


pub async fn db_index_test(VJson(arg): VJson<TestApiEdit>) -> impl IntoResponse {
    let r = TestApiModel::db_index_test(arg).await;
    ApiResponse::from_result(r)
}

pub async fn db_name_test(VJson(arg): VJson<TestApiEdit>) -> impl IntoResponse {
    let r = TestApiModel::db_name_test(arg).await;
    ApiResponse::from_result(r)
}

pub async fn db_read_write_test(VJson(arg): VJson<TestApiEdit>) -> impl IntoResponse {
    let r = TestApiModel::db_read_write_test(arg).await;
    ApiResponse::from_result(r)
}

pub async fn db_auto_test(VJson(arg): VJson<TestApiEdit>) -> impl IntoResponse {
    let r = TestApiModel::db_auto_test(arg).await;
    ApiResponse::from_result(r)
}