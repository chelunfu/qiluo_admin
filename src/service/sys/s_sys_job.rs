use crate::model::sys::model::msys_job::{
    JobAdd, JobDel, JobEdit, JobExecute, JobRes, SysJobModel, ValidateCronReq, ValidateCronRes,
};
use crate::service::prelude::*;
use crate::worker::{
    clear_periodic_worker,
    invokefunction::{InvokeFunctionMsg, InvokeFunctionWorker},
    periodic_worker,
    requesturl::{RequestUrlMsg, RequestUrlWorker},
};
use crate::worker::{AppWorker, Worker};
use cron_clock::Schedule;
use serde_json::{Result, Value};
use std::str::FromStr;
pub async fn list(VQuery(arg): VQuery<PageParams>) -> impl IntoResponse {
    let rlist = SysJobModel::list(arg).await;
    ApiResponse::from_result(rlist)
}

pub async fn edit(VJson(arg): VJson<JobEdit>) -> impl IntoResponse {
    let r = SysJobModel::edit(arg).await;
    ApiResponse::from_result(r)
}
pub async fn add(VJson(arg): VJson<JobAdd>) -> impl IntoResponse {
    let r = SysJobModel::add(arg).await;
    ApiResponse::from_result(r)
}
pub async fn delete(VJson(arg): VJson<JobDel>) -> impl IntoResponse {
    let r = SysJobModel::del(arg).await;
    ApiResponse::from_result(r)
}

pub async fn hand_execute_job(VJson(arg): VJson<JobExecute>) -> impl IntoResponse {
    let jobmodel = SysJobModel::find_by_id(arg.job_id).await;
    if let Ok(job) = jobmodel {
        execute_job(job).await;
        ApiResponse::ok("Success")
    } else {
        ApiResponse::ok("Falied")
    }
}

pub async fn validate_cron(VJson(arg): VJson<ValidateCronReq>) -> impl IntoResponse {
    let cron_expression = arg.cron_expression;
    let schedule = match Schedule::from_str(&cron_expression) {
        Ok(v) => v,
        Err(_) => {
            return ApiResponse::ok(ValidateCronRes {
                validate: false,
                next_ten: None,
            })
        }
    };
    let next_ten = schedule
        .upcoming(Local)
        .take(12)
        .map(|x| x.naive_local())
        .collect();
    let v = ValidateCronRes {
        validate: true,
        next_ten: Some(next_ten),
    };
    ApiResponse::ok(v)
}

pub async fn update_job() {
    clear_periodic_worker().await;
    let jobs = SysJobModel::all_job().await.unwrap_or_default();
    info!("update_job jobs: {}", jobs.len());
    for job in jobs {
        worker_execute_job(job).await;
    }
}

pub async fn worker_execute_job(job: JobRes) {
    if job.task_type == "geturl" {
        let cron_expression = job.cron_expression;
        if let Some(job_params) = job.job_params {
            let reqarg = RequestUrlMsg {
                url: job_params,
                job_id: job.job_id,
                task_type: job.task_type,
                job_name: job.job_name.clone(),
                job_group: job.job_group.clone(),
            };
            periodic_worker(
                &cron_expression,
                &job.job_name,
                &job.job_group,
                reqarg,
                RequestUrlWorker::class_name(),
            )
            .await;
        }
    } else if job.task_type == "invokefunction" {
        let cron_expression = job.cron_expression;
        if let Some(job_params) = job.job_params {
            let value_result: Result<Value> = serde_json::from_str(job_params.as_str());
            if value_result.is_ok() {
                let v = value_result.unwrap();
                let invokemsg = InvokeFunctionMsg {
                    job_id: Some(job.job_id),
                    callfun: v["callfun"].as_str().unwrap_or_default().to_string(),
                    parmets: v["parmets"].as_str().unwrap_or_default().to_string(),
                };
                periodic_worker(
                    &cron_expression,
                    &job.job_name,
                    &job.job_group,
                    invokemsg,
                    InvokeFunctionWorker::class_name(),
                )
                .await;
            }
        }
    }
}

pub async fn execute_job(job: JobRes) {
    info!("execute_job: {}  {}", job.job_name, job.task_type);
    if job.task_type == "geturl" {
        let reqarg = RequestUrlMsg {
            url: job.job_params.unwrap(),
            job_id: job.job_id,
            task_type: job.task_type,
            job_name: job.job_name.clone(),
            job_group: job.job_group.clone(),
        };
        let _ = RequestUrlWorker::enqueue_async(reqarg).await;
    } else if job.task_type == "invokefunction" {
        if let Some(job_params) = job.job_params {
            let value_result: Result<Value> = serde_json::from_str(job_params.as_str());

            if value_result.is_ok() {
                let v = value_result.unwrap();
                let invokemsg = InvokeFunctionMsg {
                    job_id: Some(job.job_id),
                    callfun: v["callfun"].as_str().unwrap_or_default().to_string(),
                    parmets: v["parmets"].as_str().unwrap_or_default().to_string(),
                };
                let _ = InvokeFunctionWorker::enqueue_async(invokemsg).await;
            } else {
                error!("value_result error");
            }
        } else {
            error!("job_params error");
        }
    }
}
