use std::collections::HashMap;

use crate::models::{
    piece::{Piece, PieceColor, PieceGroup},
    position::Positions,
};

pub fn get_initial_bitboards() -> HashMap<Piece, u64> {
    let mut map = HashMap::new();
    let pieces = [
        Piece {
            color: PieceColor::White,
            group: PieceGroup::Pawn,
        },
        Piece {
            color: PieceColor::White,
            group: PieceGroup::Rook,
        },
        Piece {
            color: PieceColor::White,
            group: PieceGroup::Knight,
        },
        Piece {
            color: PieceColor::White,
            group: PieceGroup::Bishop,
        },
        Piece {
            color: PieceColor::White,
            group: PieceGroup::Queen,
        },
        Piece {
            color: PieceColor::White,
            group: PieceGroup::King,
        },
        Piece {
            color: PieceColor::Black,
            group: PieceGroup::Pawn,
        },
        Piece {
            color: PieceColor::Black,
            group: PieceGroup::Rook,
        },
        Piece {
            color: PieceColor::Black,
            group: PieceGroup::Knight,
        },
        Piece {
            color: PieceColor::Black,
            group: PieceGroup::Bishop,
        },
        Piece {
            color: PieceColor::Black,
            group: PieceGroup::Queen,
        },
        Piece {
            color: PieceColor::Black,
            group: PieceGroup::King,
        },
    ];

    for piece in pieces {
        map.insert(piece, create_bitboard_for_piece(piece));
    }

    map
}

fn create_bitboard_for_piece(piece: Piece) -> u64 {
    match piece.group {
        PieceGroup::Pawn => create_pawn_bitboard(piece.color),
        PieceGroup::Rook => create_rook_bitboard(piece.color),
        PieceGroup::Knight => create_knight_bitboard(piece.color),
        PieceGroup::Bishop => create_bishop_bitboard(piece.color),
        PieceGroup::Queen => create_queen_bitboard(piece.color),
        PieceGroup::King => create_king_bitboard(piece.color),
    }
}

fn create_pawn_bitboard(color: PieceColor) -> u64 {
    match color {
        PieceColor::White => create_white_pawn_bitboard(),
        PieceColor::Black => create_black_pawn_bitboard(),
    }
}

fn create_white_pawn_bitboard() -> u64 {
    let mut pawns: u64 = 0;
    for i in Positions::A2.to_index()..=Positions::H2.to_index() {
        pawns |= 1u64 << i;
    }
    pawns
}

fn create_black_pawn_bitboard() -> u64 {
    let mut pawns: u64 = 0;
    for i in Positions::A7.to_index()..=Positions::H7.to_index() {
        pawns |= 1u64 << i;
    }
    pawns
}

fn create_rook_bitboard(color: PieceColor) -> u64 {
    match color {
        PieceColor::White => create_white_rook_bitboard(),
        PieceColor::Black => create_black_rook_bitboard(),
    }
}

fn create_white_rook_bitboard() -> u64 {
    let mut rooks: u64 = 0;
    rooks |= 1u64 << Positions::A1.to_index();
    rooks |= 1u64 << Positions::H1.to_index();
    rooks
}

fn create_black_rook_bitboard() -> u64 {
    let mut rooks: u64 = 0;
    rooks |= 1u64 << Positions::A8.to_index();
    rooks |= 1u64 << Positions::H8.to_index();
    rooks
}

fn create_knight_bitboard(color: PieceColor) -> u64 {
    match color {
        PieceColor::White => create_white_knight_bitboard(),
        PieceColor::Black => create_black_knight_bitboard(),
    }
}

fn create_white_knight_bitboard() -> u64 {
    let mut knights: u64 = 0;
    knights |= 1u64 << Positions::B1.to_index();
    knights |= 1u64 << Positions::G1.to_index();
    knights
}

fn create_black_knight_bitboard() -> u64 {
    let mut knights: u64 = 0;
    knights |= 1u64 << Positions::B8.to_index();
    knights |= 1u64 << Positions::G8.to_index();
    knights
}

fn create_bishop_bitboard(color: PieceColor) -> u64 {
    match color {
        PieceColor::White => create_white_bishop_bitboard(),
        PieceColor::Black => create_black_bishop_bitboard(),
    }
}

fn create_white_bishop_bitboard() -> u64 {
    let mut bishops: u64 = 0;
    bishops |= 1u64 << Positions::C1.to_index();
    bishops |= 1u64 << Positions::F1.to_index();
    bishops
}

fn create_black_bishop_bitboard() -> u64 {
    let mut bishops: u64 = 0;
    bishops |= 1u64 << Positions::C8.to_index();
    bishops |= 1u64 << Positions::F8.to_index();
    bishops
}

fn create_queen_bitboard(color: PieceColor) -> u64 {
    match color {
        PieceColor::White => create_white_queen_bitboard(),
        PieceColor::Black => create_black_queen_bitboard(),
    }
}

fn create_white_queen_bitboard() -> u64 {
    let mut queens: u64 = 0;
    queens |= 1u64 << Positions::D1.to_index();
    queens
}

fn create_black_queen_bitboard() -> u64 {
    let mut queens: u64 = 0;
    queens |= 1u64 << Positions::D8.to_index();
    queens
}

fn create_king_bitboard(color: PieceColor) -> u64 {
    match color {
        PieceColor::White => create_white_king_bitboard(),
        PieceColor::Black => create_black_king_bitboard(),
    }
}

fn create_white_king_bitboard() -> u64 {
    let mut king: u64 = 0;
    king |= 1u64 << Positions::D1.to_index();
    king
}

fn create_black_king_bitboard() -> u64 {
    let mut king: u64 = 0;
    king |= 1u64 << Positions::D8.to_index();
    king
}
