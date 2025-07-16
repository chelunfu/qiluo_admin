use crate::model::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize,FromQueryResult,Validate)]
pub struct DictDataRes { 
    #[serde(with = "i64_to_string")]
    pub dict_code: i64,
    pub dict_sort: i32,
    pub dict_label: String,
    pub dict_value: String,
    #[serde(with = "i64_to_string")]
    pub dict_type_id: i64, 
    pub created_at: Option<DateTime>, 
    pub remark: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize,Validate)]
pub struct DictDataSearch { 
    #[serde(with = "i64_to_string")]
    pub dict_type_id: i64,
    pub dict_label: Option<String>,
    pub dict_value: Option<String>
}
#[derive(Debug, Clone, Serialize, Deserialize,Validate)]
pub struct DictDataAdd {  
    pub dict_sort: i32,
    pub dict_label: String,
    pub dict_value: String,
    #[serde(with = "i64_to_string")]
    pub dict_type_id: i64, 
    pub remark: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize,Validate)]
pub struct DictDataEdit {  
    #[serde(with = "i64_to_string")]
    pub dict_code: i64,
    pub dict_sort: i32,
    pub dict_label: String,
    pub dict_value: String,
    #[serde(with = "i64_to_string")]
    pub dict_type_id: i64, 
    pub remark: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize,Validate)]

pub struct DictDataDel {  
    #[serde(with = "i64_to_string")]
    pub dict_code: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize,Validate)]

pub struct DictDataType {   
    // #[serde(with = "i64_to_string")]
    // pub dict_type_id: i64,
    pub dict_type: String,
}


#[derive(Debug, Clone, Serialize, Deserialize,FromQueryResult,Validate)]
pub struct DictKeyValueRes {  
    pub dict_label: String,
    pub dict_value: String, 
}