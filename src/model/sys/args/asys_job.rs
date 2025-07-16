use crate::model::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize,FromQueryResult)]
pub struct JobRes {  
    #[serde(with = "i64_to_string")]
    pub job_id: i64,
    pub task_type: String,
    pub task_count: i32,
    pub run_count: i32,
    pub job_name: String,
    pub job_params: Option<String>,
    pub job_group: String, 
    pub cron_expression: String, 
    pub status: String,
    pub remark: String
}

#[derive(Serialize, Clone, Validate, Default, Deserialize)]
pub struct JobAdd { 
    pub task_type: String,
    pub task_count: i32,
    pub run_count: i32,
    pub job_name: String,
    pub job_params: Option<String>,
    pub job_group: String, 
    pub cron_expression: String,
    pub status: String,
    pub remark: String
}


#[derive(Serialize, Clone, Validate, Default, Deserialize)]
pub struct JobEdit {
    #[serde(with = "i64_to_string")]
    pub job_id: i64,
    pub task_type: String,
    pub task_count: i32,
    pub run_count: i32,
    pub job_name: String,
    pub job_params: Option<String>,
    pub job_group: String, 
    pub cron_expression: String,
    pub status: String,
    pub remark: String
}

#[derive(Serialize, Clone, Validate, Default, Deserialize)]
pub struct JobDel {
    #[serde(with = "i64_to_string")]
    pub job_id: i64
}
#[derive(Serialize, Clone, Validate, Default, Deserialize)]
pub struct JobExecute {
    #[serde(with = "i64_to_string")]
    pub job_id: i64
}

#[derive(Deserialize, Clone, Debug,Serialize,Validate)]
pub struct ValidateCronReq {
    pub cron_expression: String,
}

#[derive(Serialize, Clone, Debug,Deserialize)]
pub struct ValidateCronRes {
    pub validate: bool,
    pub next_ten: Option<Vec<DateTime>>,
}