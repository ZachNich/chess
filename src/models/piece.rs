use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Hash)]
pub enum PieceGroup {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

#[derive(Copy, Clone, PartialEq, Eq, Debug, Serialize, Deserialize, Hash)]
pub enum PieceColor {
    White,
    Black,
}

#[derive(Copy, Clone, PartialEq, Eq, Debug, Serialize, Deserialize, Hash)]
pub struct Piece {
    pub group: PieceGroup,
    pub color: PieceColor,
}
