use crate::models::{board::Board, piece::Piece};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, sync::Arc};
use tokio::sync::Mutex;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BoardWithMoves {
    pub squares: Vec<Option<Piece>>,
    pub moves: Vec<Vec<u8>>,
}

#[derive(Clone)]
pub struct AppState {
    pub board: Arc<Mutex<Board>>,
    pub bitboards: HashMap<Piece, u64>,
}
