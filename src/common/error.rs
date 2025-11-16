use super::result::ApiResponse;
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::de::Visitor;
use serde::ser::{Serialize, Serializer};
use serde::{Deserialize, Deserializer};
use std::error::Error as StdError;
use std::fmt::{self, Debug, Display}; 
use std::sync::PoisonError;
use std::time::{Duration, Instant};
pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
#[non_exhaustive]
pub enum Error {
    Message(String),
    WithStatus(StatusCode, String), // 新增：带状态码的错误
}

impl Error {
    pub fn bad_request(msg: impl Into<String>) -> Self {
        Self::WithStatus(StatusCode::BAD_REQUEST, msg.into())
    }

    pub fn unauthorized(msg: impl Into<String>) -> Self {
        Self::WithStatus(StatusCode::UNAUTHORIZED, msg.into())
    }

    pub fn forbidden(msg: impl Into<String>) -> Self {
        Self::WithStatus(StatusCode::FORBIDDEN, msg.into())
    }

    pub fn not_found(msg: impl Into<String>) -> Self {
        Self::WithStatus(StatusCode::NOT_FOUND, msg.into())
    }

    pub fn conflict(msg: impl Into<String>) -> Self {
        Self::WithStatus(StatusCode::CONFLICT, msg.into())
    }

    pub fn validation_error(msg: impl Into<String>) -> Self {
        Self::WithStatus(StatusCode::UNPROCESSABLE_ENTITY, msg.into())
    }

    pub fn internal_error(msg: impl Into<String>) -> Self {
        Self::WithStatus(StatusCode::INTERNAL_SERVER_ERROR, msg.into())
    }
}

impl Display for Error {
    // 实现fmt::Display trait的fmt方法，用于格式化输出Error类型
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // 使用match语句匹配Error类型
        match self {
            // 如果Error类型为Message，则输出error
            Error::Message(error) => write!(f, "{}", error),
            Error::WithStatus(_, error) => write!(f, "{}", error),
        }
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        match self {
            Error::Message(msg) => ApiResponse::internal_server_error(msg).into_response(),
            Error::WithStatus(status, msg) => ApiResponse::custom(status, msg).into_response(),
        }
    }
}
// 错误转换宏
macro_rules! impl_error_from {
    ($($error_type:ty),* $(,)?) => {
        $(
            impl From<$error_type> for Error {
                fn from(err: $error_type) -> Self {
                    Error::Message(err.to_string())
                }
            }
        )*
    };
}

// 标准错误类型转换
impl_error_from!(
    std::io::Error,
    base64::DecodeError,
    axum::Error,
    axum::extract::multipart::MultipartError,
    sea_orm::DbErr,
    tera::Error,
    lettre::address::AddressError,
    lettre::transport::smtp::Error,
    serde_json::Error,
    reqwest::Error, 
    cron_clock::error::Error,
    std::num::ParseIntError,
    std::num::ParseFloatError,
    std::str::Utf8Error,
    bb8_redis::redis::RedisError, 
    bb8_redis::bb8::RunError<bb8_redis::redis::RedisError>
);

// 保留泛型PoisonError转换（因为它有泛型参数）
impl<T> From<PoisonError<T>> for Error {
    fn from(arg: PoisonError<T>) -> Self {
        Error::Message(arg.to_string())
    }
}

// 保留基础类型转换（这些不是std::error::Error类型）
impl From<&str> for Error {
    fn from(arg: &str) -> Self {
        Error::Message(arg.to_string())
    }
}

impl From<String> for Error {
    fn from(arg: String) -> Self {
        Error::Message(arg)
    }
}

impl From<()> for Error {
    fn from(_: ()) -> Self {
        Error::Message("no found".to_owned())
    }
}

// 保留特殊的自定义转换
impl From<(Instant, Duration)> for Error {
    fn from(_: (Instant, Duration)) -> Self {
        Error::Message("Instant And Duration ".to_string())
    }
}

// impl Display for Error {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         match self {
//             Error::Message(error) => write!(f, "{}", error),
//         }
//     }
// }

// impl std::convert::From<Error> for String {
//     fn from(val: Error) -> Self {
//         match val {
//             Error::Message(msg) => msg,
//         }
//     }
// }

impl StdError for Error {}

impl Clone for Error {
    fn clone(&self) -> Self {
        Error::from(self.to_string())
    }

    fn clone_from(&mut self, source: &Self) {
        *self = Self::from(source.to_string());
    }
}

impl Serialize for Error {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.to_string().as_str())
    }
}

struct ErrorVisitor;

impl<'de> Visitor<'de> for ErrorVisitor {
    type Value = String;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a string")
    }

    fn visit_string<E>(self, v: String) -> std::result::Result<Self::Value, E>
    where
        E: std::error::Error,
    {
        Ok(v)
    }

    fn visit_str<E>(self, v: &str) -> std::result::Result<Self::Value, E>
    where
        E: std::error::Error,
    {
        Ok(v.to_string())
    }
}

impl<'de> Deserialize<'de> for Error {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let r = deserializer.deserialize_string(ErrorVisitor)?;
        Ok(Error::from(r))
    }
}

impl From<Box<dyn std::error::Error + Send + Sync>> for Error {
    fn from(arg: Box<dyn std::error::Error + Send + Sync>) -> Self {
        Error::Message(arg.to_string())
    }
}
