use crate::models::{
    position::Positions,
    response::{AppState, SquaresAndMoves},
};
use axum::{
    Json,
    extract::{Path, State},
};
use hyper::StatusCode;

pub async fn move_piece_handler(
    Path((origin, destination)): Path<(Positions, Positions)>,
    State(AppState { board, bitboards }): State<AppState>,
) -> Result<Json<SquaresAndMoves>, StatusCode> {
    let mut locked_board = board.lock().await;
    let mut locked_bitboards = bitboards.lock().await;

    let (origin_idx, destination_idx) = (origin.to_index(), destination.to_index());
    locked_bitboards.move_piece(&mut locked_board, origin_idx, destination_idx);

    Ok(Json(SquaresAndMoves {
        moves: locked_bitboards.get_all_legal_moves(),
        squares: locked_board.squares.clone(),
    }))
}
