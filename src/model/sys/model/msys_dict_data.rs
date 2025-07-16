pub use super::args::asys_dict_data::*;
pub use super::entity::sys_dict_data::{self, ActiveModel, Model as SysDictDataModel};
use super::entity::sys_dict_type;
use crate::model::prelude::*;

impl SysDictDataModel {
    pub async fn list(arg: PageParams, search: DictDataSearch) -> Result<ListData<DictDataRes>> {
        let page_num = arg.page_num.unwrap_or(1);
        let page_per_size = arg.page_size.unwrap_or(10);
        let db = DB().await;
        let mut rmodel = sys_dict_data::Entity::find();

        rmodel = rmodel.filter(sys_dict_data::Column::DictTypeId.eq(search.dict_type_id));
        if let Some(dict_label) = search.dict_label {
            rmodel = rmodel.filter(sys_dict_data::Column::DictLabel.eq(dict_label));
        }

        if let Some(dict_value) = search.dict_value {
            rmodel = rmodel.filter(sys_dict_data::Column::DictValue.eq(dict_value));
        }

        let total = rmodel.clone().count(db).await?;
        let paginator = rmodel
            .order_by_asc(sys_dict_data::Column::DictSort)
            .into_model::<DictDataRes>()
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

    pub async fn add(arg: DictDataAdd) -> Result<String> {
        let db = DB().await;
        let id = GID().await;
        let amodel = sys_dict_data::ActiveModel {
            dict_code: Set(id),
            dict_type_id: Set(arg.dict_type_id),
            dict_label: Set(arg.dict_label),
            dict_value: Set(arg.dict_value),
            dict_sort: Set(arg.dict_sort),
            remark: Set(arg.remark),
            ..Default::default()
        };
        amodel.insert(db).await?;
        Ok("Success".to_string())
    }

    pub async fn edit(arg: DictDataEdit) -> Result<String> {
        let db = DB().await;
        let rmodel = sys_dict_data::Entity::find_by_id(arg.dict_code)
            .one(db)
            .await?;
        let mut amodel: sys_dict_data::ActiveModel = if let Some(r) = rmodel {
            r.into()
        } else {
            return Err("dict data not found".into());
        };
        amodel.dict_type_id = Set(arg.dict_type_id);
        amodel.dict_label = Set(arg.dict_label);
        amodel.dict_value = Set(arg.dict_value);
        amodel.dict_sort = Set(arg.dict_sort);
        amodel.remark = Set(arg.remark);
        amodel.update(db).await?;
        Ok("Success".to_string())
    }
    pub async fn del(arg: DictDataDel) -> Result<String> {
        let db = DB().await;
        sys_dict_data::Entity::delete_by_id(arg.dict_code)
            .exec(db)
            .await?;
        Ok("Success".to_string())
    }

    pub async fn get_by_type(arg: DictDataType) -> Result<Vec<DictKeyValueRes>> {
        let db = DB().await;
        let mut rmodel = sys_dict_data::Entity::find();
        rmodel = rmodel.join_rev(
            JoinType::LeftJoin,
            sys_dict_type::Entity::belongs_to(sys_dict_data::Entity)
                .from(sys_dict_type::Column::DictId)
                .to(sys_dict_data::Column::DictTypeId)
                .into(),
        );
        let rmodels = rmodel
            .filter(sys_dict_type::Column::DictType.eq(arg.dict_type))
            .order_by_asc(sys_dict_data::Column::DictSort)
            .into_model::<DictKeyValueRes>()
            .all(db)
            .await?;
        Ok(rmodels)
    }

    pub async fn delete_by_dict_type(dict_id: i64, db: &DatabaseTransaction) -> Result<String> {
        sys_dict_data::Entity::delete_many()
            .filter(sys_dict_data::Column::DictTypeId.eq(dict_id))
            .exec(db)
            .await?;
        Ok("Success".to_string())
    }
}
