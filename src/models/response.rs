use crate::models::{bitboards::Bitboards, board::Board, piece::Piece};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SquaresAndMoves {
    pub squares: Vec<Option<Piece>>,
    pub moves: Vec<u64>,
}

#[derive(Clone)]
pub struct AppState {
    pub board: Arc<Mutex<Board>>,
    pub bitboards: Arc<Mutex<Bitboards>>,
}
