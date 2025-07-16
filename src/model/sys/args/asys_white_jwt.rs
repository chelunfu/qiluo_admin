use crate::model::prelude::*;

#[derive(Deserialize, Serialize)]
pub struct WhiteJwtAdd {
    pub uid: i64,
    pub token_id: i64,
    pub token_expr: i64,
    pub info_id: i64,
}

#[derive(Deserialize, Serialize, Validate)]
pub struct WhiteJwtSearch {
    pub user_name: Option<String>,
}

#[derive(Deserialize, Serialize, FromQueryResult, Clone)]
pub struct WhiteJwtRes {
    #[serde(with = "i64_to_string")]
    pub jid: i64,
    #[serde(with = "i64_to_string")]
    pub token_id: i64,
    pub user_name: Option<String>,
    pub device_type: Option<String>,
    pub ipaddr: Option<String>,
    pub login_location: Option<String>,
    pub net_work: Option<String>,
    pub browser: Option<String>,
    pub os: Option<String>,
    pub status: Option<String>,
    pub msg: Option<String>,
    pub login_time: Option<DateTime>,
    #[sea_orm(skip)]
    pub useronline:bool,
}

#[derive(Deserialize, Serialize)]
pub struct WhiteJwtEdit {
    pub token_id: i64,
    pub token_expr: i64
}

#[derive(Deserialize, Serialize, Validate)]
pub struct WhiteJwtDel {
    #[serde(with = "i64_to_string")]
    pub jid: i64,
}
