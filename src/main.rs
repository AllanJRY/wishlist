use std::net::SocketAddr;

use axum::{routing::get, Router, Server};
use controller::SecurityController;
use tower_cookies::CookieManagerLayer;
use tower_http::services::ServeDir;

mod auth;
mod controller;
mod template;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let app = Router::new()
        .route("/", get(|| async { "Hello world!" }))
        .route(
            "/login",
            get(SecurityController::login).post(SecurityController::handle_login),
        )
        .layer(CookieManagerLayer::new())
        .nest_service("/assets", ServeDir::new("assets"));

    // todo: make the port configurable via env, which give the ability to use it
    // in the docker config aswell.
    let addr = SocketAddr::from(([0, 0, 0, 0], 7000));

    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
