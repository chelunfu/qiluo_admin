use crate::model::prelude::*;

#[derive(Debug, Serialize, Deserialize, Clone, Validate)]
pub struct CacheSearch {
    pub key: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CacheItem {
    pub key: String,
    pub value: String,
    pub ttl: Option<i64>,
}
