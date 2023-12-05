use askama::Template;

#[derive(Template)]
#[template(path = "signin.html")]
pub struct SigninTempl;

#[derive(Template)]
#[template(path = "signup.html")]
pub struct SignupTempl;
