use super::jwt::UserInfo; 
use crate::service::sys::s_sys_role_api;
use axum::{
    body::{self, Body},
    extract::OriginalUri,
    extract::Request,
    http::{header, StatusCode},
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

const MAX_LOG_PREVIEW_BYTES: usize = 1000; // 预览最多打印的字节数
const MAX_BODY_SIZE_TO_LOG: u64 = 1 * 1024 * 1024; // 超过这个大小就不记录 body

fn is_text_like_ct(ct: &str) -> bool {
    let ct = ct.to_ascii_lowercase();
    if ct.starts_with("text/") {
        return true;
    }
    if ct == "application/json" || ct.ends_with("+json") {
        return true;
    }
    if ct == "application/xml" || ct.ends_with("+xml") {
        return true;
    }
    if ct == "application/x-www-form-urlencoded" {
        return true;
    }
    if ct == "application/javascript" {
        return true;
    }
    false
}

fn is_file_like_ct(ct: &str) -> bool {
    let ct = ct.to_ascii_lowercase();
    if ct.starts_with("multipart/form-data") {
        return true;
    }
    if ct.starts_with("application/octet-stream") {
        return true;
    }
    if ct.starts_with("image/") || ct.starts_with("audio/") || ct.starts_with("video/") {
        return true;
    }
    if ct.starts_with("application/pdf") {
        return true;
    }
    if ct.starts_with("application/zip")
        || ct.starts_with("application/x-7z-compressed")
        || ct.starts_with("application/x-rar-compressed")
        || ct.starts_with("application/x-tar")
    {
        return true;
    }
    false
}

fn lossy_preview(bytes: &[u8], max: usize) -> (String, bool) {
    let n = bytes.len().min(max);
    let s = String::from_utf8_lossy(&bytes[..n]).into_owned(); // 先按字节截，再 lossy 转
    (s, bytes.len() > max)
}

pub async fn request_log_fn_mid(
    req: Request<Body>,
    next: Next,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let (parts, body) = req.into_parts();

    let method = parts.method.clone();
    let uri = parts.uri.clone();
    let path = uri.path().to_string();
    let query = uri.query().unwrap_or("").to_string();

    let user_agent = parts
        .headers
        .get(header::USER_AGENT)
        .and_then(|h| h.to_str().ok())
        .unwrap_or("");

    let content_type = parts
        .headers
        .get(header::CONTENT_TYPE)
        .and_then(|h| h.to_str().ok())
        .unwrap_or("");

    let content_length = parts
        .headers
        .get(header::CONTENT_LENGTH)
        .and_then(|h| h.to_str().ok())
        .and_then(|s| s.parse::<u64>().ok());

    let skip_body = {
        let is_file = is_file_like_ct(content_type);
        let too_big = content_length
            .map(|cl| cl > MAX_BODY_SIZE_TO_LOG)
            .unwrap_or(false);
        !(is_text_like_ct(content_type) && !too_big) || is_file
    };

    if skip_body {
        info!(
            "http-request method:{} url:{} path:{} query:{} user_agent:{} content_type:{} content_length:{} body:<skipped>",
            method,
            uri,
            path,
            query,
            user_agent,
            content_type,
            content_length.unwrap_or(0),
        );
        let rebuilt = Request::from_parts(parts, body);
        return Ok(next.run(rebuilt).await);
    }

    let body_bytes = body::to_bytes(body, usize::MAX)
        .await
        .map_err(|e| (StatusCode::BAD_REQUEST, format!("读取请求体失败: {}", e)))?;

    let (preview, truncated) = lossy_preview(&body_bytes, MAX_LOG_PREVIEW_BYTES);

    info!(
        "http-request method:{} url:{} path:{} query:{} user_agent:{} content_type:{} body_size:{} body:{}{}",
        method,
        uri,
        path,
        query,
        user_agent,
        content_type,
        body_bytes.len(),
        preview,
        if truncated { "...(truncated)" } else { "" }
    );

    let rebuilt = Request::from_parts(parts, Body::from(body_bytes));
    Ok(next.run(rebuilt).await)
}
