pub use super::args::asys_oper_log::*;
pub use super::entity::sys_oper_log::{self, ActiveModel, Model as SysOperLogModel};
use crate::model::prelude::*;

impl SysOperLogModel {
    pub async fn list(
        arg: PageParams,
        search: SysOperLogSearch,
    ) -> Result<ListData<SysOperLogRes>> {
        let page_num = arg.page_num.unwrap_or(1);
        let page_per_size = arg.page_size.unwrap_or(10);
        let db = DB().await;
        let mut rmodel = sys_oper_log::Entity::find();

        if let Some(oper_name) = search.oper_name {
            rmodel = rmodel.filter(sys_oper_log::Column::OperName.contains(oper_name));
        }

        let total = rmodel.clone().count(db).await?;
        let paginator = rmodel
            .order_by_desc(sys_oper_log::Column::OperTime)
            .into_model::<SysOperLogRes>()
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
    pub async fn add(arg: SysOperLogAdd) -> Result<String> {
        let db = DB().await;
        let id = GID().await;
        let amodel = sys_oper_log::ActiveModel {
            oper_id: Set(id),
            api_name: Set(arg.api_name),
            method: Set(arg.method),
            oper_name: Set(arg.oper_name),
            oper_url: Set(arg.oper_url),
            oper_ip: Set(arg.oper_ip),
            oper_location: Set(arg.oper_location),
            oper_param: Set(arg.oper_param),
            json_result: Set(arg.json_result),
            status: Set(arg.status),
            error_msg: Set(arg.error_msg),
            oper_time: Set(arg.oper_time),
            request_method: Set(arg.request_method),
            cost_time: Set(arg.cost_time),
        };
        amodel.insert(db).await?;
        Ok("Success".to_string())
    }
}
