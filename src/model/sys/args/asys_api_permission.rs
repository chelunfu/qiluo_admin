use crate::model::prelude::*;

#[derive(Debug, Deserialize, Serialize, Clone, Validate)]
pub struct ApiPermissionSearch {
    pub api: Option<String>,
    pub method: Option<String>,
    pub apiname: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone, Validate)]
pub struct ApiPermissionAdd {
    pub api: String,
    pub method: String,
    pub apiname: String,
    pub sort: i32,
}

#[derive(Debug, Deserialize, Serialize, Clone, Validate)]
pub struct ApiPermissionEdit {
    #[serde(with = "i64_to_string")]
    pub id: i64, 
    pub logcache: String,
    pub sort: i32,
    pub remark: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone, Validate)]
pub struct ApiPermissionDel {
    #[serde(with = "i64_to_string")]
    pub id: i64,
}

#[derive(Debug, Deserialize, Serialize, Clone, FromQueryResult)]
pub struct ApiPermissionRes {
    #[serde(with = "i64_to_string")]
    pub id: i64,
    pub api: String,
    pub method: String,
    pub apiname: String,
    pub logcache: String,
    pub sort: i32,
    pub remark: Option<String>,
}
