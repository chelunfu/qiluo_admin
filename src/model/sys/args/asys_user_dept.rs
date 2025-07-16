use crate::model::prelude::*;

#[derive(Debug, Deserialize, Serialize, FromQueryResult, Clone, Default)]
pub struct UserDeptResp {
    #[serde(with = "i64_to_string")]
    pub dept_id: i64,
    pub dept_name: String,
}

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct UserDeptAndUserResp {
    #[serde(with = "i64_to_string")]
    pub user_dept_id: i64,
    pub depts: Vec<UserDeptResp>,
}