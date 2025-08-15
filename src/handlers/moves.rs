use crate::{
    chess::helpers::get_all_moves,
    models::response::{AppState, BoardWithMoves},
};
use axum::{
    Json,
    extract::{Path, State},
};
use hyper::StatusCode;

pub async fn move_piece_handler(
    Path((origin, destination)): Path<(u8, u8)>,
    State(AppState { board, bitboards }): State<AppState>,
) -> Result<Json<BoardWithMoves>, StatusCode> {
    let mut board = board.lock().await;
    board.move_piece(origin, destination);
    Ok(Json(BoardWithMoves {
        moves: get_all_moves(&board, board.turn_color),
        squares: board.squares.clone(),
    }))
}
