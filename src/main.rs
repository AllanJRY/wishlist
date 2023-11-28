use std::{net::SocketAddr, sync::Arc};

use axum::{extract::FromRef, routing::get, Router, Server};
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

    let db = db::Db::new(
        "wishlist-db-prod:8000".to_string(),
        db_config.db_ns,
        db_config.db_name,
    );

    let app = Router::new()
        .route("/", get(|| async { "Hello world!" }))
        .route(
            "/login",
            get(SecurityController::login).post(SecurityController::handle_login),
        )
        .with_state(AppState { db }) // TODO: make with arc
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

#[derive(Debug, Clone, FromRef)]
pub struct AppState {
    pub db: db::Db,
}
