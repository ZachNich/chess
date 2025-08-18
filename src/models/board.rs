use crate::models::piece::{Piece, PieceColor, PieceGroup};
use crate::models::position::Positions;

#[derive(Clone)]
pub struct Board {
    pub squares: Vec<Option<Piece>>,
}

impl Board {
    ///Creates and returns new Bitboards instance with all_pieces set to initial piece bitboards.
    pub fn new() -> Self {
        Self {
            squares: Board::initialize_starting_squares(),
        }
    }

    pub fn update_square(&mut self, square: u8, update_to: Option<Piece>) -> () {
        self.squares[square as usize] = update_to;
    }

    pub fn initialize_starting_squares() -> Vec<Option<Piece>> {
        let mut squares: Vec<Option<Piece>> = vec![None; 64];

        //white pawns
        squares[Positions::A2.to_index() as usize] = Some(Piece {
            group: PieceGroup::Pawn,
            color: PieceColor::White,
            bitboard: 0u64,
        });
        squares[Positions::B2.to_index() as usize] = Some(Piece {
            group: PieceGroup::Pawn,
            color: PieceColor::White,
            bitboard: 0u64,
        });
        squares[Positions::C2.to_index() as usize] = Some(Piece {
            group: PieceGroup::Pawn,
            color: PieceColor::White,
            bitboard: 0u64,
        });
        squares[Positions::D2.to_index() as usize] = Some(Piece {
            group: PieceGroup::Pawn,
            color: PieceColor::White,
            bitboard: 0u64,
        });
        squares[Positions::E2.to_index() as usize] = Some(Piece {
            group: PieceGroup::Pawn,
            color: PieceColor::White,
            bitboard: 0u64,
        });
        squares[Positions::F2.to_index() as usize] = Some(Piece {
            group: PieceGroup::Pawn,
            color: PieceColor::White,
            bitboard: 0u64,
        });
        squares[Positions::G2.to_index() as usize] = Some(Piece {
            group: PieceGroup::Pawn,
            color: PieceColor::White,
            bitboard: 0u64,
        });
        squares[Positions::H2.to_index() as usize] = Some(Piece {
            group: PieceGroup::Pawn,
            color: PieceColor::White,
            bitboard: 0u64,
        });

        //black pawns
        squares[Positions::A7.to_index() as usize] = Some(Piece {
            group: PieceGroup::Pawn,
            color: PieceColor::Black,
            bitboard: 0u64,
        });
        squares[Positions::B7.to_index() as usize] = Some(Piece {
            group: PieceGroup::Pawn,
            color: PieceColor::Black,
            bitboard: 0u64,
        });
        squares[Positions::C7.to_index() as usize] = Some(Piece {
            group: PieceGroup::Pawn,
            color: PieceColor::Black,
            bitboard: 0u64,
        });
        squares[Positions::D7.to_index() as usize] = Some(Piece {
            group: PieceGroup::Pawn,
            color: PieceColor::Black,
            bitboard: 0u64,
        });
        squares[Positions::E7.to_index() as usize] = Some(Piece {
            group: PieceGroup::Pawn,
            color: PieceColor::Black,
            bitboard: 0u64,
        });
        squares[Positions::F7.to_index() as usize] = Some(Piece {
            group: PieceGroup::Pawn,
            color: PieceColor::Black,
            bitboard: 0u64,
        });
        squares[Positions::G7.to_index() as usize] = Some(Piece {
            group: PieceGroup::Pawn,
            color: PieceColor::Black,
            bitboard: 0u64,
        });
        squares[Positions::H7.to_index() as usize] = Some(Piece {
            group: PieceGroup::Pawn,
            color: PieceColor::Black,
            bitboard: 0u64,
        });

        //white rooks
        squares[Positions::A1.to_index() as usize] = Some(Piece {
            group: PieceGroup::Rook,
            color: PieceColor::White,
            bitboard: 0u64,
        });
        squares[Positions::H1.to_index() as usize] = Some(Piece {
            group: PieceGroup::Rook,
            color: PieceColor::White,
            bitboard: 0u64,
        });

        //black rooks
        squares[Positions::A8.to_index() as usize] = Some(Piece {
            group: PieceGroup::Rook,
            color: PieceColor::Black,
            bitboard: 0u64,
        });
        squares[Positions::H8.to_index() as usize] = Some(Piece {
            group: PieceGroup::Rook,
            color: PieceColor::Black,
            bitboard: 0u64,
        });

        //white knights
        squares[Positions::B1.to_index() as usize] = Some(Piece {
            group: PieceGroup::Knight,
            color: PieceColor::White,
            bitboard: 0u64,
        });
        squares[Positions::G1.to_index() as usize] = Some(Piece {
            group: PieceGroup::Knight,
            color: PieceColor::White,
            bitboard: 0u64,
        });

        //black knights
        squares[Positions::B8.to_index() as usize] = Some(Piece {
            group: PieceGroup::Knight,
            color: PieceColor::Black,
            bitboard: 0u64,
        });
        squares[Positions::G8.to_index() as usize] = Some(Piece {
            group: PieceGroup::Knight,
            color: PieceColor::Black,
            bitboard: 0u64,
        });

        //white bishops
        squares[Positions::C1.to_index() as usize] = Some(Piece {
            group: PieceGroup::Bishop,
            color: PieceColor::White,
            bitboard: 0u64,
        });
        squares[Positions::F1.to_index() as usize] = Some(Piece {
            group: PieceGroup::Bishop,
            color: PieceColor::White,
            bitboard: 0u64,
        });

        //black bishops
        squares[Positions::C8.to_index() as usize] = Some(Piece {
            group: PieceGroup::Bishop,
            color: PieceColor::Black,
            bitboard: 0u64,
        });
        squares[Positions::F8.to_index() as usize] = Some(Piece {
            group: PieceGroup::Bishop,
            color: PieceColor::Black,
            bitboard: 0u64,
        });

        //white queen
        squares[Positions::D1.to_index() as usize] = Some(Piece {
            group: PieceGroup::Queen,
            color: PieceColor::White,
            bitboard: 0u64,
        });

        //black queen
        squares[Positions::D8.to_index() as usize] = Some(Piece {
            group: PieceGroup::Queen,
            color: PieceColor::Black,
            bitboard: 0u64,
        });

        //white king
        squares[Positions::E1.to_index() as usize] = Some(Piece {
            group: PieceGroup::King,
            color: PieceColor::White,
            bitboard: 0u64,
        });

        //black king
        squares[Positions::E8.to_index() as usize] = Some(Piece {
            group: PieceGroup::King,
            color: PieceColor::Black,
            bitboard: 0u64,
        });

        squares
    }
}
