use crate::model::prelude::*;

#[derive(Debug, Deserialize, Serialize, Clone, Validate)]
pub struct RoleSearch {
    pub role_name: Option<String>,
    pub role_key: Option<String>,
}

#[derive(Deserialize, Validate, Serialize)]
pub struct RoleAddReq {
    pub role_name: String,
    pub role_key: String,
    pub order: i32,
    pub status: String,
    pub data_scope: String,
}

//菜单
#[derive(Deserialize, Validate, Serialize,Debug)]
pub struct RoleEditReq {
    #[serde(with = "i64_to_string")]
    pub role_id: i64,
    pub role_name: String,
    pub role_key: String,
    pub order: i32,
    pub data_scope: String,
    pub status: String,
    pub remark: Option<String>,
    #[serde(with = "veci64_to_vecstring")]
    pub menu: Vec<i64>,

    #[serde(with = "option_veci64_to_vecstring")]
    pub data_depts: Option<Vec<i64>>,
}

#[derive(Deserialize, Validate, Serialize)]
pub struct RoleReq {
    #[serde(with = "i64_to_string")]
    pub role_id: i64,
}

#[derive(Debug, Deserialize, Serialize, FromQueryResult, Clone, Default)]
pub struct RoleMenuResp {
    #[serde(with = "i64_to_string")]
    pub role_id: i64,
    pub role_name: String,
    pub role_key: String,
    pub order: i32,
    pub data_scope: String,
    pub status: String,
    #[sea_orm(skip)]
    pub mens: Vec<RoleMResp>,
}
#[derive(Debug, Deserialize, Serialize, FromQueryResult, Clone, Default)]
pub struct RoleMResp {
    #[serde(with = "i64_to_string")]
    pub id: i64,
    pub title: String,
}
#[derive(Debug, Deserialize, Serialize, FromQueryResult, Clone, Default)]
pub struct RoleResp {
    #[serde(with = "i64_to_string")]
    pub role_id: i64,
    pub role_name: String,
    pub role_key: String,
    pub order: i32,
    pub data_scope: String,
    pub status: String,
    pub remark: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, FromQueryResult, Clone, Default)]
pub struct RoleDeptResp {
    #[serde(with = "i64_to_string")]
    pub dept_id: i64
}