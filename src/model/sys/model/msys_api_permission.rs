pub use super::args::asys_api_permission::*;
pub use super::entity::sys_api_permission::{self, ActiveModel, Model as SysApiPermissionModel};
use crate::model::prelude::*;

impl SysApiPermissionModel {
    pub async fn list(
        arg: PageParams,
        search: ApiPermissionSearch,
    ) -> Result<ListData<ApiPermissionRes>> {
        let page_num = arg.page_num.unwrap_or(1);
        let page_per_size = arg.page_size.unwrap_or(10);
        let db = DB().await;
        let mut rmodel = sys_api_permission::Entity::find();

        if let Some(api) = search.api {
            rmodel = rmodel.filter(sys_api_permission::Column::Api.contains(api));
        }

        if let Some(apiname) = search.apiname {
            rmodel = rmodel.filter(sys_api_permission::Column::Apiname.contains(apiname));
        }
        if let Some(method) = search.method {
            rmodel = rmodel.filter(sys_api_permission::Column::Method.eq(method));
        }
        let total = rmodel.clone().count(db).await?;
        let paginator = rmodel
            .order_by_asc(sys_api_permission::Column::Sort)
            .into_model::<ApiPermissionRes>()
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

    pub async fn edit(arg: ApiPermissionEdit) -> Result<String> {
        let db = DB().await;
        let rmodel = sys_api_permission::Entity::find_by_id(arg.id)
            .one(db)
            .await?;

        let mut amodel: sys_api_permission::ActiveModel = if let Some(r) = rmodel {
            r.into()
        } else {
            return Err("role not found".into());
        };
        amodel.logcache = Set(arg.logcache);
        amodel.sort = Set(arg.sort);
        amodel.remark = Set(arg.remark);
        amodel.update(db).await?;
        Ok("Success".to_string())
    }

    pub async fn find_by_id(id: i64) -> Result<Option<SysApiPermissionModel>> {
        let db = DB().await;
        sys_api_permission::Entity::find_by_id(id)
            .one(db)
            .await
            .map_err(Into::into)
    }

    pub async fn add_or_update(
        arg: ApiPermissionAdd,
        db: &DatabaseTransaction,
    ) -> Result<ApiPermissionRes> {
        let existing = sys_api_permission::Entity::find()
            .filter(sys_api_permission::Column::Api.eq(arg.api.clone()))
            .one(db)
            .await?;
        let model = if let Some(m) = existing {
            let mut amodel: sys_api_permission::ActiveModel = m.into();
            amodel.api = Set(arg.api);
            amodel.method = Set(arg.method);
            amodel.apiname = Set(arg.apiname);
            sys_api_permission::Entity::update(amodel).exec(db).await?
        } else {
            let id = GID().await;
            let now = Local::now().naive_local();
            let amode = sys_api_permission::ActiveModel {
                id: Set(id),
                api: Set(arg.api),
                sort: Set(arg.sort),
                method: Set(arg.method),
                apiname: Set(arg.apiname),
                logcache: Set('0'.to_string()),
                created_at: Set(now),
                remark: Set(None),
            };
            sys_api_permission::Entity::insert(amode)
                .exec_with_returning(db)
                .await?
        };
        Ok(ApiPermissionRes {
            id: model.id,
            api: model.api,
            method: model.method,
            apiname: model.apiname,
            sort: model.sort,
            logcache: model.logcache,
            remark: model.remark,
        })
    }
    pub async fn delete_all() -> Result<String> {
        let db = DB().await;
        sys_api_permission::Entity::delete_many().exec(db).await?;
        Ok("Success".to_string())
    }

    pub async fn all_apis() -> Result<Vec<SysApiPermissionModel>> {
        let db = DB().await;
        sys_api_permission::Entity::find()
            .all(db)
            .await
            .map_err(Into::into)
    }
}
