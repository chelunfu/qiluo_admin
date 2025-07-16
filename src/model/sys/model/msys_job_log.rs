pub use super::args::asys_job_log::*;
pub use super::entity::sys_job_log::{self, ActiveModel, Model as SysJobLogModel};
use crate::model::prelude::*;

impl SysJobLogModel {
    pub async fn list(arg: PageParams, search: JobLogSearch) -> Result<ListData<JobLogRes>> {
        let page_num = arg.page_num.unwrap_or(1);
        let page_per_size = arg.page_size.unwrap_or(10);
        let db = DB().await;
        let mut rmodel = sys_job_log::Entity::find();

        rmodel = rmodel.filter(sys_job_log::Column::JobId.eq(search.job_id));

        let total = rmodel.clone().count(db).await?;
        let paginator = rmodel
            .order_by_desc(sys_job_log::Column::CreatedAt)
            .into_model::<JobLogRes>()
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
    pub async fn add(arg: JobLogAdd) -> Result<String> {
        let id = GID().await;
        let db = DB().await;
        let imodel = sys_job_log::ActiveModel {
            id: Set(id),
            job_id: Set(arg.job_id),
            job_message: Set(arg.job_message),
            status: Set(arg.status),
            run_count: Set(arg.run_count),
            elapsed_time: Set(arg.elapsed_time),
            ..Default::default()
        };
        imodel.insert(db).await?;
        Ok("Success".to_string())
    }
    pub async fn del() {}
    pub async fn edit() {}
}
