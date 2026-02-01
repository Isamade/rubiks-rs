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

    if payload.usermove == "U" {
        updated_pieces = top_clockwise(&payload.cubeState.pieces);
    }

    Json(CubeState {
        pieces: updated_pieces,
    })
}

fn change_colors_top_clockwise(colors: &[String]) -> Vec<String> {
    let mut result = colors.to_vec();
    
    let temp = result[5].clone();
    result[5] = result[0].clone();
    result[0] = result[4].clone();
    result[4] = result[1].clone();
    result[1] = temp;

    result
}

pub fn top_clockwise(pieces: &[Piece]) -> Vec<Piece> {
    // Create a deep copy of the pieces
    let mut new_state = pieces.to_vec();

    // Build the locations matrix: [[8, 17, 26], [7, 16, 25], [6, 15, 24]]
    let mut locations_matrix = vec![vec![0; 3]; 3];
    for i in -3..0 {
        let i: i32 = i;
        for j in 1..4 {
            let j: i32 = j;
            // Rust indices are 0-based; Go logic translated: 9*j + i
            locations_matrix[(i+3) as usize][(j-1) as usize] = 9 * j + i;
        }
    }

    // Create the rotation matrix (Transpose then Swap Columns)
    let mut rotation_matrix = vec![vec![0; 3]; 3];
    for i in 0..3 {
        for j in 0..3 {
            rotation_matrix[i][j] = locations_matrix[j][i];
        }
    }
    for i in 0..3 {
        rotation_matrix[i].swap(0, 2);
    }

    // Update new_state based on rotation
    for i in 0..3 {
        for j in 0..3 {
            let target_val = locations_matrix[i][j];
            let mut new_i = 0;
            let mut new_j = 0;

            // Find coordinates in rotation_matrix
            'outer: for m in 0..3 {
                for n in 0..3 {
                    if rotation_matrix[m][n] == target_val {
                        new_i = m;
                        new_j = n;
                        break 'outer;
                    }
                }
            }

            let source_idx = locations_matrix[i][j] as usize;
            let target_idx = locations_matrix[new_i][new_j] as usize;

            new_state[target_idx] = Piece {
                colors: change_colors_top_clockwise(&pieces[source_idx].colors),
                position: pieces[target_idx].position.clone(),
            };
        }
    }

    new_state
}