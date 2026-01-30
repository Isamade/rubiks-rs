use axum::{Json, response::IntoResponse};
use serde::{Deserialize, Serialize};

pub async fn hello_world() -> &'static str {
    "Hello, Rust World!"
}