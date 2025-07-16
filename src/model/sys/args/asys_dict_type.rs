use crate::model::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize,FromQueryResult,Validate)]
pub struct DictTypeRes { 
    #[serde(with = "i64_to_string")]
    pub dict_id: i64,
    pub dict_name: Option<String>, 
    pub dict_type: Option<String>,
    pub order: i32,
    pub created_at: DateTime, 
    pub remark: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize,Validate)]
pub struct DictDataSearch { 
    pub dict_name: Option<String>, 
    pub dict_type: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize,Validate)]

pub struct DictTypeAdd {  
    pub dict_name: String, 
    pub dict_type: String,
    pub order: i32,
    pub created_at: Option<DateTime>, 
    pub remark: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize,Validate)]

pub struct DictTypeEdit {  
    #[serde(with = "i64_to_string")]
    pub dict_id: i64,
    pub dict_name: String, 
    pub dict_type: String,
    pub order: i32,
    pub remark: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize,Validate)]

pub struct DictTypeDel {  
    #[serde(with = "i64_to_string")]
    pub dict_id: i64,
}