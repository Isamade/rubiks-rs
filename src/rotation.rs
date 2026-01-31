use axum::{Json, response::IntoResponse};
use serde::{Deserialize, Serialize};

pub async fn hello_world() -> &'static str {
    "Hello, Rust World!"
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Piece {
    pub colors: Vec<String>,
    pub position: Vec<i32>,
}

#[derive(Serialize, Deserialize)]
pub struct CubeState {
    pub pieces: Vec<Piece>
}

#[derive(Deserialize)]
pub struct RotationRequest {
    pub usermove: String,
    pub cubeState: CubeState,
}

pub async fn rotate_cube(
    Json(payload): Json<RotationRequest>,
) -> impl IntoResponse {
    let mut updated_pieces = payload.cubeState.pieces.clone();

    Json(CubeState {
        pieces: updated_pieces,
    })
}