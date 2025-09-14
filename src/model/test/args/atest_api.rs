use crate::model::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize, FromQueryResult, Validate)]
pub struct TestApiResp {
    #[serde(with = "i64_to_string")]
    pub id: i64,
    pub name: String,
    pub age: i32,
    pub email: String,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize,  Validate)]
pub struct TestApiAdd { 
    pub name: String,
    pub age: i32,
    pub email: String
}

#[derive(Debug, Clone, Serialize, Deserialize,  Validate)]
pub struct TestApiEdit {
    #[serde(with = "i64_to_string")]
    pub id: i64,
    pub name: String,
    pub age: i32,
    pub email: String
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct TestApiDel {
    #[serde(with = "i64_to_string")]
    pub id: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct TestApiSearch {
    pub name: Option<String>, 
    pub email: Option<String>,
}