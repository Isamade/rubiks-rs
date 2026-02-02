use axum::{Json, response::IntoResponse};
use serde::{Deserialize};
use rand::Rng;

use crate::rotation::{
    top_clockwise, top_counter_clockwise,
    bottom_clockwise, bottom_counter_clockwise,
    right_clockwise, right_counter_clockwise,
    left_clockwise, left_counter_clockwise,
    front_clockwise, front_counter_clockwise,
    back_clockwise, back_counter_clockwise,
    CubeState
};

#[derive(Deserialize)]
pub struct ScrambleRequest {
    pub movesCount: usize,
    pub cubeState: CubeState,
}

pub async fn scramble_cube(
    Json(payload): Json<ScrambleRequest>,
) -> impl IntoResponse {

    // 1. Create a deep copy using clone (requires #[derive(Clone)] on Piece)
    let mut new_state = payload.cubeState.pieces.to_vec();

    let possible_moves = [
        "U", "U'", "D", "D'", "R", "R'", 
        "L", "L'", "F", "F'", "B", "B'"
    ];

    let mut rng = rand::rng();

    for _ in 0..payload.movesCount {
        // 2. Generate a random index 0..12
        let index = rng.random_range(0..12);
        
        // 3. Match replaces the Go switch statement
        new_state = match possible_moves[index] {
            "U"  => top_clockwise(&new_state),
            "U'" => top_counter_clockwise(&new_state),
            "D"  => bottom_clockwise(&new_state),
            "D'" => bottom_counter_clockwise(&new_state),
            "R"  => right_clockwise(&new_state),
            "R'" => right_counter_clockwise(&new_state),
            "L"  => left_clockwise(&new_state),
            "L'" => left_counter_clockwise(&new_state),
            "F"  => front_clockwise(&new_state),
            "F'" => front_counter_clockwise(&new_state),
            "B"  => back_clockwise(&new_state),
            "B'" => back_counter_clockwise(&new_state),
            _    => new_state, // Exhaustive match requirement
        };
    }

    Json(CubeState {
        pieces: new_state,
    })
}