pub use super::args::asys_job::*;
pub use super::entity::sys_job::{self, ActiveModel, Model as SysJobModel};
use crate::model::prelude::*;
use crate::worker::job::{JobMsg, JobWorker};
use crate::worker::AppWorker;
impl SysJobModel {
    pub async fn find_by_id(jid: i64) -> Result<JobRes> {
        let db = DB().await;
        let rmodel = sys_job::Entity::find_by_id(jid)
            .into_model::<JobRes>()
            .one(db)
            .await?
            .ok_or_else(|| Error::Message("Not found".to_string()))?;
        Ok(rmodel)
    }
    pub async fn updata_run_count(jid: i64) -> Result<i32> {
        let db = DB().await;
        let rmodel = sys_job::Entity::find_by_id(jid).one(db).await?;
        let model = if let Some(r) = rmodel {
            r
        } else {
            return Err("Not found".into());
        };

        let ncount = model.run_count + 1;
        let mut amodel: sys_job::ActiveModel = model.into();

        amodel.run_count = Set(ncount);
        amodel.update(db).await?;
        Ok(ncount)
    }

    pub async fn list(arg: PageParams) -> Result<ListData<JobRes>> {
        let page_num = arg.page_num.unwrap_or(1);
        let page_per_size = arg.page_size.unwrap_or(10);

        let db = DB().await;
        let rmodel = sys_job::Entity::find();

        let total = rmodel.clone().count(db).await.unwrap();
        let paginator = rmodel
            .order_by_asc(sys_job::Column::CreatedAt)
            .into_model::<JobRes>()
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

    pub async fn all_job() -> Result<Vec<JobRes>> {
        let db = DB().await;
        let mut rmodel = sys_job::Entity::find();
        rmodel = rmodel.order_by_asc(sys_job::Column::UpdatedAt);
        rmodel = rmodel.filter(sys_job::Column::Status.eq("0"));
        let list = rmodel.into_model::<JobRes>().all(db).await?;
        Ok(list)
    }

    pub async fn add(arg: JobAdd) -> Result<String> {
        let db = DB().await;
        let id = GID().await;
        let amode = sys_job::ActiveModel {
            job_id: Set(id),
            job_name: Set(arg.job_name),
            job_group: Set(arg.job_group),
            task_type: Set(arg.task_type),
            task_count: Set(arg.task_count),
            run_count: Set(arg.run_count),
            job_params: Set(arg.job_params),
            cron_expression: Set(arg.cron_expression),
            status: Set(arg.status),
            remark: Set(arg.remark),
            ..Default::default()
        };
        amode.insert(db).await?;
        Ok("Success".to_string())
    }
    pub async fn del(arg: JobDel) -> Result<String> {
        let db = DB().await;
        let delresult = sys_job::Entity::delete_by_id(arg.job_id).exec(db).await?;
        if delresult.rows_affected > 0 {
            Ok("Success".to_string())
        } else {
            Err("Failed".into())
        }
    }
    pub async fn edit(arg: JobEdit) -> Result<String> {
        let db = DB().await;
        let rmodel = sys_job::Entity::find_by_id(arg.job_id).one(db).await?;
        let model = if let Some(r) = rmodel {
            r
        } else {
            return Err("Not found".into());
        };
        let now = Local::now().naive_local();
        let mut amodel: sys_job::ActiveModel = model.into();
        amodel.job_group = Set(arg.job_group);
        amodel.job_name = Set(arg.job_name);
        amodel.task_type = Set(arg.task_type);
        amodel.task_count = Set(arg.task_count);
        amodel.run_count = Set(arg.run_count);
        amodel.job_params = Set(arg.job_params);
        amodel.cron_expression = Set(arg.cron_expression);
        amodel.status = Set(arg.status.clone());
        amodel.remark = Set(arg.remark);
        amodel.updated_at = Set(now);

        let jobmss = JobMsg { job_id: arg.job_id };
        let _ = JobWorker::execute_async(jobmss).await;

        amodel.update(db).await?;
        Ok("Success".to_string())
    }
}
