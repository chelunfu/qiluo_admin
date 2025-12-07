use sea_orm::{ColumnTrait, Condition, DatabaseConnection, DbErr, EntityTrait, QueryFilter};

use super::DataScope;
use crate::model::sys::entity::{sys_dept, sys_role, sys_role_dept, sys_user};
/// 某个登录用户在「当前激活角色」下的数据范围
#[derive(Debug, Clone)]
pub struct DataScopeContext {
    /// 是否无限制（当前激活角色 ALL）
    pub all: bool,
    /// 是否包含“本人数据”（SELF）
    pub has_self: bool,
    /// 精确部门 id 列表（DEPT、自定义部门精确范围）
    pub dept_ids: Vec<i64>,
    /// 部门范围 (lft, rgt) 列表（本部门及以下、自定义部门及以下）
    pub dept_ranges: Vec<(i32, i32)>,
}

impl DataScopeContext {
    pub async fn from_user_id(db: &DatabaseConnection, user_id: i64) -> Result<Self, DbErr> {
        // 1. 查用户
        let user = sys_user::Entity::find_by_id(user_id)
            .one(db)
            .await?
            .ok_or_else(|| DbErr::RecordNotFound("user not found".into()))?;

        let user_dept_id = user.dept_id;
        let active_role_id = user.role_id;

        // 没有激活角色：最严格，默认仅本人
        if active_role_id == 0 {
            return Ok(Self {
                all: false,
                has_self: true,
                dept_ids: vec![],
                dept_ranges: vec![],
            });
        }

        // 2. 查当前激活角色
        let role = sys_role::Entity::find_by_id(active_role_id)
            .one(db)
            .await?
            .ok_or_else(|| DbErr::RecordNotFound("role not found".into()))?;

        let scope = DataScope::from_db(&role.data_scope);

        // 3. 按 data_scope 构造范围
        match scope {
            DataScope::All => {
                // 全部数据
                Ok(Self {
                    all: true,
                    has_self: false,
                    dept_ids: vec![],
                    dept_ranges: vec![],
                })
            }

            DataScope::SelfOnly => Ok(Self {
                all: false,
                has_self: true,
                dept_ids: vec![],
                dept_ranges: vec![],
            }),

            DataScope::Dept => {
                // 本部门
                Ok(Self {
                    all: false,
                    has_self: false,
                    dept_ids: vec![user_dept_id],
                    dept_ranges: vec![],
                })
            }

            DataScope::DeptAndSub => {
                // 本部门及以下：查 lft/rgt
                let user_dept = sys_dept::Entity::find_by_id(user_dept_id)
                    .one(db)
                    .await?
                    .ok_or_else(|| DbErr::RecordNotFound("dept not found".into()))?;

                Ok(Self {
                    all: false,
                    has_self: false,
                    dept_ids: vec![],
                    dept_ranges: vec![(user_dept.lft, user_dept.rgt)],
                })
            }

            DataScope::Custom => {
                // 自定义部门：sys_role_dept
                use std::collections::HashSet;
                let mut custom_ids: HashSet<i64> = HashSet::new();

                let rds = sys_role_dept::Entity::find()
                    .filter(sys_role_dept::Column::RoleId.eq(active_role_id))
                    .all(db)
                    .await?;

                for rd in rds {
                    custom_ids.insert(rd.dept_id);
                }

                if custom_ids.is_empty() {
                    return Ok(Self {
                        all: false,
                        has_self: false,
                        dept_ids: vec![],
                        dept_ranges: vec![],
                    });
                }

                // 策略：自定义部门及其子部门
                let depts = sys_dept::Entity::find()
                    .filter(
                        sys_dept::Column::DeptId
                            .is_in(custom_ids.iter().copied().collect::<Vec<_>>()),
                    )
                    .all(db)
                    .await?;

                let mut ranges = Vec::new();
                for d in depts {
                    ranges.push((d.lft, d.rgt));
                }

                Ok(Self {
                    all: false,
                    has_self: false,
                    dept_ids: vec![],
                    dept_ranges: ranges,
                })
            }
        }
    }

