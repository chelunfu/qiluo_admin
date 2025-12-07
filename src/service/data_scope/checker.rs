use sea_orm::{ColumnTrait, DatabaseConnection, DbErr, EntityTrait, QueryFilter};

use super::{DataScope, RecordOp, RecordScope};
use crate::model::{
    prelude::UserInfo,
    sys::entity::{sys_dept, sys_role, sys_role_dept, sys_user},
};

/// 统一的“写权限 + 归属”检查：
/// - 根据当前用户 role.data_scope 决定权限范围
/// - 结合部门树 / role_dept 判断目标 dept/owner 是否在范围内
/// - 返回“最终可写入”的 dept_id / owner_id
pub async fn check_record_scope(
    db: &DatabaseConnection,
    user: &UserInfo,
    op: RecordOp,
) -> Result<RecordScope, DbErr> {
    // 1. 当前角色的数据范围
    let role = sys_role::Entity::find_by_id(user.rid)
        .one(db)
        .await?
        .ok_or_else(|| DbErr::RecordNotFound("role not found".into()))?;
    let scope = DataScope::from_db(&role.data_scope);

    // 3. 根据 op 类型和 scope，做统一处理
    match op {
        // 创建：返回“最终的 dept_id / owner_id”（可能和前端请求不同）
        RecordOp::Create {
            req_dept_id,
            req_owner_id,
        } => {
            match scope {
                DataScope::All => {
                    // ALL：允许指定任意归属（你也可以再额外加限制）
                    let dept_id = req_dept_id.unwrap_or(user.did);
                    let owner_id = req_owner_id.unwrap_or(user.uid);
                    Ok(RecordScope { dept_id, owner_id })
                }

                DataScope::SelfOnly => {
                    // 仅本人：忽略前端传的归属，一律自己 + 自己部门
                    Ok(RecordScope {
                        dept_id: user.did,
                        owner_id: user.uid,
                    })
                }

                DataScope::Dept | DataScope::DeptAndSub | DataScope::Custom => {
                    // 按部门范围：
                    let dept_id = req_dept_id.unwrap_or(user.did);
                    if !dept_in_scope(db, user, scope, dept_id).await? {
                        return Err(DbErr::Custom("无权在该部门创建数据".to_string()));
                    }

                    let owner_id = req_owner_id.unwrap_or(user.uid);
                    if !owner_in_scope(db, user, scope, owner_id).await? {
                        return Err(DbErr::Custom("无权把数据分配给该用户".to_string()));
                    }

                    Ok(RecordScope { dept_id, owner_id })
                }
            }
        }

        // 更新：先判断“有没有资格管理旧记录”，再判断新归属是否合法，最后返回“最终归属”
        RecordOp::Update {
            old,
            new_dept_id,
            new_owner_id,
        } => {
            let in_old_scope = match scope {
                DataScope::All => true,
                DataScope::SelfOnly => old.owner_id == user.uid,
                DataScope::Dept | DataScope::DeptAndSub | DataScope::Custom => {
                    dept_in_scope(db, user, scope, old.dept_id).await?
                }
            };

            if !in_old_scope {
                return Err(DbErr::Custom("无权修改这条数据".to_string()));
            }

            let mut final_dept_id = old.dept_id;
            let mut final_owner_id = old.owner_id;

            match scope {
                DataScope::SelfOnly => {
                    // 仅本人：不允许改归属，忽略 new_dept_id / new_owner_id
                }
                DataScope::All | DataScope::Dept | DataScope::DeptAndSub | DataScope::Custom => {
                    if let Some(did) = new_dept_id {
                        if !dept_in_scope(db, user, scope, did).await? {
                            return Err(DbErr::Custom("无权把数据移到该部门".to_string()));
                        }
                        final_dept_id = did;
                    }

                    if let Some(oid) = new_owner_id {
                        if !owner_in_scope(db, user, scope, oid).await? {
                            return Err(DbErr::Custom("无权把数据分配给该用户".to_string()));
                        }
                        final_owner_id = oid;
                    }
                }
            }

            Ok(RecordScope {
                dept_id: final_dept_id,
                owner_id: final_owner_id,
            })
        }

        // 删除：只判断有没有资格删，不修改归属
        RecordOp::Delete { old } => {
            let can_delete = match scope {
                DataScope::All => true,
                DataScope::SelfOnly => old.owner_id == user.uid,
                DataScope::Dept | DataScope::DeptAndSub | DataScope::Custom => {
                    dept_in_scope(db, user, scope, old.dept_id).await?
                }
            };

            if !can_delete {
                return Err(DbErr::Custom("无权删除这条数据".to_string()));
            }

            Ok(old)
        }
    }
}

async fn dept_in_scope(
    db: &DatabaseConnection,
    user: &UserInfo,
    scope: DataScope,
    target_dept_id: i64,
) -> Result<bool, DbErr> {
    match scope {
        DataScope::All => Ok(true),

        DataScope::SelfOnly => Ok(target_dept_id == user.did),

        DataScope::Dept => Ok(target_dept_id == user.did),

        DataScope::DeptAndSub => {
            let user_dept = sys_dept::Entity::find_by_id(user.did)
                .one(db)
                .await?
                .ok_or_else(|| DbErr::RecordNotFound("user dept not found".into()))?;
            let target_dept = sys_dept::Entity::find_by_id(target_dept_id)
                .one(db)
                .await?
                .ok_or_else(|| DbErr::RecordNotFound("target dept not found".into()))?;
            Ok(target_dept.lft >= user_dept.lft && target_dept.rgt <= user_dept.rgt)
        }

        DataScope::Custom => {
            let rds = sys_role_dept::Entity::find()
                .filter(sys_role_dept::Column::RoleId.eq(user.rid))
                .all(db)
                .await?;

            if rds.is_empty() {
                return Ok(false);
            }

            let root_ids: Vec<i64> = rds.iter().map(|rd| rd.dept_id).collect();
            let roots = sys_dept::Entity::find()
                .filter(sys_dept::Column::DeptId.is_in(root_ids.clone()))
                .all(db)
                .await?;

            if roots.is_empty() {
                return Ok(false);
            }

            let target_dept = sys_dept::Entity::find_by_id(target_dept_id)
                .one(db)
                .await?
                .ok_or_else(|| DbErr::RecordNotFound("target dept not found".into()))?;

            for root in roots {
                if target_dept.lft >= root.lft && target_dept.rgt <= root.rgt {
                    return Ok(true);
                }
            }
            Ok(false)
        }
    }
}

async fn owner_in_scope(
    db: &DatabaseConnection,
    user: &UserInfo,
    scope: DataScope,
    target_owner_id: i64,
) -> Result<bool, DbErr> {
    let target_user = sys_user::Entity::find_by_id(target_owner_id)
        .one(db)
        .await?
        .ok_or_else(|| DbErr::RecordNotFound("target owner not found".into()))?;
    dept_in_scope(db, user, scope, target_user.dept_id).await
}
