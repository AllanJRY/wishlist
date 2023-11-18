use axum::{
    http::StatusCode,
    response::{IntoResponse, Redirect},
    Form,
};

use crate::auth::{Claims, LoginCredentials};

pub struct SecurityController;

impl SecurityController {
    pub async fn login() {}

    pub async fn handle_login(
        claims: Option<Claims>,
        Form(login_credentials): Form<LoginCredentials>,
    ) -> impl IntoResponse {
        if claims.is_some() {
            return Redirect::to("/");
        }

        if login_credentials.login.as_str() == "allan.jarry@gmail.com"
            && login_credentials.pwd == "pwd"
        {
            todo!("set cookie")
        } else {
            Redirect::to("/login")
        }
    }
}
