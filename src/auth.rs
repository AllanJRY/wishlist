use std::sync::Arc;

use axum::{
    async_trait,
    extract::FromRequestParts,
    http::{request::Parts, StatusCode},
    response::{IntoResponse, Response},
};
use serde::{Deserialize, Serialize};
use tower_cookies::Cookies;

use crate::{db::Db, AppState};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignupCredentials {
    // TODO: make a tuple struct Email with custom Serialize and Deserialize behaviours.
    // also add validation to ensure a valid email.
    pub username: String,
    pub email: String,
    pub pwd: String,
    pub confirm_pwd: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SigninCredentials {
    // TODO: make a tuple struct Email with custom Serialize and Deserialize behaviours.
    // also add validation to ensure a valid email.
    pub email: String,
    pub pwd: String,
}
/// The application store the Json Web Token inside a cookie due to the fact that
/// the application use template on backend side with Askama.
/// This constant is the name of the cookie set on the client which contain the access token.
pub const ACCESS_TOKEN_COOKIE: &str = "access_token";

#[derive(Debug, Clone, Deserialize)]
pub struct AuthenticatedUser {
    email: String,
}

// TODO: Arc<AppState>
#[async_trait]
impl FromRequestParts<Arc<AppState>> for AuthenticatedUser {
    type Rejection = AuthError;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &Arc<AppState>,
    ) -> Result<Self, Self::Rejection> {
        let cookies = Cookies::from_request_parts(parts, state).await.unwrap();
        if let Some(access_token_cookie) = cookies.get(ACCESS_TOKEN_COOKIE) {
            let db = state.db.connect_scope(access_token_cookie.value()).await;
            let user: Option<AuthenticatedUser> = db
                .query("SELECT * FROM $auth;")
                .await
                .unwrap()
                .take(0)
                .unwrap();

            dbg!(&user);

            match user {
                Some(authenticated_user) => Ok(authenticated_user),
                None => Err(AuthError::InvalidToken), // todo no user
            }
        } else {
            return Err(AuthError::MissingToken);
        }
    }
}

#[async_trait]
impl FromRequestParts<Db> for AuthenticatedUser {
    type Rejection = AuthError;

    async fn from_request_parts(parts: &mut Parts, db: &Db) -> Result<Self, Self::Rejection> {
        let cookies = Cookies::from_request_parts(parts, db).await.unwrap();
        if let Some(access_token_cookie) = cookies.get(ACCESS_TOKEN_COOKIE) {
            let db = db.connect_scope(access_token_cookie.value()).await;
            let user: Option<AuthenticatedUser> = db
                .query("SELECT user:$auth.id FROM user")
                .await
                .unwrap()
                .take(0)
                .unwrap();

            match user {
                Some(authenticated_user) => Ok(authenticated_user),
                None => Err(AuthError::InvalidToken), // todo no user
            }
        } else {
            return Err(AuthError::MissingToken);
        }
    }
}

#[derive(Debug, Clone)]
pub enum AuthError {
    InvalidCredentials,
    MissingCredentials,
    TokenCreation,
    InvalidToken,
    MissingToken,
    SignupFailed,
}

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AuthError::InvalidCredentials => (StatusCode::UNAUTHORIZED, "Wrong credentials"),
            AuthError::MissingCredentials => (StatusCode::BAD_REQUEST, "Missing credentials"),
            AuthError::TokenCreation => (StatusCode::INTERNAL_SERVER_ERROR, "Token creation error"),
            AuthError::InvalidToken => (StatusCode::BAD_REQUEST, "Invalid access token"),
            AuthError::MissingToken => (StatusCode::UNAUTHORIZED, "Missing access token"),
            AuthError::SignupFailed => (StatusCode::INTERNAL_SERVER_ERROR, "Signup failed"),
        };

        // TODO: template as body
        // let body = Json(json!({
        //    "error": error_message,
        //}));

        (status, error_message).into_response()
    }
}

impl core::fmt::Display for AuthError {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
        write!(fmt, "{self:?}")
    }
}

impl std::error::Error for AuthError {}
