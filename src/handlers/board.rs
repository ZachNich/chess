use crate::models::response::{AppState, SquaresAndMoves};
use axum::{Json, extract::State};
use hyper::StatusCode;

pub async fn get_all_moves_handler(
    State(AppState { board, bitboards }): State<AppState>,
) -> Result<Json<SquaresAndMoves>, StatusCode> {
    let locked_board = board.lock().await;
    let mut locked_bitboards = bitboards.lock().await;

    Ok(Json(SquaresAndMoves {
        moves: locked_bitboards.get_all_legal_moves(),
        squares: locked_board.squares.clone(),
    }))
}
