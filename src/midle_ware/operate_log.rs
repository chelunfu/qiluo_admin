use super::{auth::ReqCtx, jwt::UserInfo};
use crate::cache::CacheManager;
use crate::common::result::RespDataString;
use crate::model::sys::model::msys_api_permission::ApiPermissionRes;
use crate::model::sys::model::msys_oper_log::SysOperLogAdd;
use crate::service::sys::s_sys_operation_log::add_oper_log;
use crate::service::sys::s_sys_white_jwt;
use axum::{extract::Request, http::StatusCode, middleware::Next, response::IntoResponse};
use chrono::Local;
use std::time::{Duration, Instant};
pub async fn operate_log_fn_mid(
    req: Request,
    next: Next,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let req_ctx = match req.extensions().get::<ReqCtx>() {
        Some(x) => x.clone(),
        None => return Ok(next.run(req).await),
    };

    let ctx_user = match req.extensions().get::<UserInfo>() {
        Some(x) => x.clone(),
        None => return Ok(next.run(req).await),
    };
    let now = Instant::now();
    let res_end = next.run(req).await;
    let duration = now.elapsed();
    let respdata = match res_end.extensions().get::<RespDataString>() {
        Some(x) => x.0.clone(),
        None => "".to_string(),
    };

    oper_log_add(
        req_ctx,
        ctx_user,
        respdata,
        "1".to_string(),
        "".to_string(),
        duration,
    )
    .await;
    Ok(res_end)
}
pub async fn oper_log_add(
    ctx: ReqCtx,
    ctx_user: UserInfo,
    respdata: String,
    status: String,
    err_msg: String,
    duration: Duration,
) {
    tokio::spawn(async move {
        match oper_log_add_fn(ctx, ctx_user, respdata, status, err_msg, duration).await {
            Ok(_) => {}
            Err(e) => {
                tracing::info!("日志添加失败：{}", e.to_string());
            }
        };
    });
}

pub async fn oper_log_add_fn(
    ctx: ReqCtx,
    ctx_user: UserInfo,
    res: String,
    status: String,
    err_msg: String,
    duration: Duration,
) -> Result<impl IntoResponse, String> {
    let path = &ctx.path;
    let cache = CacheManager::instance().await;
    let apipermiss = match cache
        .get_value::<ApiPermissionRes>(&format!("api:{}", path))
        .await
    {
        Ok(x) => x,
        Err(_) => return Err("err".into()),
    };
 
    let now = Local::now().naive_local();
    // 打印日志
    let req_data = ctx.clone();
    let res_data = res.clone();
    let err_msg_data = err_msg.clone();
    let duration_data = duration;
    let api_name = apipermiss.apiname;
    match apipermiss.logcache.as_str() {
        "1" => {
            file_log(req_data, now, duration_data, res_data, err_msg_data);
        }
        "2" => {
            match db_log(
                duration_data,
                ctx,
                ctx_user,
                now,
                api_name,
                res,
                status,
                err_msg,
            )
            .await
            {
                Ok(_) => {}
                Err(e) => {
                    tracing::info!("日志添加失败：{}", e.to_string());
                }
            };
        }
        "3" => {
            file_log(req_data, now, duration_data, res_data, err_msg_data);
            match db_log(
                duration_data,
                ctx,
                ctx_user,
                now,
                api_name,
                res,
                status,
                err_msg,
            )
            .await
            {
                Ok(_) => {}
                Err(e) => {
                    tracing::info!("日志添加失败：{}", e.to_string());
                }
            };
        }
        _ => return Ok(()),
    }
    Ok(())
}
fn file_log(
    req_data: ReqCtx,
    now: chrono::NaiveDateTime,
    duration_data: Duration,
    res_data: String,
    err_msg_data: String,
) {
    tracing::info!(
        "\n请求路径:{:?}\n完成时间:{:?}\n消耗时间:{:?}微秒 | {:?}毫秒\n请求数据:{:?}\n响应数据:{}\n错误信息:{:?}\n",
        req_data.path.clone(),
        now,
        duration_data.as_micros(),
        duration_data.as_millis(),
        req_data,
        res_data,
        err_msg_data,
    );
}

#[allow(clippy::too_many_arguments)]
async fn db_log(
    duration: Duration,
    ctx: ReqCtx,
    ctx_user: UserInfo,
    now: chrono::NaiveDateTime,
    api_name: String,
    res: String,
    status: String,
    err_msg: String,
) -> Result<impl IntoResponse, String> {
    let d = duration.as_micros() as i64;
    if let Ok(model) = s_sys_white_jwt::get_token(ctx_user.token_id).await {
        let operadd = SysOperLogAdd {
            api_name: Some(api_name),
            method: Some(ctx.path),
            request_method: Some(ctx.method),
            oper_name: Some(ctx_user.username),
            oper_url: Some(ctx.ori_uri),
            oper_ip: model.ipaddr,
            oper_location: model.login_location,
            oper_param: Some({
                let txt = ctx.path_params;
                if txt.len() > 2048 {
                    let truncated = &txt[..2000];
                    format!("{truncated} ...数据太长，不记录完整内容。")
                } else {
                    txt
                }
            }),
            json_result: Some(if res.len() > 2048 {
                let truncated = &res[..2000];
                format!("{truncated} ...数据太长，不记录完整内容。")
            } else {
                res
            }),
            status: Some(status),
            error_msg: Some(err_msg),
            oper_time: Some(now),
            cost_time: Some(d),
        };
        add_oper_log(operadd).await;
    }
    Ok(())
}
