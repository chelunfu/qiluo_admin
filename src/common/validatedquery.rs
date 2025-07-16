use crate::common::result::ApiResponse;
use async_trait::async_trait;
use axum::{
    extract::{FromRequestParts, Query},
    http::request::Parts, 
    response::{IntoResponse, Response},
};
use serde::de::DeserializeOwned;
use thiserror::Error;
use validator::Validate;

#[derive(Debug, Clone, Copy, Default)]
pub struct VQuery<T>(pub T);

#[async_trait]
impl<T, S> FromRequestParts<S> for VQuery<T>
where
    T: DeserializeOwned + Validate,
    S: Send + Sync,
{
    type Rejection = ServerError;
    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let Query(value) = Query::<T>::from_request_parts(parts, _state).await?;
        value.validate()?;
        Ok(VQuery(value))
    }
}
axum_core::__impl_deref!(VQuery);

#[derive(Debug, Error)]
pub enum ServerError {
    #[error(transparent)]
    ValidationError(#[from] validator::ValidationErrors),

    #[error(transparent)]
    AxumQueryRejection(#[from] axum::extract::rejection::QueryRejection),
}

impl IntoResponse for ServerError {
    fn into_response(self) -> Response {
        match self {
            ServerError::ValidationError(e) => {
                tracing::error!("{:?}", e);
                ApiResponse::bad_request(e.to_string())
            }
            ServerError::AxumQueryRejection(e) => {
                tracing::error!("{:?}", e);
                ApiResponse::bad_request(e.to_string())
            }
        }
        .into_response()
    }
}
