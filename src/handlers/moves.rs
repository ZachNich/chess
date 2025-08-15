use crate::{
    chess::helpers::get_all_moves,
    models::{board::Board, response::BoardWithMoves},
};
use axum::{
    Json,
    extract::{Path, State},
};
use hyper::StatusCode;
use std::sync::Arc;
use tokio::sync::Mutex;

pub async fn move_piece_handler(
    Path((origin, destination)): Path<(u8, u8)>,
    State(board): State<Arc<Mutex<Board>>>,
) -> Result<Json<BoardWithMoves>, StatusCode> {
    let mut board = board.lock().await;
    board.move_piece(origin, destination);
    Ok(Json(BoardWithMoves {
        moves: get_all_moves(&board, board.turn_color),
        squares: board.squares.clone(),
    }))
}
