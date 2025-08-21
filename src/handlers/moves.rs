use crate::models::response::{AppState, MoveParams, SquaresAndMoves};
use axum::{Json, debug_handler, extract::State};
use hyper::StatusCode;

#[debug_handler]
pub async fn move_piece_handler(
    State(AppState { board, bitboards }): State<AppState>,
    Json(MoveParams {
        promotion,
        origin,
        destination,
    }): Json<MoveParams>,
) -> Result<Json<SquaresAndMoves>, StatusCode> {
    let mut locked_board = board.lock().await;
    let mut locked_bitboards = bitboards.lock().await;
    let (origin_idx, destination_idx) = (origin.to_index(), destination.to_index());

    if let Some(promotion) = promotion {
        match locked_bitboards.promote_pawn(
            &mut locked_board,
            origin_idx,
            destination_idx,
            promotion,
        ) {
            Ok(_) => {
                locked_board.toggle_turn_color();

                return Ok(Json(SquaresAndMoves {
                    moves: locked_bitboards.get_all_legal_moves(&mut locked_board),
                    squares: locked_board.squares.clone(),
                }));
            }
            Err(status_code) => return Err(status_code),
        }
    }

    match locked_bitboards.move_piece(&mut locked_board, origin_idx, destination_idx) {
        Ok(_) => {
            locked_board.toggle_turn_color();

            return Ok(Json(SquaresAndMoves {
                moves: locked_bitboards.get_all_legal_moves(&mut locked_board),
                squares: locked_board.squares.clone(),
            }));
        }
        Err(status_code) => Err(status_code),
    }
}
