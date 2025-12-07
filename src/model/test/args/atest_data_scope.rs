use crate::model::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, FromQueryResult, Validate)]
pub struct TestDataScopeResp {
    #[serde(with = "i64_to_string")]
    pub id: i64,
    pub title: String,
    pub content: Option<String>,
    #[serde(with = "i64_to_string")]
    pub dept_id: i64,
    #[serde(with = "i64_to_string")]
    pub owner_id: i64,
    pub status: String,
    pub created_at: DateTime,
    pub updated_at: DateTime,
    pub deleted_at: Option<DateTime>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct TestDataScopeAdd {
    pub title: String,
    pub content: Option<String>,
    pub status: String,
    #[serde(with = "option_string_or_i64",default)]
    pub dept_id: Option<i64>,
    #[serde(with = "option_string_or_i64",default)]
    pub owner_id: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct TestDataScopeEdit {
    #[serde(with = "i64_to_string")]
    pub id: i64,
    pub title: String,
    pub content: Option<String>,
    pub status: String,
    #[serde(with = "option_string_or_i64",default)]
    pub dept_id: Option<i64>,
    #[serde(with = "option_string_or_i64",default)]
    pub owner_id: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct TestDataScopeDel {
    #[serde(with = "i64_to_string")]
    pub id: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct TestDataScopeSearch {
    pub title: Option<String>,
    pub content: Option<String>,
}
