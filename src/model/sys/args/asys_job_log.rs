use crate::model::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize,Default,FromQueryResult)]
pub struct JobLogRes {  
    pub id: i64,
    pub job_id: i64,  
    pub run_count: i32,
    pub job_message: Option<String>,
    pub status: String, 
    pub elapsed_time: Option<DateTime>,
}


#[derive(Debug, Clone, Serialize, Deserialize,Default)]
pub struct JobLogAdd {  
    pub job_id: i64,  
    pub run_count: i32,
    pub job_message: Option<String>,
    pub status: String, 
    pub elapsed_time: Option<DateTime>,
}
 
#[derive(Debug, Clone, Serialize, Deserialize,Default,Validate)]
pub struct JobLogSearch {  
    pub job_id: String
}