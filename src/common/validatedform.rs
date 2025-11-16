use crate::common::result::ApiResponse;
use async_trait::async_trait;
use axum::{
    extract::{Form, FromRequest, Request},
    response::{IntoResponse, Response},
};
use serde::de::DeserializeOwned;
use thiserror::Error;
use validator::Validate;

#[derive(Debug, Clone, Copy, Default)]
pub struct VForm<T>(pub T);

 
impl<T, S> FromRequest<S> for VForm<T>
where
    T: DeserializeOwned + Validate,
    S: Send + Sync,
{
    type Rejection = ServerError;
    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let Form(value) = Form::<T>::from_request(req, state).await?;
        value.validate()?;
        Ok(VForm(value))
    }
}

#[derive(Debug, Error)]
pub enum ServerError {
    #[error(transparent)]
    ValidationError(#[from] validator::ValidationErrors),

    #[error(transparent)]
    AxumFormRejection(#[from] axum::extract::rejection::FormRejection),
}

impl IntoResponse for ServerError {
    fn into_response(self) -> Response {
        match self {
            ServerError::ValidationError(e) => {
                tracing::error!("{:?}", e);
                ApiResponse::bad_request(e.to_string())
            }
            ServerError::AxumFormRejection(e) => {
                tracing::error!("{:?}", e);
                ApiResponse::bad_request(e.to_string())
            }
        }
        .into_response()
    }
}
