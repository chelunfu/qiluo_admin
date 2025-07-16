use crate::model::prelude::*;

pub use super::args::asys_role::*;
pub use super::entity::{
    sys_menu::{self, Model as SysMenuModel},
    sys_role::{self, ActiveModel, Model as SysRoleModel},
    sys_role_menu::{self, Model as SysRoleMenuModel},
};
use super::msys_role_dept::SysRoleDeptModel;

impl SysRoleModel {
    pub async fn list(arg: PageParams, search: RoleSearch) -> Result<ListData<RoleResp>> {
        let page_num = arg.page_num.unwrap_or(1);
        let page_per_size = arg.page_size.unwrap_or(10);
        let db = DB().await;
        let mut rmodel = sys_role::Entity::find();
        rmodel = rmodel.filter(sys_role::Column::DeletedAt.is_null());

        if let Some(role_name) = search.role_name {
            rmodel = rmodel.filter(sys_role::Column::RoleName.contains(role_name));
        }
        if let Some(role_key) = search.role_key {
            rmodel = rmodel.filter(sys_role::Column::RoleKey.contains(role_key));
        }

        let total = rmodel.clone().count(db).await?;
        let paginator = rmodel
            .order_by_asc(sys_role::Column::Order)
            .into_model::<RoleResp>()
            .paginate(db, page_per_size);
        let total_pages = paginator.num_pages().await?;
        let list = paginator.fetch_page(page_num - 1).await?;
        let res = ListData {
            list,
            total,
            total_pages,
            page_num,
        };
        Ok(res)
    }

    pub async fn tree() -> Result<Vec<RoleResp>> {
        let db = DB().await;
        let mut rmodel = sys_role::Entity::find();
        rmodel = rmodel.filter(sys_role::Column::DeletedAt.is_null());
        let list = rmodel.into_model::<RoleResp>().all(db).await?;
        Ok(list)
    }
    pub async fn menu() -> Result<Vec<RoleMenuResp>> {
        let db = DB().await;

        let mut rmodel = sys_role::Entity::find();

        rmodel = rmodel.columns([
            sys_role::Column::RoleId,
            sys_role::Column::RoleName,
            sys_role::Column::RoleKey,
            sys_role::Column::Order,
            sys_role::Column::DataScope,
            sys_role::Column::Status,
        ]);

        let mut mode = rmodel.into_model::<RoleMenuResp>().all(db).await.unwrap();

        for m in mode.iter_mut() {
            let ms1: Vec<RoleMResp> = SysRoleMenuModel::listmenus::<RoleMResp>(m.role_id)
                .await
                .unwrap();
            m.mens = ms1;
        }
        Ok(mode)
    }

    //根据角色ID获取角色菜单
    pub async fn get_role_menus(role_id: i64) -> Result<Vec<RoleMResp>> {
        SysRoleMenuModel::listmenus::<RoleMResp>(role_id).await
    }

    pub async fn get_role_depts(role_id: i64) -> Result<Vec<RoleDeptResp>> {
        SysRoleDeptModel::listdepts::<RoleDeptResp>(role_id).await
    }

    pub async fn edit(arg: RoleEditReq) -> Result<String> {
        let menu_ids = arg.menu;
        let db = DB().await;
        let rmodel = sys_role::Entity::find_by_id(arg.role_id).one(db).await?;

        let mut amodel: sys_role::ActiveModel = if let Some(rmodel) = rmodel {
            rmodel.into()
        } else {
            return Err("role not found".into());
        };
        amodel.role_name = Set(arg.role_name);
        amodel.role_key = Set(arg.role_key);
        amodel.order = Set(arg.order);
        amodel.data_scope = Set(arg.data_scope);
        amodel.status = Set(arg.status);
        amodel.remark = Set(arg.remark);
        let umodel = amodel.update(db).await;
        if umodel.is_err() {
            return Err("update failed".into());
        }
        let _ = SysRoleMenuModel::update_menus(arg.role_id, menu_ids).await;

        if let Some(data_depts) = arg.data_depts {
            let _ = SysRoleDeptModel::update_depts(arg.role_id, data_depts).await;
        }
        Ok("Success".to_string())
    }

    pub async fn add(arg: RoleAddReq) -> Result<String> {
        let db = DB().await;
        let id = GID().await;
        let amode = sys_role::ActiveModel {
            role_id: Set(id),
            role_name: Set(arg.role_name),
            role_key: Set(arg.role_key),
            order: Set(arg.order),
            data_scope: Set(arg.data_scope),
            status: Set(arg.status),
            ..Default::default()
        };
        amode.insert(db).await?;
        Ok("Success".to_string())
    }

    pub async fn delete(arg: RoleReq) -> Result<String> {
        let db = DB().await;
        let _ = SysRoleMenuModel::delete(arg.role_id).await;

        // 删除与角色ID相关的所有记录
        let dmodel = sys_role::Entity::delete_many()
            .filter(sys_role_menu::Column::RoleId.eq(arg.role_id))
            .exec(db)
            .await?;
        // 如果删除的记录数大于0，则返回成功
        if dmodel.rows_affected > 0 {
            Ok("Success".to_string())
        // 否则返回删除失败
        } else {
            Err("delete failed".into())
        }
    }
}
