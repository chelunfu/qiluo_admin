use crate::model::prelude::*;

#[derive(Debug, Deserialize, Serialize, FromQueryResult, Clone, Default)]
pub struct UserRoleResp {
    #[serde(with = "i64_to_string")]
    pub role_id: i64,
    pub role_name: String,
}
#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct UserRoleAndUserResp {
    #[serde(with = "i64_to_string")]
    pub user_role_id: i64,
    pub roles: Vec<UserRoleResp>,
}
