use axum::{
    response::{IntoResponse, Redirect},
    Form,
};
use tower_cookies::{Cookie, Cookies};

use crate::{
    auth::{encode_access_token, make_user_claims, Claims, LoginCredentials, ACCESS_TOKEN_COOKIE},
    template::LoginTempl,
};

pub struct SecurityController;

impl SecurityController {
    pub async fn login(claims: Option<Claims>) -> Result<LoginTempl, Redirect> {
        if claims.is_some() {
            return Err(Redirect::to("/"));
        }

        Ok(LoginTempl)
    }

    pub async fn handle_login(
        claims: Option<Claims>,
        cookies: Cookies,
        Form(login_credentials): Form<LoginCredentials>,
    ) -> Result<impl IntoResponse, crate::auth::Error> {
        println!("{:?}", claims);

        if claims.is_some() {
            return Ok(Redirect::to("/"));
        }

        // TODO: check in db, and might be better to move this logic in auth module.
        if login_credentials.login.as_str() == "allan.jarry@gmail.com"
            && login_credentials.pwd == "pwd"
        {
            let access_token = encode_access_token(make_user_claims("allan.jarry@gmail.com"))?;
            cookies.add(Cookie::new(ACCESS_TOKEN_COOKIE, access_token));
            Ok(Redirect::to("/"))
        } else {
            Ok(Redirect::to("/login"))
        }
    }
}
