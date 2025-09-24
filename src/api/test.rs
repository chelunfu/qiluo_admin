use super::web_path::{WebPath, WebPathType};
use crate::service::test::*;

use axum::{
    routing::{delete, get, post, put},
    Router,
};
pub fn router_test() -> WebPath {
    WebPath::new().nest("/test", WebPath::new().nest("/test_api", test_test_api()))
}

pub fn white_test() -> Router {
    Router::new()
}

fn test_test_api() -> WebPath {
    WebPath::new()
        .route(
            "/list",
            WebPathType::Get,
            Some("获取列表"),
            get(s_test_api::list),
        )
        .route(
            "/edit",
            WebPathType::Put,
            Some("编辑TestApi"),
            put(s_test_api::edit),
        )
        .route(
            "/add",
            WebPathType::Post,
            Some("添加TestApi"),
            post(s_test_api::add),
        )
        .route(
            "/del",
            WebPathType::Delete,
            Some("删除TestApi"),
            delete(s_test_api::delete),
        )
}
