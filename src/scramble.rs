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

    let (rotation_sequence, new_state) = scramble(&payload.cubeState.pieces, payload.movesCount);

    Json(CubeState {
        pieces: new_state,
    })
}


pub fn scramble(pieces: &[Piece], moves_count: usize) -> (Vec<String>, Vec<Piece>) {
    // Create a deep copy of pieces using Rust's built-in cloning
    let mut new_state = pieces.to_vec();

    let possible_moves = [
        "U", "U'", "D", "D'", "R", "R'", "L", "L'", "F", "F'", "B", "B'",
    ];

    let mut rotation_sequence = Vec::with_capacity(moves_count);
    let mut rng = rand::rng();

    for _ in 0..moves_count {
        let index = rng.random_range(0..possible_moves.len());
        let selected_move = possible_moves[index];
        rotation_sequence.push(selected_move.to_string());

        // Update state based on the selected move
        new_state = match selected_move {
            "U"  => top_clockwise(new_state),
            "U'" => top_counter_clockwise(new_state),
            "D"  => bottom_clockwise(new_state),
            "D'" => bottom_counter_clockwise(new_state),
            "R"  => right_clockwise(new_state),
            "R'" => right_counter_clockwise(new_state),
            "L"  => left_clockwise(new_state),
            "L'" => left_counter_clockwise(new_state),
            "F"  => front_clockwise(new_state),
            "F'" => front_counter_clockwise(new_state),
            "B"  => back_clockwise(new_state),
            "B'" => back_counter_clockwise(new_state),
            _    => new_state, // Should not happen given the array contents
        };
    }

    (rotation_sequence, new_state)
}

