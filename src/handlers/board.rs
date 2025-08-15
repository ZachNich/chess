use crate::{
    chess::helpers::get_all_moves,
    models::response::{AppState, BoardWithMoves},
};
use axum::{Json, extract::State};
use hyper::StatusCode;

pub async fn get_all_moves_handler(
    State(AppState { board, bitboards }): State<AppState>,
) -> Result<Json<BoardWithMoves>, StatusCode> {
    let board = board.lock().await;
    Ok(Json(BoardWithMoves {
        moves: get_all_moves(&board, board.turn_color),
        squares: board.squares.clone(),
    }))
}
