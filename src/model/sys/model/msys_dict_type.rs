pub use super::args::asys_dict_type::*;
pub use super::entity::sys_dict_type::{self, ActiveModel, Model as SysDictTypeModel};
use super::msys_dict_data::SysDictDataModel;
use crate::model::prelude::*;

impl SysDictTypeModel {
    pub async fn list(arg: PageParams, search: DictDataSearch) -> Result<ListData<DictTypeRes>> {
        let page_num = arg.page_num.unwrap_or(1);
        let page_per_size = arg.page_size.unwrap_or(10);
        let db = DB().await;
        let mut rmodel = sys_dict_type::Entity::find();

        if let Some(dict_type) = search.dict_type {
            rmodel = rmodel.filter(sys_dict_type::Column::DictType.eq(dict_type));
        }
        if let Some(dict_name) = search.dict_name {
            rmodel = rmodel.filter(sys_dict_type::Column::DictName.eq(dict_name));
        }
        let total = rmodel.clone().count(db).await?;
        let paginator = rmodel
            .order_by_asc(sys_dict_type::Column::Order)
            .into_model::<DictTypeRes>()
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

    pub async fn add(arg: DictTypeAdd) -> Result<String> {
        let db = DB().await;
        let id = GID().await;
        let now = Local::now().naive_local();
        let amodel = sys_dict_type::ActiveModel {
            dict_id: Set(id),
            dict_name: Set(arg.dict_name),
            dict_type: Set(arg.dict_type),
            order: Set(arg.order),
            remark: Set(arg.remark),
            created_at: Set(now),
            updated_at: Set(now),
        };
        amodel.insert(db).await?;
        Ok("Success".to_string())
    }

    pub async fn edit(arg: DictTypeEdit) -> Result<String> {
        let db = DB().await;
        let now = Local::now().naive_local();
        let rmodel = sys_dict_type::Entity::find_by_id(arg.dict_id)
            .one(db)
            .await?;
        let mut amodel: sys_dict_type::ActiveModel = if let Some(r) = rmodel {
            r.into()
        } else {
            return Err("dict_type not found".into());
        };
        amodel.dict_name = Set(arg.dict_name);
        amodel.dict_type = Set(arg.dict_type);
        amodel.order = Set(arg.order);
        amodel.updated_at = Set(now);
        amodel.remark = Set(arg.remark);
        amodel.update(db).await?;
        Ok("Success".to_string())
    }
    pub async fn del(arg: DictTypeDel) -> Result<String> {
        let db = DB().await;
        let rmodel = sys_dict_type::Entity::find_by_id(arg.dict_id)
            .one(db)
            .await?;
        let model = if let Some(r) = rmodel {
            r
        } else {
            return Err("dict_type not found".into());
        };
        let txn = db.begin().await?;
        SysDictDataModel::delete_by_dict_type(model.dict_id, &txn).await?;
        sys_dict_type::Entity::delete_by_id(model.dict_id)
            .exec(&txn)
            .await?;
        txn.commit().await?;
        Ok("Success".to_string())
    }
}
