use crate::model::prelude::*;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CaptchaImage {
    pub captcha_on_off: bool,
    #[serde(with = "i64_to_string")]
    pub uuid: i64,
    pub img: String,
}

#[derive(Debug, Serialize, Deserialize, Clone,Validate)]
pub struct  ClientInfo{
    pub client_id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CaptchaCacheInfo {
     pub client_id: String,
     pub cache_text:String 
}