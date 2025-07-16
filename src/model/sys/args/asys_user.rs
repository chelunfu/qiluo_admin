use crate::model::prelude::*;

#[derive(Debug, Deserialize, Serialize, Clone, Validate)]
pub struct UserSearch {
    #[serde(with = "i64_to_string")]
    pub dept_id: i64,
    pub user_name: Option<String>,
    pub phonenumber: Option<String>,
    pub email: Option<String>,
    pub status: Option<String>,
}
#[derive(Debug, Deserialize, Serialize, Clone, Validate)]
pub struct LoginParams {
    pub username: String,
    pub password: String,
    #[serde(with = "i64_to_string")]
    pub captchaid: i64,
    pub client_id: String,
    pub captcha: String,
    pub email: Option<String>,
    pub phone: Option<String>,
}

#[derive(Serialize, Clone, Deserialize, Validate, Debug)]
pub struct ResetPasswordParams {
    pub old_password: String,
    pub new_password: String,
}

#[derive(Serialize, Clone, Deserialize, Validate, Debug)]
pub struct ChangePasswordParams {
    #[serde(with = "i64_to_string")]
    pub uid: i64,
    pub new_password: String,
}
#[derive(Serialize, Clone, Deserialize, Debug, FromQueryResult)]
pub struct SysUserRes {
    #[serde(with = "i64_to_string")]
    pub id: i64,
    #[serde(with = "i64_to_string")]
    pub dept_id: i64,
    #[serde(with = "i64_to_string")]
    pub role_id: i64,
    pub user_name: String,
    pub nick_name: String,
    pub user_type: Option<String>,
    pub email: Option<String>,
    pub phonenumber: Option<String>,
    pub sex: Option<String>,
    pub avatar: Option<String>,
    pub status: Option<String>,
    pub remark: Option<String>,
    pub create_time: Option<DateTime>,
    pub role_name: String,
}
#[derive(Serialize, Clone, Deserialize, Debug, FromQueryResult)]
pub struct SysUserAndRole {
    #[serde(with = "i64_to_string")]
    pub dept_id: i64,
    pub user_name: String,
    pub nick_name: String,
    pub user_type: Option<String>,
    pub email: Option<String>,
    pub phonenumber: Option<String>,
    pub sex: Option<String>,
    pub avatar: Option<String>,
    pub status: Option<String>,
    pub remark: Option<String>,
    #[serde(with = "i64_to_string")]
    pub role_id: i64,
    pub role_name: String,
    pub role_key: String,
    pub order: i32,
    pub data_scope: Option<String>,
    pub menu_check_strictly: Option<String>,
    pub dept_check_strictly: Option<String>,
}

#[derive(Serialize, Clone, Deserialize, Debug, Validate)]
pub struct SysUserEdit {
    #[serde(with = "i64_to_string")]
    pub id: i64,
    #[serde(with = "veci64_to_vecstring")]
    pub dept_ids: Vec<i64>,
    #[serde(with = "i64_to_string")]
    pub dept_id: i64,
    #[serde(with = "veci64_to_vecstring")]
    pub role_ids: Vec<i64>,
    #[serde(with = "i64_to_string")]
    pub role_id: i64,
    pub user_name: String,
    pub nick_name: String,
    pub user_type: Option<String>,
    pub email: Option<String>,
    pub phonenumber: Option<String>,
    pub sex: Option<String>,
    pub avatar: Option<String>,
    pub status: Option<String>,
    pub remark: Option<String>,
}

#[derive(Serialize, Clone, Deserialize, Debug, Validate)]
pub struct SysUserAdd {
    #[serde(with = "veci64_to_vecstring")]
    pub dept_ids: Vec<i64>,
    #[serde(with = "i64_to_string")]
    pub dept_id: i64,
    #[serde(with = "veci64_to_vecstring")]
    pub role_ids: Vec<i64>,
    #[serde(with = "i64_to_string")]
    pub role_id: i64,
    pub user_name: String,
    pub nick_name: String,
    pub password: String,
    pub user_type: Option<String>,
    pub email: Option<String>,
    pub phonenumber: Option<String>,
    pub sex: Option<String>,
    pub avatar: Option<String>,
    pub status: Option<String>,
    pub remark: Option<String>,
}

#[derive(Serialize, Clone, Deserialize)]

pub struct UserInfoDetail {
    pub username: String,
    pub nickname: String,
    pub email: Option<String>,
    pub phone: Option<String>,
    #[serde(with = "i64_to_string")]
    pub did: i64,
    #[serde(with = "i64_to_string")]
    pub rid: i64,
}

#[derive(Serialize, Clone, Deserialize)]
pub struct UserInfoRes {
    pub username: String,
    pub nickname: String,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub avatar: Option<String>,
    #[serde(with = "i64_to_string")]
    pub did: i64,
    #[serde(with = "i64_to_string")]
    pub rid: i64,
}

#[derive(Serialize, Clone, Deserialize)]
pub struct DeptsAndRoles {
    #[serde(with = "veci64_to_vecstring")]
    pub depts: Vec<i64>,
    #[serde(with = "veci64_to_vecstring")]
    pub roles: Vec<i64>,
}

#[derive(Serialize, Clone, Deserialize, Debug, Validate)]
pub struct UserId {
    #[serde(with = "i64_to_string")]
    pub uid: i64,
}

#[derive(Serialize, Clone, Deserialize, Debug, Validate)]
pub struct UserIds {
    #[serde(with = "veci64_to_vecstring")]
    pub uids: Vec<i64>,
}

#[derive(Serialize, Clone, Deserialize, Debug, Validate)]
pub struct UserAvatarEdit {
    pub avatar: String,
}

#[derive(Serialize, Clone, Deserialize, Validate)]
pub struct DeptAndRole {
    #[serde(with = "i64_to_string")]
    pub rid: i64,
    #[serde(with = "i64_to_string")]
    pub did: i64,
}
