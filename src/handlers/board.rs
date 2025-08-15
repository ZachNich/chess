use crate::{
    chess::helpers::get_all_moves,
    models::{board::Board, response::BoardWithMoves},
};
use axum::{Json, extract::State};
use hyper::StatusCode;
use std::sync::Arc;
use tokio::sync::Mutex;

pub async fn get_all_moves_handler(
    State(board): State<Arc<Mutex<Board>>>,
) -> Result<Json<BoardWithMoves>, StatusCode> {
    let board = board.lock().await;
    Ok(Json(BoardWithMoves {
        moves: get_all_moves(&board, board.turn_color),
        squares: board.squares.clone(),
    }))
}
