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
    pub bitboard: u64,
}

impl Piece {
    //must match order of "piece.from_index(), piece.to_index()"
    pub fn initialize_all_pieces() -> [Piece; 12] {
        [
            Piece {
                color: PieceColor::White,
                group: PieceGroup::Pawn,
                bitboard: 0u64,
            },
            Piece {
                color: PieceColor::White,
                group: PieceGroup::Rook,
                bitboard: 0u64,
            },
            Piece {
                color: PieceColor::White,
                group: PieceGroup::Knight,
                bitboard: 0u64,
            },
            Piece {
                color: PieceColor::White,
                group: PieceGroup::Bishop,
                bitboard: 0u64,
            },
            Piece {
                color: PieceColor::White,
                group: PieceGroup::Queen,
                bitboard: 0u64,
            },
            Piece {
                color: PieceColor::White,
                group: PieceGroup::King,
                bitboard: 0u64,
            },
            Piece {
                color: PieceColor::Black,
                group: PieceGroup::Pawn,
                bitboard: 0u64,
            },
            Piece {
                color: PieceColor::Black,
                group: PieceGroup::Rook,
                bitboard: 0u64,
            },
            Piece {
                color: PieceColor::Black,
                group: PieceGroup::Knight,
                bitboard: 0u64,
            },
            Piece {
                color: PieceColor::Black,
                group: PieceGroup::Bishop,
                bitboard: 0u64,
            },
            Piece {
                color: PieceColor::Black,
                group: PieceGroup::Queen,
                bitboard: 0u64,
            },
            Piece {
                color: PieceColor::Black,
                group: PieceGroup::King,
                bitboard: 0u64,
            },
        ]
    }

    pub fn to_index(&self) -> usize {
        let piece_index = match self.group {
            PieceGroup::Pawn => 0,
            PieceGroup::Rook => 1,
            PieceGroup::Knight => 2,
            PieceGroup::Bishop => 3,
            PieceGroup::Queen => 4,
            PieceGroup::King => 5,
        };
        piece_index
            + match self.color {
                PieceColor::White => 0,
                PieceColor::Black => 6,
            }
    }

    //0-5 White, 6-11 Black
    //TODO: Make .bitboard an Option?
    pub fn from_index(index: usize) -> Option<Piece> {
        match index {
            0 => Some(Piece {
                group: PieceGroup::Pawn,
                color: PieceColor::White,
                bitboard: 0u64,
            }),
            1 => Some(Piece {
                group: PieceGroup::Rook,
                color: PieceColor::White,
                bitboard: 0u64,
            }),
            2 => Some(Piece {
                group: PieceGroup::Knight,
                color: PieceColor::White,
                bitboard: 0u64,
            }),
            3 => Some(Piece {
                group: PieceGroup::Bishop,
                color: PieceColor::White,
                bitboard: 0u64,
            }),
            4 => Some(Piece {
                group: PieceGroup::Queen,
                color: PieceColor::White,
                bitboard: 0u64,
            }),
            5 => Some(Piece {
                group: PieceGroup::King,
                color: PieceColor::White,
                bitboard: 0u64,
            }),
            6 => Some(Piece {
                group: PieceGroup::Pawn,
                color: PieceColor::Black,
                bitboard: 0u64,
            }),
            7 => Some(Piece {
                group: PieceGroup::Rook,
                color: PieceColor::Black,
                bitboard: 0u64,
            }),
            8 => Some(Piece {
                group: PieceGroup::Knight,
                color: PieceColor::Black,
                bitboard: 0u64,
            }),
            9 => Some(Piece {
                group: PieceGroup::Bishop,
                color: PieceColor::Black,
                bitboard: 0u64,
            }),
            10 => Some(Piece {
                group: PieceGroup::Queen,
                color: PieceColor::Black,
                bitboard: 0u64,
            }),
            11 => Some(Piece {
                group: PieceGroup::King,
                color: PieceColor::Black,
                bitboard: 0u64,
            }),
            _ => None,
        }
    }

    pub fn color_to_index(piece_color: PieceColor) -> usize {
        match piece_color {
            PieceColor::White => 0,
            PieceColor::Black => 1,
        }
    }

    pub fn get_opposite_color(piece_color: PieceColor) -> PieceColor {
        match piece_color {
            PieceColor::White => PieceColor::Black,
            PieceColor::Black => PieceColor::White,
        }
    }
}
