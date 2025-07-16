use crate::model::prelude::*;


#[derive(Debug, Deserialize, Serialize, Clone, Validate)]
pub struct LoginInfoSearch {
    pub user_name: Option<String>
}

#[derive(Deserialize, Serialize, Clone,FromQueryResult)]
pub struct LoginInfoRes {
    pub user_name: String, 
    pub device_type: Option<String>,
    pub ipaddr: Option<String>,
    pub login_location: Option<String>,
    pub browser: Option<String>,
    pub os: Option<String>,
    pub status: Option<String>,
    pub msg: Option<String>,
    pub login_time: Option<DateTime>,
}

#[derive(Deserialize, Serialize, Clone,Default)]
pub struct LoginInfoMsg { 
    pub info_id:i64,
    pub ipaddr: String,
}
#[derive(Deserialize, Serialize, Clone,Default)]
pub struct LoginInfoAdd {
    pub user_name: String,
    pub uid:i64, 
    pub device_type: Option<String>,
    pub ipaddr: Option<String>,
    pub login_location: Option<String>,
    pub browser: Option<String>,
    pub os: Option<String>,
    pub status: Option<String>,
    pub msg: Option<String>,
    pub login_time: Option<DateTime>,
    pub net_work: Option<String>,
}


#[derive(Deserialize, Serialize, Clone,Default)]
pub struct LoginInfoEdit {
    pub info_id:i64,  
    pub device_type: Option<String>,
    pub ipaddr: Option<String>,
    pub login_location: Option<String>,
    pub browser: Option<String>,
    pub os: Option<String>,
    pub status: Option<String>,
    pub msg: Option<String>,
    pub login_time: Option<DateTime>,
    pub net_work: Option<String>,
}

#[derive(Deserialize, Clone, Debug, Serialize)]
pub struct ClientNetInfo {
    pub ip: String,
    pub location: String,
    pub net_work: String,
}

#[derive(Deserialize, Clone, Debug, Serialize)]
pub struct UserAgentInfo {
    pub browser: String,
    pub os: String,
    pub device: String,
}
 
