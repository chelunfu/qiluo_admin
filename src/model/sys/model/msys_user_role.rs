pub use super::args::asys_user_role::*;
pub use super::entity::{
    sys_role::{self, Model as SysRoleModel},
    sys_user_role::{self, ActiveModel, Model as SysUserRoleModel},
};
use crate::model::prelude::*;

impl SysUserRoleModel {
    pub async fn find_role_id(uid: i64) -> Result<SysUserRoleModel> {
        let db = DB().await;
        sys_user_role::Entity::find()
            .filter(sys_user_role::Column::UserId.eq(uid))
            .one(db)
            .await?
            .ok_or("Failed to find role id".into())
    }

    pub async fn add_user_roles(uid: i64, role_ids: Vec<i64>) -> Result<String> {
        let db = DB().await;
        sys_user_role::Entity::delete_many()
            .filter(sys_user_role::Column::UserId.eq(uid))
            .exec(db)
            .await?;

        let mut imoses = Vec::new();
        for role_id in role_ids {
            imoses.push(sys_user_role::ActiveModel {
                user_id: Set(uid),
                role_id: Set(role_id),
            });
        }
        sys_user_role::Entity::insert_many(imoses).exec(db).await?;
        Ok("success".to_string())
    }

    pub async fn user_role_list(uid: i64) -> Result<Vec<i64>> {
        let db = DB().await;
        let user_roles = sys_user_role::Entity::find()
            .filter(sys_user_role::Column::UserId.eq(uid))
            .all(db)
            .await?;

        let ids = user_roles
            .into_iter()
            .map(|user_dept| user_dept.role_id)
            .collect();
        Ok(ids)
    }

    pub async fn user_role_name_list(uid: i64, rid: i64) -> Result<UserRoleAndUserResp> {
        let db = DB().await;
        let mut rmodel = sys_user_role::Entity::find();
        rmodel = rmodel.filter(sys_user_role::Column::UserId.eq(uid));
        rmodel = rmodel.join_rev(
            JoinType::LeftJoin,
            sys_role::Entity::belongs_to(sys_user_role::Entity)
                .from(sys_role::Column::RoleId)
                .to(sys_user_role::Column::RoleId)
                .into(),
        );
        rmodel = rmodel.columns([sys_role::Column::RoleName, sys_role::Column::RoleId]);
        let roles = rmodel.into_model::<UserRoleResp>().all(db).await?;
        let user_roles = UserRoleAndUserResp {
            user_role_id: rid,
            roles,
        };
        Ok(user_roles)
    }
}
