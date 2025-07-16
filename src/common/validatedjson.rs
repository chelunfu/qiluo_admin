use crate::common::result::ApiResponse;
use async_trait::async_trait;
use axum::{
    extract::{FromRequest, Request},
    response::{IntoResponse, Response},
    Json,
};
use serde::de::DeserializeOwned;
use thiserror::Error;
use validator::Validate;

#[derive(Debug, Clone, Copy, Default)]
pub struct VJson<T>(pub T);

#[async_trait]
impl<T, S> FromRequest<S> for VJson<T>
where
    T: DeserializeOwned + Validate,
    S: Send + Sync,
{
    type Rejection = ServerError;
    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let Json(value) = Json::<T>::from_request(req, state).await?;
        value.validate()?;
        Ok(VJson(value))
    }
}

#[derive(Debug, Error)]
pub enum ServerError {
    #[error(transparent)]
    ValidationError(#[from] validator::ValidationErrors),

    #[error(transparent)]
    AxumJsonRejection(#[from] axum::extract::rejection::JsonRejection),

    #[error(transparent)]
    MissingJsonContentType(#[from] axum::extract::rejection::MissingJsonContentType),
}

impl IntoResponse for ServerError {
    fn into_response(self) -> Response {
        match self {
            ServerError::ValidationError(e) => {
                tracing::error!("{:?}", e); 
                ApiResponse::bad_request(e.to_string())
            }
            ServerError::AxumJsonRejection(e) => {
                tracing::error!("{:?}", e);
                ApiResponse::bad_request(e.to_string())
            }
            ServerError::MissingJsonContentType(e) => {
                tracing::error!("{:?}", e);
                ApiResponse::bad_request(e.to_string())
            }
        }
        .into_response()
    }
}
