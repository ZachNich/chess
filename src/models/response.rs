use crate::models::piece::Piece;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BoardWithMoves {
    pub squares: Vec<Option<Piece>>,
    pub moves: Vec<Vec<u8>>,
}
