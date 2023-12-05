use std::sync::Arc;

use axum::{extract::FromRef, routing::get, Router};
use controller::SecurityController;
use tower_cookies::CookieManagerLayer;
use tower_http::services::ServeDir;

mod auth;
mod controller;
mod db;
mod template;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let db_config = envy::prefixed(db::SurrealConfig::PREFIX)
        .from_env::<db::SurrealConfig>()
        .unwrap();

    let db = db::Db::new(db_config.addr, db_config.db_ns, db_config.db_name);

    let app = Router::new()
        .route("/", get(|| async { "Hello world!" }))
        .route(
            "/signin",
            get(SecurityController::signin).post(SecurityController::handle_signin),
        )
        .route(
            "/signup",
            get(SecurityController::signup).post(SecurityController::handle_signup),
        )
        .with_state(Arc::new(AppState { db }))
        .layer(CookieManagerLayer::new())
        .nest_service("/assets", ServeDir::new("assets"));

    // todo: make the port configurable via env, which give the ability to use it
    // in the docker config aswell.
    let tcp_listener = tokio::net::TcpListener::bind("0.0.0.0:7000").await.unwrap();
    axum::serve(tcp_listener, app).await.unwrap();
}

#[derive(Debug, Clone, FromRef)]
pub struct AppState {
    pub db: db::Db,
}
