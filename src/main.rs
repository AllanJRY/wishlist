use std::net::SocketAddr;

use axum::{routing::get, Router, Server};

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(|| async { "Hello, world!" }));

    // todo: make the port configurable via env, which give the ability to use it
    // in the docker config aswell.
    let addr = SocketAddr::from(([0, 0, 0, 0], 7000));

    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
