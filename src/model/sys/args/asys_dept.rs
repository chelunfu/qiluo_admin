use crate::model::prelude::*;

#[derive(Debug, Deserialize, Serialize, FromQueryResult, Clone, Default)]
pub struct DeptResp {
    #[serde(with = "i64_to_string")]
    pub dept_id: i64,
    #[serde(with = "i64_to_string")]
    pub parent_id: i64,
    pub dept_name: Option<String>,
    pub dept_category: Option<String>,
    pub order: i32,
    pub leader: Option<i64>,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub status: String, 
    pub remark: Option<String>,
}
#[derive(Serialize, Clone, Debug, Default, Deserialize)]
pub struct DeptTree {
    #[serde(flatten)]
    pub dept: DeptResp,
    pub children: Option<Vec<DeptTree>>,
}

#[derive(Serialize, Clone, Validate, Default, Deserialize)]
pub struct SysDeptEdit {
    #[serde(with = "i64_to_string")]
    pub dept_id: i64,
    #[serde(with = "i64_to_string")]
    pub parent_id: i64,
    pub dept_name: Option<String>,
    pub dept_category: Option<String>,
    pub order: i32,
    pub leader: Option<i64>,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub status: String,
    pub remark: Option<String>,
}

#[derive(Serialize, Clone, Validate, Default, Deserialize)]
pub struct SysDeptAdd {
    #[serde(with = "i64_to_string")]
    pub parent_id: i64,
    pub dept_name: Option<String>,
    pub dept_category: Option<String>,
    pub order: i32,
    pub leader: Option<i64>,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub status: String,
    pub remark: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone, Validate)]
pub struct SysDeptDel {    
    #[serde(with = "i64_to_string")]
    pub dept_id: i64,
}

#[derive(Debug, Deserialize, Serialize, Clone, Validate)]
pub struct SysDeptSearch { 
    pub dept_name: Option<String>,
}

#[derive(Serialize, Clone, Debug, Default, Deserialize)]
pub struct DeptTreeData {
    pub dept_id: i64,
    pub parent_id: i64,
    pub lft: i32,
    pub rgt: i32,
    pub depth: i32,
    pub children: Option<Vec<DeptTreeData>>,
}

#[derive(Debug, Deserialize, Serialize, FromQueryResult, Clone, Default)]
pub struct DeptData {
    pub dept_id: i64,
    pub parent_id: i64,
    pub lft: i32,
    pub rgt: i32,
    pub depth: i32,
}


