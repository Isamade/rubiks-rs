use axum::{routing::get, routing::post, Router};
use std::net::SocketAddr;

mod rotation;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(rotation::hello_world));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Server listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}