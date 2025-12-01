pub use axum::response::{Html, IntoResponse};
pub use chrono::{Duration, Local};
pub use sea_orm::{
    entity::prelude::DateTime, prelude::Expr, ActiveModelBehavior, ActiveModelTrait, ActiveValue,
    ColumnTrait, Condition, ConnectionTrait, DatabaseConnection, DatabaseTransaction, DbBackend,
    DbErr, EntityTrait, FromQueryResult, IntoActiveModel, JoinType, ModelTrait, Order,
    PaginatorTrait, QueryFilter, QueryOrder, QuerySelect, QueryTrait, Set, TransactionTrait,
};
pub use serde::{Deserialize, Serialize};
pub use serde_json::json;
pub use tracing::{error, info};

pub use crate::{
    common::{
        self,
        error::{Error, Result},
        ser::*,
        util,
        validatedjson::VJson,
        validatedquery::VQuery,
        ApiResponse,
    },
    config::APPCOFIG,
    db::{with_read, DB, DB_AUTO, DB_BY_INDEX, DB_BY_NAME, DB_READ, DB_WRITE, GID},
    midle_ware::jwt::{self, AuthPayload, Claims, UserInfo},
};
pub use validator::Validate;

#[derive(Debug, Serialize, Clone, Deserialize)]
/// 查 数据返回
pub struct ListData<T> {
    pub list: Vec<T>,
    pub total: u64,
    pub total_pages: u64,
    pub page_num: u64,
}

///页面
#[derive(Deserialize, Clone, Debug, Serialize, Default, Validate)]
pub struct PageParams {
    pub page_num: Option<u64>,
    pub page_size: Option<u64>,
}
pub use crate::cache::CacheManager;
pub use sea_orm::prelude::DateTimeUtc;
