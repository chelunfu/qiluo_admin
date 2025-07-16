use crate::service::sys::s_sys_white_jwt;
use axum::{
    async_trait,
    extract::FromRequestParts,
    http::{request::Parts, StatusCode},
    response::{IntoResponse, Response},
    RequestPartsExt,
};
use axum_extra::{
    headers::{authorization::Bearer, Authorization},
    TypedHeader,
};
use chrono::Local;

use crate::common::ApiResponse;
use crate::config::APPCOFIG;
use jsonwebtoken::{
    decode, encode, errors::ErrorKind, DecodingKey, EncodingKey, Header, Validation,
};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};

use crate::service::sys::s_sys_user;

pub struct Keys {
    pub encoding: EncodingKey,
    pub decoding: DecodingKey,
}
pub static KEYS: Lazy<Keys> = Lazy::new(|| {
    let secret = &APPCOFIG.auth.jwt.secret;
    Keys::new(secret.as_bytes())
});
impl Keys {
    fn new(secret: &[u8]) -> Self {
        Self {
            encoding: EncodingKey::from_secret(secret),
            decoding: DecodingKey::from_secret(secret),
        }
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UserInfo {
    pub username: String,
    pub uid: i64,
    pub did: i64,
    pub rid: i64,
    pub token_id: i64,
}

#[derive(Debug, Deserialize, Clone)]
pub struct AuthPayload {
    pub uid: i64,
    pub did: i64,
    pub rid: i64,
    pub username: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub uid: i64,
    pub token_id: i64,
    pub username: String,
    pub exp: i64,
}

#[async_trait]
impl<S> FromRequestParts<S> for UserInfo
where
    S: Send + Sync,
{
    type Rejection = AuthError;
    /// 将用户信息注入request
    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let token_v = get_bear_token(parts).await?;
        // Decode the user data

        let token_data = match decode::<Claims>(&token_v, &KEYS.decoding, &Validation::default()) {
            Ok(token) => {
                let token_id = token.claims.token_id;
                let gtoken = s_sys_white_jwt::get_token(token_id).await;
                if gtoken.is_err() {
                    return Err(AuthError::CheckOutToken);
                }
                token
            }

            Err(err) => match *err.kind() {
                ErrorKind::InvalidToken => {
                    return Err(AuthError::InvalidToken);
                }
                ErrorKind::ExpiredSignature => {
                    return Err(AuthError::MissingCredentials);
                }
                _ => {
                    tracing::info!("AuthError:{:?}", err);
                    return Err(AuthError::TokenParseError);
                }
            },
        };
        let claims: Claims = token_data.claims;

        let userinfo = s_sys_user::get_user_info(claims.uid).await;

        let userinfo = match userinfo {
            Ok(user) => user,
            Err(err) => {
                tracing::info!("AuthError:{:?}", err);
                return Err(AuthError::NoneRole);
            }
        };
        tracing::info!(" userinfo.rid:{:?}", userinfo.rid);
        let user = UserInfo {
            username: claims.username,
            uid: claims.uid,
            did: userinfo.did,
            rid: userinfo.rid,
            token_id: claims.token_id,
        };
        parts.extensions.insert(user.clone());
        Ok(user)
    }
}

pub async fn get_bear_token(parts: &mut Parts) -> Result<String, AuthError> {
    // Extract the token from the authorization header
    let TypedHeader(Authorization(bearer)) = parts
        .extract::<TypedHeader<Authorization<Bearer>>>()
        .await
        .map_err(|_| AuthError::InvalidToken)?;
    // Decode the user data
    let bearer_data: &str = bearer.token();
    Ok(bearer_data.to_owned())
}

pub async fn authorize(payload: AuthPayload, token_id: i64) -> Result<AuthBody, AuthError> {
    if payload.username.is_empty() {
        return Err(AuthError::MissingCredentials);
    }
    let iat = Local::now();

    let exp = iat.timestamp() + APPCOFIG.auth.jwt.expiration;

    let claims = Claims {
        uid: payload.uid,
        token_id,
        username: payload.username,
        exp,
    };
    // Create the authorization token
    let token = encode(&Header::default(), &claims, &KEYS.encoding)
        .map_err(|_| AuthError::WrongCredentials)?;

    // Send the authorized token
    Ok(AuthBody::new(
        token,
        claims.exp,
        APPCOFIG.auth.jwt.expiration,
    ))
}

#[derive(Debug)]
pub enum AuthError {
    WrongCredentials,
    MissingCredentials,
    TokenCreation,
    InvalidToken,
    ExpiredToken,
    CheckOutToken,
    NoneRole,
    DatabaseError,
    TokenParseError,
}
impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AuthError::WrongCredentials => (StatusCode::UNAUTHORIZED, "Wrong credentials"),
            AuthError::MissingCredentials => (StatusCode::UNAUTHORIZED, "Missing credentials"),
            AuthError::TokenCreation => (StatusCode::INTERNAL_SERVER_ERROR, "Token creation error"),
            AuthError::InvalidToken => (StatusCode::UNAUTHORIZED, "Invalid token"),
            AuthError::ExpiredToken => (StatusCode::UNAUTHORIZED, "Token expired"),
            AuthError::CheckOutToken => {
                (StatusCode::UNAUTHORIZED, "The account has been logged out")
            }
            AuthError::NoneRole => (
                StatusCode::FORBIDDEN,
                "This account does not have permission",
            ),
            AuthError::DatabaseError => (StatusCode::INTERNAL_SERVER_ERROR, "Database error"),
            AuthError::TokenParseError => (StatusCode::UNAUTHORIZED, "Token parse error"),
        };
        ApiResponse::custom(status, error_message.to_owned())
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AuthBody {
    token: String,
    token_type: String,
    pub exp: i64,
    exp_in: i64,
}
impl AuthBody {
    fn new(access_token: String, exp: i64, exp_in: i64) -> Self {
        Self {
            token: access_token,
            token_type: "Bearer".to_string(),
            exp,
            exp_in,
        }
    }
}
