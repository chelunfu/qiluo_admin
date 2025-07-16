pub use super::args::asys_role_api::*;
pub use super::entity::sys_role_api::{self, ActiveModel, Model as SysRoleApiModel};
use super::msys_api_permission::SysApiPermissionModel;
use crate::model::prelude::*;

impl SysRoleApiModel {
    pub async fn list(arg: PageParams, search: RoleApiSearch) -> Result<ListData<RoleApiInfo>> {
        let page_num = arg.page_num.unwrap_or(1);
        let page_per_size = arg.page_size.unwrap_or(10);

        let db = DB().await;
        let mut rmodel = sys_role_api::Entity::find();
        rmodel = rmodel.filter(sys_role_api::Column::RoleId.eq(search.role_id));
        if let Some(api) = search.api {
            rmodel = rmodel.filter(sys_role_api::Column::Api.contains(api));
        }

        if let Some(apiname) = search.apiname {
            rmodel = rmodel.filter(sys_role_api::Column::Apiname.contains(apiname));
        }

        let total = rmodel.clone().count(db).await.unwrap();
        let paginator = rmodel
            .order_by_asc(sys_role_api::Column::Sort)
            .into_model::<RoleApiInfo>()
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

    pub async fn add(arg: RoleApiAdd) -> Result<String> {
        let db = DB().await;

        let modes = sys_role_api::Entity::find()
            .filter(
                Condition::all()
                    .add(sys_role_api::Column::RoleId.eq(arg.role_id))
                    .add(sys_role_api::Column::Api.eq(arg.api.to_string())),
            )
            .one(db)
            .await?;

        if modes.is_some() {
            // 存在
            return Err("API Has".into());
        }

        let id = GID().await;
        let amodel = sys_role_api::ActiveModel {
            id: Set(id),
            role_id: Set(arg.role_id),
            api: Set(arg.api.to_string()),
            method: Set(arg.method.to_string()),
            ..Default::default()
        };
        let _ = amodel.insert(db).await.unwrap();
        Ok("model".to_string())
    }

    pub async fn del(arg: RoleApiInfo) -> Result<String> {
        let db = DB().await;
        let model = sys_role_api::Entity::find()
            .filter(
                Condition::all()
                    .add(sys_role_api::Column::RoleId.eq(arg.role_id.to_string()))
                    .add(sys_role_api::Column::Api.eq(arg.api.to_string()))
                    .add(sys_role_api::Column::Method.eq(arg.method.to_string())),
            )
            .one(db)
            .await
            .unwrap();
        match model {
            Some(model) => {
                //let amodel = model.into_active_model();
                model.delete(db).await.unwrap();
                Ok("Success".to_string())
            }
            None => Ok("None".to_string()),
        }
    }

    //获取角色权限
    pub async fn role_permission_list(role_id: i64) -> Result<Vec<String>> {
        let db = DB().await;
        let mut rmodel = sys_role_api::Entity::find();
        if !APPCOFIG.system.super_role.contains(&role_id) {
            rmodel = rmodel.filter(sys_role_api::Column::RoleId.eq(role_id));
        }
        let list = rmodel.all(db).await?;
        let permissions = list
            .iter()
            .map(|x| x.api.to_string())
            .collect::<Vec<String>>();
        Ok(permissions)
    }

    //穿梭框的数据
    pub async fn role_api_transfer_list(
        arg: RoleApiTransferReq,
    ) -> Result<Vec<RoleApiTransferInfo>> {
        let db = DB().await;
        let list = sys_role_api::Entity::find()
            .filter(sys_role_api::Column::RoleId.eq(arg.role_id.to_string()))
            .into_model::<RoleApiTransferInfo>()
            .all(db)
            .await?;
        Ok(list)
    }
    pub async fn add_many_role_api_transfer(arg: RoleApiTransferListIdReq) -> Result<String> {
        let db = DB().await;
        sys_role_api::Entity::delete_many()
            .filter(sys_role_api::Column::RoleId.eq(arg.role_id))
            .exec(db)
            .await?;
        for (index, apiid) in arg.api_ids.into_iter().enumerate() {
            let osysapimodel = SysApiPermissionModel::find_by_id(apiid).await?;
            if let Some(sysapimodel) = osysapimodel {
                let gid = GID().await;
                let amodel = sys_role_api::ActiveModel {
                    id: Set(gid),
                    role_id: Set(arg.role_id),
                    api: Set(sysapimodel.api),
                    api_id: Set(sysapimodel.id),
                    method: Set(sysapimodel.method),
                    apiname:Set(sysapimodel.apiname),
                    sort: Set((index + 1) as i32),
                };
                let _ = amodel.insert(db).await.unwrap();
            }
        }
        Ok("Success".to_string())
    }
    ///检查权限
    pub async fn check_api(arg: RoleApiCheckInfo) -> bool {
        info!("check_api:{:?}", arg);
        let model = sys_role_api::Entity::find()
            .filter(
                Condition::all()
                    .add(sys_role_api::Column::RoleId.eq(arg.role_id))
                    .add(sys_role_api::Column::Api.eq(arg.api))
                    .add(sys_role_api::Column::Method.eq(arg.method)),
            )
            .one(DB().await)
            .await
            .unwrap();
        model.is_some()
    }
}
