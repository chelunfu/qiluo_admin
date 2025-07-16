pub use super::args::asys_user_dept::*;
pub use super::entity::{
    sys_dept::{self, Model as SysDeptModel},
    sys_user_dept::{self, ActiveModel, Model as SysUserDeptModel},
};
use crate::model::prelude::*;

impl SysUserDeptModel {
    pub async fn add_user_depts(uid: i64, dept_ids: Vec<i64>) -> Result<String> {
        let db = DB().await;
        sys_user_dept::Entity::delete_many()
            .filter(sys_user_dept::Column::UserId.eq(uid))
            .exec(db)
            .await?;

        let mut imoses = Vec::new();
        for dept_id in dept_ids {
            imoses.push(sys_user_dept::ActiveModel {
                user_id: Set(uid),
                dept_id: Set(dept_id),
            });
        }
        sys_user_dept::Entity::insert_many(imoses).exec(db).await?;
        Ok("success".to_string())
    }

    pub async fn user_dept_list(uid: i64) -> Result<Vec<i64>> {
        let db = DB().await;
        let user_depts = sys_user_dept::Entity::find()
            .filter(sys_user_dept::Column::UserId.eq(uid))
            .all(db)
            .await?;

        let ids = user_depts
            .into_iter()
            .map(|user_dept| user_dept.dept_id)
            .collect();
        Ok(ids)
    }

    pub async fn user_dept_name_list(uid: i64, did: i64) -> Result<UserDeptAndUserResp> {
        let db = DB().await;
        let mut rmodel = sys_user_dept::Entity::find();
        rmodel = rmodel.filter(sys_user_dept::Column::UserId.eq(uid));
        rmodel = rmodel.join_rev(
            JoinType::LeftJoin,
            sys_dept::Entity::belongs_to(sys_user_dept::Entity)
                .from(sys_dept::Column::DeptId)
                .to(sys_user_dept::Column::DeptId)
                .into(),
        );
        rmodel = rmodel.columns([sys_dept::Column::DeptName, sys_dept::Column::DeptId]);
        let depts = rmodel.into_model::<UserDeptResp>().all(db).await?;
        let userdepts = UserDeptAndUserResp {
            user_dept_id: did,
            depts,
        };
        Ok(userdepts)
    }
}
