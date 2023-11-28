use axum::{
    debug_handler,
    extract::State,
    response::{IntoResponse, Redirect},
    Form,
};
use tower_cookies::{Cookie, Cookies};

use crate::{
    auth::{AuthenticatedUser, SigninCredentials, SignupCredentials, ACCESS_TOKEN_COOKIE},
    template::LoginTempl,
};

pub struct SecurityController;

impl SecurityController {
    pub async fn signup(auth_user: Option<AuthenticatedUser>) -> Result<LoginTempl, Redirect> {
        if auth_user.is_some() {
            return Err(Redirect::to("/"));
        }

        Ok(LoginTempl)
    }

    pub async fn handle_signup(
        auth_user: Option<AuthenticatedUser>,
        State(db): State<crate::db::Db>,
        Form(signup_credentials): Form<SignupCredentials>,
    ) -> Result<impl IntoResponse, crate::auth::AuthError> {
        if auth_user.is_some() {
            return Ok(Redirect::to("/"));
        }

        let _ = db.signup(signup_credentials).await?;

        Ok(Redirect::to("/login"))
    }

    pub async fn login(claims: Option<AuthenticatedUser>) -> Result<LoginTempl, Redirect> {
        if claims.is_some() {
            return Err(Redirect::to("/"));
        }

        Ok(LoginTempl)
    }

    pub async fn handle_login(
        auth_user: Option<AuthenticatedUser>,
        cookies: Cookies,
        State(db): State<crate::db::Db>,
        Form(signin_credentials): Form<SigninCredentials>,
    ) -> impl IntoResponse {
        if auth_user.is_some() {
            return Redirect::to("/");
        }

        match db.signin(signin_credentials).await {
            Ok(access_token) => {
                cookies.add(Cookie::new(
                    ACCESS_TOKEN_COOKIE,
                    access_token.into_insecure_token(),
                ));
                Redirect::to("/")
            }
            Err(_) => Redirect::to("/login"),
        }
    }
}
