use axum::{routing::get, routing::post, Router};
use std::net::SocketAddr;
use tower_http::cors::{Any, CorsLayer};
use axum::http::Method;

mod rotation;

#[tokio::main]
async fn main() {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([Method::GET, Method::POST])
        .allow_headers(Any);

    let app = Router::new()
        .route("/", get(rotation::hello_world));
        .route("/rotate", post(rotation::rotate_cube))
        .layer(cors);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Server listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}