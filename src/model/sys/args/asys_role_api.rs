use crate::model::prelude::*;

#[derive(Debug, Deserialize, Serialize, Clone,FromQueryResult)]
pub struct RoleApiInfo {
    #[serde(with = "i64_to_string")]
    pub role_id: i64,
    pub api: String,
    pub method: String,
    pub apiname: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct RoleApiCheckInfo {
    #[serde(with = "i64_to_string")]
    pub role_id: i64,
    pub api: String,
    pub method: String
}
#[derive(Debug, Deserialize, Serialize, Clone, Validate)]
pub struct RoleApiAdd {
    #[serde(with = "i64_to_string")]
    pub role_id: i64,
    pub api: String,
    pub method: String,
    pub apiname: String,
}

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct RoleApiSearch {
    #[serde(with = "i64_to_string")]
    pub role_id: i64,
    pub api: Option<String>,
    pub apiname: Option<String>,
}


 

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct RoleApiPermissions { 
    pub permissions: Vec<String>,
}
//返回权限

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct RoleApiTransferReq {
    #[serde(with = "i64_to_string")]
    pub role_id: i64
}


#[derive(Debug, Deserialize, Serialize, Clone,FromQueryResult)]
pub struct RoleApiTransferInfo {
    #[serde(with = "i64_to_string")]
    pub role_id: i64,
    #[serde(with = "i64_to_string")]
    pub api_id: i64,
    pub api: String,
    pub method: String,
}

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct RoleApiTransferListIdReq {
    #[serde(with = "i64_to_string")]
    pub role_id: i64,
    #[serde(with = "veci64_to_vecstring")]
    pub api_ids:Vec< i64>
}