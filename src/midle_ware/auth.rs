use super::jwt::UserInfo; 
use crate::service::sys::s_sys_role_api;
use axum::{
    body::Body,
    extract::OriginalUri,
    extract::Request,
    http::StatusCode,
    middleware::Next,
    response::{IntoResponse, Response},
    Json,
};
use http_body_util::BodyExt;
use serde_json::json;
use tracing::info;
#[derive(Clone, Debug, Default)]
pub struct ReqCtx {
    pub ori_uri: String,
    pub path: String,
    pub path_params: String,
    pub method: String,
}

pub async fn auth_fn_mid(
    req: Request,
    next: Next,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let ori_uri_path = if let Some(path) = req.extensions().get::<OriginalUri>() {
        path.0.path().to_owned()
    } else {
        req.uri().path().to_owned()
    };
    let path = ori_uri_path.replacen("/api", "", 1);
    let method = req.method().to_string();
    let path_params = req.uri().query().unwrap_or("").to_string();

    let (parts, body) = req.into_parts();

    let bytes = body
        .collect()
        .await
        .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()).into_response())
        .unwrap()
        .to_bytes();

    let req_ctx: ReqCtx = ReqCtx {
        ori_uri: if path_params.is_empty() {
            ori_uri_path
        } else {
            format!("{}?{}", ori_uri_path, path_params)
        },
        path,
        path_params,
        method: method.to_string(),
    };

    let mut req = Request::from_parts(parts, Body::from(bytes));
    req.extensions_mut().insert(req_ctx);
    Ok(next.run(req).await)
}

pub async fn api_fn_mid(req: Request, next: Next) -> Result<Response, (StatusCode, String)> {
    let ctx = req.extensions().get::<ReqCtx>().expect("ReqCtx not found");

    let user = req
        .extensions()
        .get::<UserInfo>()
        .expect("UserInfo not found");
    let apiauth = s_sys_role_api::check_api_permission(user.rid, &ctx.path, &ctx.method).await;
    if apiauth {
        Ok(next.run(req).await)
    } else { 
        info!("没有API权限{:?} {}", user, apiauth);
        let body = Json(json!({
            "message": "没有API权限",
        }));
        Err((StatusCode::NOT_FOUND, body.to_string()))
    }
}

pub async fn request_log_fn_mid(
    req: Request,
    next: Next,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    // 先分离请求，避免借用冲突
    let (parts, body) = req.into_parts();

    // 从parts中提取信息
    let method = parts.method.to_string();
    let uri = parts.uri.clone();
    let path = uri.path();
    let query = uri.query().unwrap_or("");

    let user_agent = parts
        .headers
        .get(axum::http::header::USER_AGENT)
        .map_or("", |h| h.to_str().unwrap_or(""));

    let content_type = parts
        .headers
        .get(axum::http::header::CONTENT_TYPE)
        .map_or("", |h| h.to_str().unwrap_or(""));

    // 读取请求体
    let body_bytes = axum::body::to_bytes(body, usize::MAX)
        .await
        .map_err(|e| (StatusCode::BAD_REQUEST, format!("读取请求体失败: {}", e)))?;

    let body_content = String::from_utf8_lossy(&body_bytes);

    // 记录日志
    tracing::info!(
        "http-request method:{} url:{} path:{} query:{} user_agent:{} content_type:{} body_size:{} body:{}",
        method,
        uri,
        path,
        query,
        user_agent,
        content_type,
        body_bytes.len(),
        if body_content.len() > 1000 {
            format!("{}...(truncated)", &body_content[..1000])
        } else {
            body_content.to_string()
        }
    );

    // 重新构建请求
    let rebuilt_request = Request::from_parts(parts, Body::from(body_bytes));
     tracing::info!("auth end");
    Ok(next.run(rebuilt_request).await)
}
