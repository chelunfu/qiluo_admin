pub use super::entity::{
    sys_menu::{self},
    sys_role_menu::{self, ActiveModel, Model as SysRoleMenuModel},
};
use crate::model::prelude::*;
impl SysRoleMenuModel {
    pub async fn listmenus<T>(role_id: i64) -> Result<Vec<T>>
    where
        T: FromQueryResult,
    {
        let db = DB().await;
        let mut menus = sys_role_menu::Entity::find();
        menus = menus.columns([sys_menu::Column::Id, sys_menu::Column::Title]);
        menus = menus.join_rev(
            JoinType::LeftJoin,
            sys_menu::Entity::belongs_to(sys_role_menu::Entity)
                .from(sys_menu::Column::Id)
                .to(sys_role_menu::Column::MenuId)
                .into(),
        );
        menus = menus.filter(sys_role_menu::Column::RoleId.eq(role_id));
        menus.into_model::<T>().all(db).await.map_err(Into::into)
    }

    pub async fn update_menus(role_id: i64, menu_ids: Vec<i64>) -> Result<String> {
        let db = DB().await;
        sys_role_menu::Entity::delete_many()
            .filter(sys_role_menu::Column::RoleId.eq(role_id))
            .exec(db)
            .await?; 
        for menu_id in menu_ids {
            let mut role_menu = ActiveModel::new();
            role_menu.role_id = Set(role_id);
            role_menu.menu_id = Set(menu_id);
            role_menu.insert(db).await?;
        }
        Ok("Success".to_string())
    }
    pub async fn delete(role_id: i64) -> Result<String> {
        let db = DB().await;
        let dmodel = sys_role_menu::Entity::delete_many()
            .filter(sys_role_menu::Column::RoleId.eq(role_id))
            .exec(db)
            .await?;
        if dmodel.rows_affected > 0 {
            Ok("Success".to_string())
        } else {
            Err("delete failed".into())
        }
    }
}
