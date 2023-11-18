use axum::{
    async_trait,
    extract::FromRequestParts,
    http::{request::Parts, StatusCode},
    response::{IntoResponse, Response},
};
use jsonwebtoken::{decode, DecodingKey, EncodingKey, Validation};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use tower_cookies::Cookies;

/// The application store the Json Web Token inside a cookie due to the fact that
/// the application use template on backend side with Askama.
/// This constant is the name of the cookie set on the client which contain the access token.
const ACCESS_TOKEN_COOKIE: &str = "access_token";

static JWT_ENCODE_KEY: Lazy<JWTEncodeKeys> = Lazy::new(|| {
    JWTEncodeKeys::new(
        std::env::var("JWT_SECRET")
            .expect("JWT_SECRET must be set.")
            .as_bytes(),
    )
});

pub fn login(credentials: LoginCredentials) {
    todo!()
}

#[derive(Debug, Clone, Deserialize)]
pub struct LoginCredentials {
    // TODO: make a tuple struct Email with custom Serialize and Deserialize behaviours.
    // also add validation to ensure a valid email.
    pub login: String,
    pub pwd: String,
}

struct JWTEncodeKeys {
    encoding: EncodingKey,
    decoding: DecodingKey,
}

impl JWTEncodeKeys {
    fn new(secret: &[u8]) -> Self {
        Self {
            encoding: EncodingKey::from_secret(secret),
            decoding: DecodingKey::from_secret(secret),
        }
    }
}

/// Define the claims present in the Json Web Token payload according to the
/// specifications.
/// This application use JWT in the authentication process.
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    // Subject (whom token refers to)
    sub: String,

    /// Required (validate_exp defaults to true in validation). Expiration time (as UTC timestamp)
    exp: usize,

    /// Issued at (as UTC timestamp)
    iat: usize,

    /// Issuer
    iss: String,

    /// Audience
    aud: Option<String>,

    // Not Before (as UTC timestamp)
    nbf: Option<usize>,
}

#[async_trait]
impl<S> FromRequestParts<S> for Claims
where
    S: Send + Sync,
{
    type Rejection = Error;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let cookies = Cookies::from_request_parts(parts, state).await.unwrap();
        if let Some(access_token_cookie) = cookies.get(ACCESS_TOKEN_COOKIE) {
            let claims = decode::<Claims>(
                access_token_cookie.value(),
                &JWT_ENCODE_KEY.decoding,
                &Validation::default(),
            )
            .map_err(|_| Error::InvalidToken)?
            .claims;

            return Ok(claims);
        } else {
            return Err(Error::MissingToken);
        }
    }
}

#[derive(Debug, Clone)]
pub enum Error {
    WrongCredentials,
    MissingCredentials,
    TokenCreation,
    InvalidToken,
    MissingToken,
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            Error::WrongCredentials => (StatusCode::UNAUTHORIZED, "Wrong credentials"),
            Error::MissingCredentials => (StatusCode::BAD_REQUEST, "Missing credentials"),
            Error::TokenCreation => (StatusCode::INTERNAL_SERVER_ERROR, "Token creation error"),
            Error::InvalidToken => (StatusCode::BAD_REQUEST, "Invalid access token"),
            Error::MissingToken => (StatusCode::UNAUTHORIZED, "Missing access token"),
        };

        // TODO: template as body
        // let body = Json(json!({
        //    "error": error_message,
        //}));

        (status, error_message).into_response()
    }
}

impl core::fmt::Display for Error {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
        write!(fmt, "{self:?}")
    }
}

impl std::error::Error for Error {}