    pub fn to_scope_condition<DCol, OCol>(
        &self,
        dept_col: Option<DCol>,
        owner_col: Option<OCol>,
        user_id: i64,
    ) -> Option<Condition>
    where
        DCol: ColumnTrait + Clone,
        OCol: ColumnTrait + Clone,
    {
        if self.all {
            return None;
        }

        let mut cond = Condition::any();
        let mut has_any = false;

        // 1) SELF：仅本人
        if self.has_self {
            if let Some(owner) = owner_col {
                cond = cond.add(owner.eq(user_id));
                has_any = true;
            }
        }

        // 2) 精确部门（DEPT 等）
        if !self.dept_ids.is_empty() {
            if let Some(dept) = dept_col {
                cond = cond.add(dept.is_in(self.dept_ids.clone()));
                has_any = true;
            }
        }

        // 3) 范围型部门（DEPT_AND_SUB / CUSTOM）
        for (lft, rgt) in &self.dept_ranges {
            let range_cond = Condition::all()
                .add(sys_dept::Column::Lft.gte(*lft))
                .add(sys_dept::Column::Rgt.lte(*rgt));
            cond = cond.add(range_cond);
            has_any = true;
        }

        if !has_any {
            if let Some(owner) = owner_col {
                Some(
                    Condition::all()
                        .add(owner.is_null())
                        .add(owner.is_not_null()),
                )
            } else {
                dept_col.map(|dept| Condition::all().add(dept.is_null()).add(dept.is_not_null()))
            }
        } else {
            Some(cond)
        }
    }
    pub fn to_dept_condition_for_list(&self, current_user_dept_id: i64) -> Option<Condition> {
        if self.all {
            return None;
        }

        if self.has_self && self.dept_ids.is_empty() && self.dept_ranges.is_empty() {
            return Some(Condition::all().add(sys_dept::Column::DeptId.eq(current_user_dept_id)));
        }

        let mut cond = Condition::any();
        let mut has_any = false;

        if !self.dept_ids.is_empty() {
            cond = cond.add(sys_dept::Column::DeptId.is_in(self.dept_ids.clone()));
            has_any = true;
        }

        for (lft, rgt) in &self.dept_ranges {
            let range_cond = Condition::all()
                .add(sys_dept::Column::Lft.gte(*lft))
                .add(sys_dept::Column::Rgt.lte(*rgt));
            cond = cond.add(range_cond);
            has_any = true;
        }

        if !has_any {
            Some(
                Condition::all()
                    .add(sys_dept::Column::DeptId.is_null())
                    .add(sys_dept::Column::DeptId.is_not_null()),
            )
        } else {
            Some(cond)
        }
    }

    pub fn to_user_condition(&self, user_id: i64) -> Option<Condition> {
        if self.all {
            return None;
        }

        let mut cond = Condition::any();
        let mut has_any = false;

        // 1) SELF：仅本人
        if self.has_self {
            cond = cond.add(sys_user::Column::Id.eq(user_id));
            has_any = true;
        }

        // 2) 精确部门（DEPT 等）
        if !self.dept_ids.is_empty() {
            cond = cond.add(sys_user::Column::DeptId.is_in(self.dept_ids.clone()));
            has_any = true;
        }

        // 3) 范围型部门（DEPT_AND_SUB / CUSTOM：基于 lft/rgt）
        for (lft, rgt) in &self.dept_ranges {
            let range_cond = Condition::all()
                .add(sys_dept::Column::Lft.gte(*lft))
                .add(sys_dept::Column::Rgt.lte(*rgt));
            cond = cond.add(range_cond);
            has_any = true;
        }

        if !has_any {
            Some(
                Condition::all()
                    .add(sys_user::Column::Id.is_null())
                    .add(sys_user::Column::Id.is_not_null()),
            )
        } else {
            Some(cond)
        }
    }
}
