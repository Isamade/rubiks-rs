use axum::{routing::get, routing::post, Router};
use std::net::SocketAddr;
use tower_http::cors::{Any, CorsLayer}; // Import CORS types
use axum::http::Method;

mod rotation;
mod scramble;

#[tokio::main]
async fn main() {
    // Define CORS policy
    let cors = CorsLayer::new()
        // Allow requests from any origin - use .allow_origin(["http://localhost:5173".parse().unwrap()]) for production
        .allow_origin(Any)
        // Allow specific HTTP methods
        .allow_methods([Method::GET, Method::POST])
        // Allow specific headers (like Content-Type)
        .allow_headers(Any);

    let app = Router::new()
        .route("/", get(rotation::hello_world))
        .route("/rotate", post(rotation::rotate_cube))
        .route("/scramble", post(scramble::scramble_cube))
        .layer(cors);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Server listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

