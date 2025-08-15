use crate::models::{
    board::Board,
    piece::{Piece, PieceColor, PieceGroup},
    position::Positions,
};

pub fn get_all_moves(board: &Board, color: PieceColor) -> Vec<Vec<u8>> {
    let mut moves = vec![];
    for idx in 0..64 {
        if let Some(pos) = Positions::from_index(idx) {
            moves.push(
                board
                    .get_legal_moves(pos, color)
                    .iter()
                    .map(|pos| pos.to_index())
                    .collect(),
            );
        }
    }
    moves
}

pub fn initialize_starting_position() -> Board {
    let mut squares: Vec<Option<Piece>> = vec![None; 64];

    //white pawns
    squares[Positions::A2.to_index() as usize] = Some(Piece {
        group: PieceGroup::Pawn,
        color: PieceColor::White,
    });
    squares[Positions::B2.to_index() as usize] = Some(Piece {
        group: PieceGroup::Pawn,
        color: PieceColor::White,
    });
    squares[Positions::C2.to_index() as usize] = Some(Piece {
        group: PieceGroup::Pawn,
        color: PieceColor::White,
    });
    squares[Positions::D2.to_index() as usize] = Some(Piece {
        group: PieceGroup::Pawn,
        color: PieceColor::White,
    });
    squares[Positions::E2.to_index() as usize] = Some(Piece {
        group: PieceGroup::Pawn,
        color: PieceColor::White,
    });
    squares[Positions::F2.to_index() as usize] = Some(Piece {
        group: PieceGroup::Pawn,
        color: PieceColor::White,
    });
    squares[Positions::G2.to_index() as usize] = Some(Piece {
        group: PieceGroup::Pawn,
        color: PieceColor::White,
    });
    squares[Positions::H2.to_index() as usize] = Some(Piece {
        group: PieceGroup::Pawn,
        color: PieceColor::White,
    });

    //black pawns
    squares[Positions::A7.to_index() as usize] = Some(Piece {
        group: PieceGroup::Pawn,
        color: PieceColor::Black,
    });
    squares[Positions::B7.to_index() as usize] = Some(Piece {
        group: PieceGroup::Pawn,
        color: PieceColor::Black,
    });
    squares[Positions::C7.to_index() as usize] = Some(Piece {
        group: PieceGroup::Pawn,
        color: PieceColor::Black,
    });
    squares[Positions::D7.to_index() as usize] = Some(Piece {
        group: PieceGroup::Pawn,
        color: PieceColor::Black,
    });
    squares[Positions::E7.to_index() as usize] = Some(Piece {
        group: PieceGroup::Pawn,
        color: PieceColor::Black,
    });
    squares[Positions::F7.to_index() as usize] = Some(Piece {
        group: PieceGroup::Pawn,
        color: PieceColor::Black,
    });
    squares[Positions::G7.to_index() as usize] = Some(Piece {
        group: PieceGroup::Pawn,
        color: PieceColor::Black,
    });
    squares[Positions::H7.to_index() as usize] = Some(Piece {
        group: PieceGroup::Pawn,
        color: PieceColor::Black,
    });

    //white rooks
    squares[Positions::A1.to_index() as usize] = Some(Piece {
        group: PieceGroup::Rook,
        color: PieceColor::White,
    });
    squares[Positions::H1.to_index() as usize] = Some(Piece {
        group: PieceGroup::Rook,
        color: PieceColor::White,
    });

    //black rooks
    squares[Positions::A8.to_index() as usize] = Some(Piece {
        group: PieceGroup::Rook,
        color: PieceColor::Black,
    });
    squares[Positions::H8.to_index() as usize] = Some(Piece {
        group: PieceGroup::Rook,
        color: PieceColor::Black,
    });

    //white knights
    squares[Positions::B1.to_index() as usize] = Some(Piece {
        group: PieceGroup::Knight,
        color: PieceColor::White,
    });
    squares[Positions::G1.to_index() as usize] = Some(Piece {
        group: PieceGroup::Knight,
        color: PieceColor::White,
    });

    //black knights
    squares[Positions::B8.to_index() as usize] = Some(Piece {
        group: PieceGroup::Knight,
        color: PieceColor::Black,
    });
    squares[Positions::G8.to_index() as usize] = Some(Piece {
        group: PieceGroup::Knight,
        color: PieceColor::Black,
    });

    //white bishops
    squares[Positions::C1.to_index() as usize] = Some(Piece {
        group: PieceGroup::Bishop,
        color: PieceColor::White,
    });
    squares[Positions::F1.to_index() as usize] = Some(Piece {
        group: PieceGroup::Bishop,
        color: PieceColor::White,
    });

    //black bishops
    squares[Positions::C8.to_index() as usize] = Some(Piece {
        group: PieceGroup::Bishop,
        color: PieceColor::Black,
    });
    squares[Positions::F8.to_index() as usize] = Some(Piece {
        group: PieceGroup::Bishop,
        color: PieceColor::Black,
    });

    //white queen
    squares[Positions::D1.to_index() as usize] = Some(Piece {
        group: PieceGroup::Queen,
        color: PieceColor::White,
    });

    //black queen
    squares[Positions::D8.to_index() as usize] = Some(Piece {
        group: PieceGroup::Queen,
        color: PieceColor::Black,
    });

    //white king
    squares[Positions::E1.to_index() as usize] = Some(Piece {
        group: PieceGroup::King,
        color: PieceColor::White,
    });

    //black king
    squares[Positions::E8.to_index() as usize] = Some(Piece {
        group: PieceGroup::King,
        color: PieceColor::Black,
    });

    Board {
        squares,
        turn_color: PieceColor::White,
    }
}
