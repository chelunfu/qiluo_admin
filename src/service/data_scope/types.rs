/// 一条记录的归属信息（部门 + 人）
#[derive(Debug, Clone, Copy)]
pub struct RecordScope {
    pub dept_id: i64,
    pub owner_id: i64,
}

/// 写操作类型：创建 / 更新 / 删除
pub enum RecordOp {
    /// 创建：
    /// - req_dept_id / req_owner_id 是前端“想要”的归属（可空）
    /// - 当前用户 + 角色 data_scope 决定最终可用的归属
    Create {
        req_dept_id: Option<i64>,
        req_owner_id: Option<i64>,
    },

    /// 更新：
    /// - old: 旧记录的归属（从 DB 查出来）
    /// - new_dept_id / new_owner_id：前端想改成的归属（可空 = 不改）
    Update {
        old: RecordScope,
        new_dept_id: Option<i64>,
        new_owner_id: Option<i64>,
    },

    /// 删除：
    /// - old: 旧记录的归属（从 DB 查出来）
    Delete { old: RecordScope },
}
/// 角色的数据范围枚举
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DataScope {
    /// 全部数据
    All,
    /// 自定义部门
    Custom,
    /// 本部门
    Dept,
    /// 本部门及以下
    DeptAndSub,
    /// 仅本人
    SelfOnly,
}

impl DataScope {
    pub fn from_db(v: &str) -> Self {
        match v {
            "1" | "ALL" => DataScope::All,
            "2" | "CUSTOM" => DataScope::Custom,
            "3" | "DEPT" => DataScope::Dept,
            "4" | "DEPT_SUB" => DataScope::DeptAndSub,
            "5" | "SELF" => DataScope::SelfOnly,
            _ => {
                tracing::warn!("Unknown data_scope value: {}", v);
                DataScope::SelfOnly
            }
        }
    }
}
