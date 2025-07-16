use crate::model::prelude::*;

#[derive(Serialize, Clone, Debug, Default, Deserialize,FromQueryResult)]
pub struct SysOperLogRes {  
    #[serde(with = "i64_to_string")]
    pub oper_id: i64,
    pub api_name: Option<String>, 
    pub method: Option<String>,
    pub request_method: Option<String>, 
    pub oper_name: Option<String>, 
    pub oper_url: Option<String>,
    pub oper_ip: Option<String>,
    pub oper_location: Option<String>,
    pub oper_param: Option<String>,
    pub json_result: Option<String>,
    pub status: Option<String>,
    pub error_msg: Option<String>,
    pub oper_time: Option<DateTime>,
    pub cost_time: Option<i64>,
}

#[derive(Serialize, Clone, Debug, Default, Deserialize)]
pub struct SysOperLogAdd {  
    pub api_name: Option<String>, 
    pub method: Option<String>,
    pub request_method: Option<String>, 
    pub oper_name: Option<String>, 
    pub oper_url: Option<String>,
    pub oper_ip: Option<String>,
    pub oper_location: Option<String>,
    pub oper_param: Option<String>,
    pub json_result: Option<String>,
    pub status: Option<String>,
    pub error_msg: Option<String>,
    pub oper_time: Option<DateTime>,
    pub cost_time: Option<i64>,
}

#[derive(Serialize, Clone, Debug,  Validate, Deserialize)]
pub struct SysOperLogSearch {  
    pub oper_name: Option<String>, 
}