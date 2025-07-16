pub use super::entity::{
    sys_dept::{self},
    sys_role_dept::{self, ActiveModel, Model as SysRoleDeptModel},
};
use crate::model::prelude::*;

impl SysRoleDeptModel {
    pub async fn update_depts(role_id: i64, dept_ids: Vec<i64>) -> Result<String> {
        let db = DB().await;
        sys_role_dept::Entity::delete_many()
            .filter(sys_role_dept::Column::RoleId.eq(role_id))
            .exec(db)
            .await?;
        let txn = db.begin().await?;
        for menu_id in dept_ids {
            let mut role_menu = ActiveModel::new();
            role_menu.role_id = Set(role_id);
            role_menu.dept_id = Set(menu_id);
            sys_role_dept::Entity::insert(role_menu).exec(&txn).await?;
        }
        txn.commit().await?;
        Ok("".into())
    }

    pub async fn listdepts<T>(role_id: i64) -> Result<Vec<T>>
    where
        T: FromQueryResult,
    {
        let db = DB().await;
        let mut depts = sys_role_dept::Entity::find();
        depts = depts.columns([sys_role_dept::Column::DeptId, sys_role_dept::Column::RoleId]);
        depts = depts.join_rev(
            JoinType::LeftJoin,
            sys_dept::Entity::belongs_to(sys_role_dept::Entity)
                .from(sys_dept::Column::DeptId)
                .to(sys_role_dept::Column::DeptId)
                .into(),
        );
        depts = depts.filter(sys_role_dept::Column::RoleId.eq(role_id));
        depts.into_model::<T>().all(db).await.map_err(Into::into)
    }
}
