use super::error::{Error, Result};
use axum::response::{IntoResponse, Response};
use hyper::StatusCode;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub struct RespDataString(pub String);

// 空数据类型，用于没有业务数据的响应
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EmptyData {}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ApiResponse<T> {
    pub message: String, // 必填
    pub data: T,         // 必填，但可以是 EmptyData
}

impl<T> ApiResponse<T>
where
    T: Serialize,
{
    // 基础构造方法
    pub fn new(message: String, data: T) -> Self {
        Self { message, data }
    }

    // 成功响应 (200) - 使用默认消息
    pub fn ok(data: T) -> Response {
        (StatusCode::OK, Self::new("操作成功".to_string(), data)).into_response()
    }

    // 成功响应带消息 (200)
    pub fn ok_with_msg(data: T, message: impl Into<String>) -> Response {
        (StatusCode::OK, Self::new(message.into(), data)).into_response()
    }

    // 创建成功 (201)
    pub fn created(data: T) -> Response {
        (StatusCode::CREATED, Self::new("创建成功".to_string(), data)).into_response()
    }

    // 创建成功带消息 (201)
    pub fn created_with_msg(data: T, message: impl Into<String>) -> Response {
        (StatusCode::CREATED, Self::new(message.into(), data)).into_response()
    }

    // 从 Result 转换
    pub fn from_result(result: Result<T>) -> Response
    where
        T: Serialize,
    {
        match result {
            Ok(data) => Self::ok(data),
            Err(err) => {
                let (status, message) = match &err {
                    Error::Message(msg) => {
                        let status = if msg.contains("not found") {
                            StatusCode::NOT_FOUND
                        } else if msg.contains("unauthorized") {
                            StatusCode::UNAUTHORIZED
                        } else if msg.contains("forbidden") {
                            StatusCode::FORBIDDEN
                        } else if msg.contains("validation") {
                            StatusCode::UNPROCESSABLE_ENTITY
                        } else {
                            StatusCode::INTERNAL_SERVER_ERROR
                        };
                        (status, msg.clone())
                    }
                    Error::WithStatus(status, msg) => (*status, msg.clone()),
                };

                // 注意：这里直接使用泛型方法，不需要特殊的 impl ApiResponse<()>
                Self::error_response(status, message)
            }
        }
    }

    // 统一的错误响应方法 - 返回 ApiResponse<EmptyData>
    fn error_response(status: StatusCode, message: String) -> Response {
        let api_response = ApiResponse::<EmptyData> {
            message,
            data: EmptyData {},
        };
        (status, api_response).into_response()
    }
}
impl ApiResponse<EmptyData> {
    // 自定义状态码
    pub fn custom(status: StatusCode, message: String) -> Response {
        let api_response = ApiResponse::<EmptyData> {
            message,
            data: EmptyData {},
        };
        (status, api_response).into_response()
    }


    // 未授权 (401)
    pub fn unauthorized(message: impl Into<String>) -> Response {
        Self::error_response(StatusCode::UNAUTHORIZED, message.into())
    }

    // 禁止访问 (403)
    pub fn forbidden(message: impl Into<String>) -> Response {
        Self::error_response(StatusCode::FORBIDDEN, message.into())
    }

    // 未找到 (404)
    pub fn not_found(message: impl Into<String>) -> Response {
        Self::error_response(StatusCode::NOT_FOUND, message.into())
    }

    // 冲突 (409)
    pub fn conflict(message: impl Into<String>) -> Response {
        Self::error_response(StatusCode::CONFLICT, message.into())
    }

    // 参数验证失败 (422)
    pub fn unprocessable_entity(message: impl Into<String>) -> Response {
        Self::error_response(StatusCode::UNPROCESSABLE_ENTITY, message.into())
    }

    // 服务器错误 (500)
    pub fn internal_server_error(message: impl Into<String>) -> Response {
        Self::error_response(StatusCode::INTERNAL_SERVER_ERROR, message.into())
    }

    // 服务不可用 (503)
    pub fn service_unavailable(message: impl Into<String>) -> Response {
        Self::error_response(StatusCode::SERVICE_UNAVAILABLE, message.into())
    }
    // 操作成功但无数据返回
    pub fn success_no_data(message: impl Into<String>) -> Response {
        let api_response = ApiResponse::<EmptyData> {
            message: message.into(),
            data: EmptyData {},
        };
        (StatusCode::OK, api_response).into_response()
    }
    // 删除成功 - 返回确认消息
    pub fn deleted(message: impl Into<String>) -> Response {
        let api_response = ApiResponse::<EmptyData> {
            message: message.into(),
            data: EmptyData {},
        };
        (StatusCode::OK, api_response).into_response()
    }
    // 客户端错误 (400)
    pub fn bad_request(message: impl Into<String>) -> Response {
        Self::error_response(StatusCode::BAD_REQUEST, message.into())
    }
}
impl<T> IntoResponse for ApiResponse<T>
where
    T: Serialize,
{
    fn into_response(self) -> Response {
        match serde_json::to_string(&self) {
            Ok(json) => (
                [
                    ("Content-Type", "application/json;charset=UTF-8"),
                    ("Access-Control-Allow-Origin", "*"),
                    ("Cache-Control", "no-cache"),
                ],
                json,
            )
                .into_response(),
            Err(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                [("Content-Type", "application/json;charset=UTF-8")],
                r#"{"message":"序列化响应失败","data":{}}"#,
            )
                .into_response(),
        }
    }
}
#[allow(unused_macros)]
// 宏定义保持不变
macro_rules! api_ok {
    ($data:expr) => {
        ApiResponse::ok($data)
    };
    ($data:expr, $msg:expr) => {
        ApiResponse::ok_with_msg($data, $msg)
    };
}
#[allow(unused_macros)]
macro_rules! api_error {
    (400, $msg:expr) => {
        ApiResponse::bad_request($msg)
    };
    (401, $msg:expr) => {
        ApiResponse::unauthorized($msg)
    };
    (403, $msg:expr) => {
        ApiResponse::forbidden($msg)
    };
    (404, $msg:expr) => {
        ApiResponse::not_found($msg)
    };
    (409, $msg:expr) => {
        ApiResponse::conflict($msg)
    };
    (422, $msg:expr) => {
        ApiResponse::unprocessable_entity($msg)
    };
    (500, $msg:expr) => {
        ApiResponse::internal_server_error($msg)
    };
    (503, $msg:expr) => {
        ApiResponse::service_unavailable($msg)
    };
}
#[allow(unused_macros)]
macro_rules! api_created {
    ($data:expr) => {
        ApiResponse::created($data)
    };
    ($data:expr, $msg:expr) => {
        ApiResponse::created_with_msg($data, $msg)
    };
}
#[allow(unused_macros)]
macro_rules! api_deleted {
    ($msg:expr) => {
        ApiResponse::deleted($msg)
    };
}
