use crate::models::{
    board::Board,
    piece::{Piece, PieceColor, PieceGroup},
    position::Positions,
};

pub type PieceBitboards = [u64; 12];
pub type ColorBitboards = [u64; 2];

#[derive(Clone)]
pub struct Bitboards {
    pub all_pieces: PieceBitboards, //idx order: wp, wr, wn, wb, wq, wk, bp, br, bn, bb, bq, bk
    attacks: ColorBitboards,
    checking_pieces: ColorBitboards,
    en_passant: u64,
}

impl Bitboards {
    ///Creates and returns new Bitboards instance with all_pieces set to initial piece bitboards.
    pub fn new() -> Self {
        Self {
            all_pieces: Bitboards::create_piece_bitboards(),
            attacks: [
                Bitboards::create_empty_bitboard(),
                Bitboards::create_empty_bitboard(),
            ],
            checking_pieces: [
                Bitboards::create_empty_bitboard(),
                Bitboards::create_empty_bitboard(),
            ],
            en_passant: Bitboards::create_empty_bitboard(),
        }
    }

    ///Gets all legal moves and returns a vector containing bitboards for each position.
    pub fn get_all_legal_moves(&mut self, color: PieceColor) -> Vec<u64> {
        self.attacks = self.get_all_attacks();
        let mut all_legal_moves = vec![];

        for origin in 0..64 {
            all_legal_moves.push(self.get_legal_moves(origin, color));
        }
        all_legal_moves
    }

    ///Gets all attacks and returns a bitboard of attacked squares for each color.
    pub fn get_all_attacks(&mut self) -> ColorBitboards {
        self.attacks = [0u64, 0u64];
        self.checking_pieces = [0u64, 0u64];
        let mut all_attacks: ColorBitboards = [0u64, 0u64];

        for origin in 0..64 {
            match self.get_occupant(origin) {
                Some(occupant) => {
                    all_attacks[Piece::color_to_index(occupant.color)] |=
                        self.get_attacks(origin, occupant)
                }
                None => {}
            }
        }

        all_attacks
    }

    ///Moves piece by updating moved piece's bitboard and any captured piece's bitboard
    pub fn move_piece(&mut self, board: &mut Board, origin: u8, destination: u8) {
        println!("Moving piece from {} to {}", origin, destination);
        //get piece to move
        if let Some(piece) = self.get_occupant(origin) {
            //check move validity
            if !self.is_valid_move(
                origin,
                destination,
                &self.all_pieces[piece.to_index()],
                piece,
            ) {
                //invalid, exit
                println!("Invalid move from {:?} to {:?}", origin, destination);
                return; //TODO: Return invalid move error?
            }
            if self.is_square_occupied_by_color(destination, Piece::get_opposite_color(piece.color))
            {
                //is capture
                if let Some(captured_piece) = self.get_occupant(destination) {
                    println!("Capturing {:?} on {:?}", captured_piece, destination);
                    //capture piece by clearing destination on captured piece's bitboard
                    let captured_piece_bitboard = &mut self.all_pieces[captured_piece.to_index()];
                    *captured_piece_bitboard &= !(1u64 << destination);
                }
            }
            if self.en_passant & Bitboards::convert_to_bit(destination) != 0 {
                //is en passant
                let captured_pawn_square = match piece.color {
                    PieceColor::White => destination - 8,
                    PieceColor::Black => destination + 8,
                };
                println!("En passant captures square {:?}", captured_pawn_square);

                let captured_pawn_bitboard = &mut self.all_pieces[Piece::to_piece_index(
                    Piece::get_opposite_color(piece.color),
                    PieceGroup::Pawn,
                )];

                //clear square behind en passant destination
                *captured_pawn_bitboard &= !(1u64 << captured_pawn_square);
                board.update_square(captured_pawn_square, None);
            }

            //calculate en passants for next move
            let sign = match piece.color {
                PieceColor::White => -1,
                PieceColor::Black => 1,
            };

            if piece.group == PieceGroup::Pawn && origin.abs_diff(destination) == 8 * 2 {
                let mut en_passant_bb = 0u64;

                if self.get_occupant(destination - 1).is_some()
                    || self.get_occupant(destination + 1).is_some()
                {
                    //destination of possible en passanting pawn
                    en_passant_bb |= 1u64 << (destination as i8 + 8 * sign);
                }
                Bitboards::print_bitboard(en_passant_bb, "en_passant board");
                self.en_passant = en_passant_bb;
            }

            //move piece by updating origin and destination on piece's bitboard
            let bitboard = &mut self.all_pieces[piece.to_index()];
            *bitboard = (*bitboard & !(1u64 << origin)) | (1u64 << destination);
            board.update_square(destination, Some(piece));
            board.update_square(origin, None);

            self.checking_pieces = [0u64, 0u64];
        } else {
            println!(
                "No piece found on origin {:?}",
                Positions::from_index(origin)
            );
        }
    }

    /* Bitboard Initialization */

    fn create_piece_bitboards() -> PieceBitboards {
        Piece::initialize_all_pieces().map(|piece| Bitboards::create_bitboard_for_piece(piece))
    }

    fn create_empty_bitboard() -> u64 {
        0u64
    }

    fn create_bitboard_for_piece(piece: Piece) -> u64 {
        match piece.group {
            PieceGroup::Pawn => Bitboards::create_pawn_bitboard(piece.color),
            PieceGroup::Rook => Bitboards::create_rook_bitboard(piece.color),
            PieceGroup::Knight => Bitboards::create_knight_bitboard(piece.color),
            PieceGroup::Bishop => Bitboards::create_bishop_bitboard(piece.color),
            PieceGroup::Queen => Bitboards::create_queen_bitboard(piece.color),
            PieceGroup::King => Bitboards::create_king_bitboard(piece.color),
        }
    }

    fn create_bitboard_for_positions(positions: Vec<Positions>) -> u64 {
        let mut bitboard: u64 = 0;
        for position in positions {
            bitboard |= 1u64 << position.to_index();
        }
        bitboard
    }

    fn create_pawn_bitboard(color: PieceColor) -> u64 {
        match color {
            PieceColor::White => Bitboards::create_bitboard_for_positions(vec![
                Positions::A2,
                Positions::B2,
                Positions::C2,
                Positions::D2,
                Positions::E2,
                Positions::F2,
                Positions::G2,
                Positions::H2,
            ]),
            PieceColor::Black => Bitboards::create_bitboard_for_positions(vec![
                Positions::A7,
                Positions::B7,
                Positions::C7,
                Positions::D7,
                Positions::E7,
                Positions::F7,
                Positions::G7,
                Positions::H7,
            ]),
        }
    }

    fn create_rook_bitboard(color: PieceColor) -> u64 {
        match color {
            PieceColor::White => {
                Bitboards::create_bitboard_for_positions(vec![Positions::A1, Positions::H1])
            }
            PieceColor::Black => {
                Bitboards::create_bitboard_for_positions(vec![Positions::A8, Positions::H8])
            }
        }
    }

    fn create_knight_bitboard(color: PieceColor) -> u64 {
        match color {
            PieceColor::White => {
                Bitboards::create_bitboard_for_positions(vec![Positions::B1, Positions::G1])
            }
            PieceColor::Black => {
                Bitboards::create_bitboard_for_positions(vec![Positions::B8, Positions::G8])
            }
        }
    }

    fn create_bishop_bitboard(color: PieceColor) -> u64 {
        match color {
            PieceColor::White => {
                Bitboards::create_bitboard_for_positions(vec![Positions::C1, Positions::F1])
            }
            PieceColor::Black => {
                Bitboards::create_bitboard_for_positions(vec![Positions::C8, Positions::F8])
            }
        }
    }

    fn create_queen_bitboard(color: PieceColor) -> u64 {
        match color {
            PieceColor::White => Bitboards::create_bitboard_for_positions(vec![Positions::D1]),
            PieceColor::Black => Bitboards::create_bitboard_for_positions(vec![Positions::D8]),
        }
    }

    fn create_king_bitboard(color: PieceColor) -> u64 {
        match color {
            PieceColor::White => Bitboards::create_bitboard_for_positions(vec![Positions::E1]),
            PieceColor::Black => Bitboards::create_bitboard_for_positions(vec![Positions::E8]),
        }
    }

    fn print_bitboard(bitboard: u64, message: &str) {
        println!("{}", message);
        for rank in (0..8).rev() {
            for file in 0..8 {
                let square = rank * 8 + file;
                let bit = (bitboard >> square) & 1;
                print!("{}", bit);
            }
            println!();
        }
        println!();
    }

    /* Legal Move Calculations */

    fn get_legal_moves(&mut self, origin: u8, color: PieceColor) -> u64 {
        let empty_bitboard = Bitboards::create_empty_bitboard();
        if let Some(piece) = self.get_occupant(origin) {
            match color == piece.color {
                true => match piece.group {
                    PieceGroup::Pawn => self.get_pawn_moves(origin, piece),
                    PieceGroup::Rook => self.get_sliding_moves(origin, piece),
                    PieceGroup::Knight => self.get_knight_moves(origin, piece),
                    PieceGroup::Bishop => self.get_sliding_moves(origin, piece),
                    PieceGroup::Queen => self.get_sliding_moves(origin, piece),
                    PieceGroup::King => self.get_king_moves(origin, piece),
                },
                false => empty_bitboard,
            }
        } else {
            empty_bitboard
        }
    }

    fn is_square_defended(&self, square: u8, defending_color: PieceColor) -> bool {
        let defending_bitboard = self.attacks[Piece::color_to_index(defending_color)];
        let combined_bitboard = Bitboards::convert_to_bit(square) & defending_bitboard;
        let is_defended = combined_bitboard != 0;
        is_defended
    }

    fn get_pawn_moves(&mut self, origin: u8, piece: Piece) -> u64 {
        match piece.color {
            PieceColor::White => return self.get_white_pawn_moves(origin, piece),
            PieceColor::Black => return self.get_black_pawn_moves(origin, piece),
        };
    }

    fn get_white_pawn_moves(&mut self, origin: u8, piece: Piece) -> u64 {
        let starting_range = (8, 15);

        let one_forward = origin + 8;
        let two_forward = origin + 16;
        let left_diagonal = origin + 7;
        let right_diagonal = origin + 9;

        let mut possible_moves = vec![];

        if self.get_occupant(one_forward).is_none() {
            possible_moves.push(one_forward);

            if origin >= starting_range.0
                && origin <= starting_range.1
                && self.get_occupant(two_forward).is_none()
            {
                possible_moves.push(two_forward);
            }
        }

        if Bitboards::is_piece_in_horizontal_bounds(origin as i8, left_diagonal as i8, 7, piece)
            && (self
                .is_square_occupied_by_color(left_diagonal, Piece::get_opposite_color(piece.color))
                || self.en_passant & Bitboards::convert_to_bit(left_diagonal) != 0)
        {
            //is capture or en passant
            self.update_checking_pieces_bitboards(origin, left_diagonal, piece);
            possible_moves.push(left_diagonal);
        }

        if Bitboards::is_piece_in_horizontal_bounds(origin as i8, right_diagonal as i8, 9, piece)
            && (self.is_square_occupied_by_color(
                right_diagonal,
                Piece::get_opposite_color(piece.color),
            ) || self.en_passant & Bitboards::convert_to_bit(right_diagonal) != 0)
        {
            //is capture or en passant
            self.update_checking_pieces_bitboards(origin, right_diagonal, piece);
            possible_moves.push(right_diagonal);
        }

        self.create_legal_moves_bitboard(piece, possible_moves, origin)
    }

    fn get_black_pawn_moves(&mut self, origin: u8, piece: Piece) -> u64 {
        let starting_range = (Positions::A7.to_index(), Positions::H7.to_index());

        let one_forward = origin.checked_sub(8);
        let two_forward = origin.checked_sub(16);
        let left_diagonal = origin.checked_sub(7);
        let right_diagonal = origin.checked_sub(9);

        let mut possible_moves: Vec<u8> = vec![];

        if let Some(one_forward) = one_forward {
            if self.get_occupant(one_forward).is_none() {
                possible_moves.push(one_forward);

                if let Some(two_forward) = two_forward {
                    if origin >= starting_range.0
                        && origin <= starting_range.1
                        && self.get_occupant(two_forward).is_none()
                    {
                        possible_moves.push(two_forward);
                    }
                }
            }
        }

        if let Some(left_diagonal) = left_diagonal {
            if Bitboards::is_piece_in_horizontal_bounds(
                origin as i8,
                left_diagonal as i8,
                -7,
                piece,
            ) && (self
                .is_square_occupied_by_color(left_diagonal, Piece::get_opposite_color(piece.color))
                || self.en_passant & Bitboards::convert_to_bit(left_diagonal) != 0)
            {
                possible_moves.push(left_diagonal);
                self.update_checking_pieces_bitboards(origin, left_diagonal, piece);
            }
        }

        if let Some(right_diagonal) = right_diagonal {
            if Bitboards::is_piece_in_horizontal_bounds(
                origin as i8,
                right_diagonal as i8,
                -9,
                piece,
            ) && (self.is_square_occupied_by_color(
                right_diagonal,
                Piece::get_opposite_color(piece.color),
            ) || self.en_passant & Bitboards::convert_to_bit(right_diagonal) != 0)
            {
                possible_moves.push(right_diagonal);
                self.update_checking_pieces_bitboards(origin, right_diagonal, piece);
            }
        }

        self.create_legal_moves_bitboard(piece, possible_moves, origin)
    }

    fn get_sliding_moves(&mut self, origin: u8, piece: Piece) -> u64 {
        let directions = match piece.group {
            PieceGroup::Bishop => vec![7, -7, 9, -9],
            PieceGroup::Rook => vec![1, -1, 8, -8],
            PieceGroup::Queen => vec![1, -1, 7, -7, 8, -8, 9, -9],
            _ => vec![],
        };
        let mut possible_moves = vec![];

        for direction in directions {
            let mut destination = origin as i8;

            for _ in 0..8 {
                destination += direction;

                if destination < 0 || destination > 63 {
                    break; //out of vertical bounds
                }

                if !Bitboards::is_piece_in_horizontal_bounds(
                    origin as i8,
                    destination,
                    direction,
                    piece,
                ) {
                    break; //out of horizontal bounds
                }

                if let Some(piece_at_destination) = self.get_occupant(destination as u8) {
                    if piece_at_destination.color == Piece::get_opposite_color(piece.color) {
                        //is capture
                        possible_moves.push(destination as u8);
                        self.update_checking_pieces_bitboards(origin, destination as u8, piece);
                    }
                    break;
                }

                possible_moves.push(destination as u8);
                self.update_checking_pieces_bitboards(origin, destination as u8, piece);
            }
        }

        self.create_legal_moves_bitboard(piece, possible_moves, origin)
    }

    fn get_knight_moves(&mut self, origin: u8, piece: Piece) -> u64 {
        let directions: [i8; 8] = [6, 10, 15, 17, -6, -10, -15, -17];
        let mut possible_moves = vec![];

        for direction in directions {
            let destination = origin as i8 + direction;

            if destination < 0 || destination > 63 {
                continue; //out of vertical bounds
            }

            if !Bitboards::is_piece_in_horizontal_bounds(
                origin as i8,
                destination,
                direction,
                piece,
            ) {
                continue;
            }

            if let Some(piece_at_destination) = self.get_occupant(destination as u8) {
                if piece_at_destination.color == Piece::get_opposite_color(piece.color) {
                    //is capture
                    possible_moves.push(destination as u8);
                    continue;
                }
            }

            possible_moves.push(destination as u8);
            self.update_checking_pieces_bitboards(origin, destination as u8, piece);
        }

        self.create_legal_moves_bitboard(piece, possible_moves, origin)
    }

    fn get_king_moves(&mut self, origin: u8, piece: Piece) -> u64 {
        let directions: [i8; 8] = [1, 8, 7, 9, -1, -8, -7, -9];
        let mut possible_moves = vec![];

        for direction in directions {
            let destination = origin as i8 + direction;

            if destination < 0 || destination > 63 {
                continue; //out of vertical bounds
            }

            if !Bitboards::is_piece_in_horizontal_bounds(
                origin as i8,
                destination,
                direction,
                piece,
            ) {
                continue;
            }

            if let Some(piece_at_destination) = self.get_occupant(destination as u8) {
                if piece_at_destination.color == Piece::get_opposite_color(piece.color) {
                    //is capture
                    if self.is_square_defended(destination as u8, piece_at_destination.color) {
                        //square is defended, king can't capture
                        continue;
                    }
                }
            } else if self
                .is_square_defended(destination as u8, Piece::get_opposite_color(piece.color))
            {
                //square is defended, king can't capture
                continue;
            }

            possible_moves.push(destination as u8);
        }

        self.create_legal_moves_bitboard(piece, possible_moves, origin)
    }

    fn create_legal_moves_bitboard(
        &self,
        piece: Piece,
        possible_moves: Vec<u8>,
        origin: u8,
    ) -> u64 {
        let piece_bitboard = self.all_pieces[piece.to_index()];
        let mut moves_bitboard = 0u64;

        for possible_move in possible_moves {
            if self.is_valid_move(origin, possible_move, &piece_bitboard, piece) {
                moves_bitboard |= 1u64 << possible_move;
            }
        }

        //return bitboard with 1s in all legal move positions
        moves_bitboard
    }

    /* Attack Calculations */

    fn get_attacks(&mut self, origin: u8, piece: Piece) -> u64 {
        match piece.group {
            PieceGroup::Pawn => self.get_pawn_attacks(origin, piece),
            PieceGroup::Rook => self.get_sliding_attacks(origin, piece),
            PieceGroup::Knight => self.get_knight_attacks(origin, piece),
            PieceGroup::Bishop => self.get_sliding_attacks(origin, piece),
            PieceGroup::Queen => self.get_sliding_attacks(origin, piece),
            PieceGroup::King => self.get_king_attacks(origin, piece),
        }
    }

    fn get_pawn_attacks(&mut self, origin: u8, piece: Piece) -> u64 {
        match piece.color {
            PieceColor::White => return self.get_white_pawn_attacks(origin, piece),
            PieceColor::Black => return self.get_black_pawn_attacks(origin, piece),
        };
    }

    fn get_white_pawn_attacks(&mut self, origin: u8, piece: Piece) -> u64 {
        let starting_range = (8, 15);
        let left_diagonal = origin + 7;
        let right_diagonal = origin + 9;
        let mut possible_attacks = vec![];

        if origin >= starting_range.0 && origin <= starting_range.1 {
            //TODO: en passant?
        }

        possible_attacks.push(left_diagonal);
        possible_attacks.push(right_diagonal);

        self.update_checking_pieces_bitboards(origin, left_diagonal, piece);
        self.update_checking_pieces_bitboards(origin, right_diagonal, piece);

        self.create_legal_attacks_bitboard(piece, possible_attacks, origin)
    }

    fn get_black_pawn_attacks(&mut self, origin: u8, piece: Piece) -> u64 {
        let starting_range = (48, 55);
        let left_diagonal = origin.checked_sub(7);
        let right_diagonal = origin.checked_sub(9);
        let mut possible_attacks: Vec<u8> = vec![];

        if origin >= starting_range.0 && origin <= starting_range.1 {
            //TODO: en passant?
        }

        if let Some(left_diagonal) = left_diagonal {
            possible_attacks.push(left_diagonal);
            self.update_checking_pieces_bitboards(origin, left_diagonal, piece);
        }

        if let Some(right_diagonal) = right_diagonal {
            possible_attacks.push(right_diagonal);
            self.update_checking_pieces_bitboards(origin, right_diagonal, piece);
        }

        self.create_legal_attacks_bitboard(piece, possible_attacks, origin)
    }

    fn get_knight_attacks(&mut self, origin: u8, piece: Piece) -> u64 {
        let directions: [i8; 8] = [6, 10, 15, 17, -6, -10, -15, -17];
        let mut possible_attacks = vec![];

        for direction in directions {
            let destination = origin as i8 + direction;

            if destination < 0 || destination > 63 {
                continue; //out of vertical bounds
            }

            let from_file = origin as i8 % 8;
            let to_file = destination % 8;
            let from_rank = origin as i8 / 8;
            let to_rank = destination / 8;
            let file_diff = (from_file - to_file).abs();
            let rank_diff = (from_rank - to_rank).abs();

            if !((file_diff == 1 && rank_diff == 2) || (file_diff == 2 && rank_diff == 1)) {
                continue; //out of horizontal bounds
            }

            possible_attacks.push(destination as u8);
            self.update_checking_pieces_bitboards(origin, destination as u8, piece);
        }

        self.create_legal_attacks_bitboard(piece, possible_attacks, origin)
    }

    fn get_sliding_attacks(&mut self, origin: u8, piece: Piece) -> u64 {
        let directions = match piece.group {
            PieceGroup::Bishop => vec![7, -7, 9, -9],
            PieceGroup::Rook => vec![1, -1, 8, -8],
            PieceGroup::Queen => vec![1, -1, 7, -7, 8, -8, 9, -9],
            _ => vec![],
        };
        let mut possible_attacks = vec![];

        for direction in directions {
            let mut destination = origin as i8;

            for _ in 0..7 {
                destination += direction;

                if destination < 0 || destination > 63 {
                    break; //out of vertical bounds
                }

                let from_file = origin as i8 % 8;
                let to_file = destination % 8;
                let from_rank = origin as i8 / 8;
                let to_rank = destination / 8;
                let file_diff = (from_file - to_file).abs();
                let rank_diff = (from_rank - to_rank).abs();

                match direction {
                    1 if to_file <= from_file => break,
                    -1 if to_file >= from_file => break,
                    7 | -7 | 9 | -9 if file_diff != rank_diff => break,
                    _ => {}
                }

                possible_attacks.push(destination as u8);
                self.update_checking_pieces_bitboards(origin, destination as u8, piece);

                if let Some(_) = self.get_occupant(destination as u8) {
                    break;
                }
            }
        }

        self.create_legal_attacks_bitboard(piece, possible_attacks, origin)
    }

    fn get_king_attacks(&mut self, origin: u8, piece: Piece) -> u64 {
        let directions: [i8; 8] = [1, 8, 7, 9, -1, -8, -7, -9];
        let mut possible_attacks = vec![];

        for direction in directions {
            let destination = origin as i8 + direction;

            if destination < 0 || destination > 63 {
                continue; //out of vertical bounds
            }

            let from_file = origin as i8 % 8;
            let to_file = destination % 8;
            let from_rank = origin as i8 / 8;
            let to_rank = destination / 8;
            let file_diff = (from_file - to_file).abs();
            let rank_diff = (from_rank - to_rank).abs();

            if file_diff > 1 || rank_diff > 1 || (file_diff == 0 && rank_diff == 0) {
                continue; //out of horizontal bounds
            }

            possible_attacks.push(destination as u8);
            self.update_checking_pieces_bitboards(origin, destination as u8, piece);
        }

        self.create_legal_attacks_bitboard(piece, possible_attacks, origin)
    }

    fn create_legal_attacks_bitboard(
        &self,
        piece: Piece,
        possible_attacks: Vec<u8>,
        origin: u8,
    ) -> u64 {
        let piece_bitboard = self.all_pieces[piece.to_index()];
        let mut attacks_bitboard = 0u64;

        //check validity, shift bitboard if valid
        for possible_move in possible_attacks {
            if self.is_valid_attack(origin, possible_move, &piece_bitboard) {
                attacks_bitboard |= 1u64 << possible_move;
            }
        }

        //return bitboard with 1s in all legal attack positions
        attacks_bitboard
    }

    /* Helper Methods */

    fn get_occupant(&self, square: u8) -> Option<Piece> {
        let destination_bitboard = Bitboards::convert_to_bit(square);
        for (piece_index, piece_bitboard) in self.all_pieces.iter().enumerate() {
            if piece_bitboard & destination_bitboard != 0 {
                //both have 1 in same place, so piece is at destination
                return Piece::from_index(piece_index);
            }
        }
        return None;
    }

    fn is_checked(&self, color: PieceColor) -> bool {
        let king_bitboard = match color {
            PieceColor::White => {
                self.all_pieces[Piece {
                    group: PieceGroup::King,
                    color: PieceColor::White,
                    bitboard: 0u64,
                }
                .to_index()]
            }
            PieceColor::Black => {
                self.all_pieces[Piece {
                    group: PieceGroup::King,
                    color: PieceColor::Black,
                    bitboard: 0u64,
                }
                .to_index()]
            }
        };
        self.attacks[Piece::color_to_index(Piece::get_opposite_color(color))] & king_bitboard != 0
    }

    fn is_piece_in_horizontal_bounds(
        origin: i8,
        destination: i8,
        direction: i8,
        piece: Piece,
    ) -> bool {
        match piece.group {
            PieceGroup::Pawn | PieceGroup::Knight | PieceGroup::King => {
                Bitboards::is_fixed_piece_in_horizontal_bounds(origin, destination)
            }
            _ => Bitboards::is_sliding_piece_in_horizontal_bounds(origin, destination, direction),
        }
    }

    fn is_fixed_piece_in_horizontal_bounds(origin: i8, destination: i8) -> bool {
        let origin_file = origin % 8;
        let destination_file = destination % 8;

        (origin_file - destination_file).abs() <= 2
    }

    fn is_sliding_piece_in_horizontal_bounds(origin: i8, destination: i8, direction: i8) -> bool {
        let from_file = origin % 8;
        let to_file = destination % 8;

        match direction {
            1 if to_file == 0 => return false,  // wrapped from H to A
            -1 if to_file == 7 => return false, // wrapped from A to H
            9 | -7 if to_file <= from_file => return false, // right diagonal must increase file
            7 | -9 if to_file >= from_file => return false, // left diagonal must decrease file
            _ => {}
        }

        true
    }

    fn get_all_pieces_on_one_bitboard(&self) -> u64 {
        self.all_pieces[Piece::to_piece_index(PieceColor::White, PieceGroup::Pawn)]
            | self.all_pieces[Piece::to_piece_index(PieceColor::White, PieceGroup::Rook)]
            | self.all_pieces[Piece::to_piece_index(PieceColor::White, PieceGroup::Knight)]
            | self.all_pieces[Piece::to_piece_index(PieceColor::White, PieceGroup::Bishop)]
            | self.all_pieces[Piece::to_piece_index(PieceColor::White, PieceGroup::Queen)]
            | self.all_pieces[Piece::to_piece_index(PieceColor::White, PieceGroup::King)]
            | self.all_pieces[Piece::to_piece_index(PieceColor::Black, PieceGroup::Pawn)]
            | self.all_pieces[Piece::to_piece_index(PieceColor::Black, PieceGroup::Rook)]
            | self.all_pieces[Piece::to_piece_index(PieceColor::Black, PieceGroup::Knight)]
            | self.all_pieces[Piece::to_piece_index(PieceColor::Black, PieceGroup::Bishop)]
            | self.all_pieces[Piece::to_piece_index(PieceColor::Black, PieceGroup::Queen)]
            | self.all_pieces[Piece::to_piece_index(PieceColor::Black, PieceGroup::King)]
    }

    fn get_same_color_pieces_on_one_bitboard(&self, color: PieceColor) -> u64 {
        match color {
            PieceColor::White => {
                self.all_pieces[Piece::to_piece_index(PieceColor::White, PieceGroup::Pawn)]
                    | self.all_pieces[Piece::to_piece_index(PieceColor::White, PieceGroup::Rook)]
                    | self.all_pieces[Piece::to_piece_index(PieceColor::White, PieceGroup::Knight)]
                    | self.all_pieces[Piece::to_piece_index(PieceColor::White, PieceGroup::Bishop)]
                    | self.all_pieces[Piece::to_piece_index(PieceColor::White, PieceGroup::Queen)]
                    | self.all_pieces[Piece::to_piece_index(PieceColor::White, PieceGroup::King)]
            }

            PieceColor::Black => {
                self.all_pieces[Piece::to_piece_index(PieceColor::Black, PieceGroup::Pawn)]
                    | self.all_pieces[Piece::to_piece_index(PieceColor::Black, PieceGroup::Rook)]
                    | self.all_pieces[Piece::to_piece_index(PieceColor::Black, PieceGroup::Knight)]
                    | self.all_pieces[Piece::to_piece_index(PieceColor::Black, PieceGroup::Bishop)]
                    | self.all_pieces[Piece::to_piece_index(PieceColor::Black, PieceGroup::Queen)]
                    | self.all_pieces[Piece::to_piece_index(PieceColor::Black, PieceGroup::King)]
            }
        }
    }

    /// Checks if origin & destination are within bounds, if origin is occupied by given piece, if destination is not occupied by same color.
    fn is_valid_move(
        &self,
        origin: u8,
        destination: u8,
        origin_bitboard: &u64,
        piece: Piece,
    ) -> bool {
        if origin < 0 || destination < 0 || origin > 63 || destination > 63 {
            //out of bounds
            return false;
        }
        if Bitboards::convert_to_bit(origin) & *origin_bitboard == 0 {
            //piece to move's bitboard doesn't have piece at origin
            return false;
        }
        if self.is_square_occupied_by_color(destination, piece.color) {
            //destination is occupied by same colored piece
            return false;
        }
        if !self.validate_pins(piece, origin, destination) {
            //pinned piece tries to move out of pin
            return false;
        }
        if !self.validate_checks(piece, destination) {
            //if king is in check, this is a non-king move that neither captures nor blocks a checking piece
            return false;
        }

        if piece.group == PieceGroup::King
            && self.is_square_defended(destination as u8, Piece::get_opposite_color(piece.color))
        {
            //king move is to defended square
            return false;
        }

        true
    }

    fn validate_checks(&self, piece: Piece, destination: u8) -> bool {
        if self.is_checked(piece.color) && piece.group != PieceGroup::King {
            //is check
            let king_bitboard =
                self.all_pieces[Piece::to_piece_index(piece.color, PieceGroup::King)];
            let checkers_bitboard =
                self.checking_pieces[Piece::color_to_index(Piece::get_opposite_color(piece.color))];
            let blocking_bitboard =
                Bitboards::get_rays_from_bitboards(checkers_bitboard, king_bitboard);

            if self.checking_pieces[Piece::color_to_index(Piece::get_opposite_color(piece.color))]
                & Bitboards::convert_to_bit(destination)
                == 0
                && blocking_bitboard & Bitboards::convert_to_bit(destination) == 0
            {
                //is non-king move that is neither capturing nor blocking a checking piece
                return false;
            }
        }
        true
    }

    fn validate_pins(&self, piece: Piece, origin: u8, destination: u8) -> bool {
        let bishop_bb = self.all_pieces
            [Piece::to_piece_index(Piece::get_opposite_color(piece.color), PieceGroup::Bishop)];
        let rook_bb = self.all_pieces
            [Piece::to_piece_index(Piece::get_opposite_color(piece.color), PieceGroup::Rook)];
        let queen_bb = self.all_pieces
            [Piece::to_piece_index(Piece::get_opposite_color(piece.color), PieceGroup::Queen)];

        let king_bitboard = self.all_pieces[Piece::to_piece_index(piece.color, PieceGroup::King)];
        for mut sliding_bb in [bishop_bb, rook_bb, queen_bb] {
            while sliding_bb != 0 {
                let ray_origin = sliding_bb.trailing_zeros() as u8;
                let ray_destination = king_bitboard.trailing_zeros() as u8;
                let pin_ray = Bitboards::get_ray_bitboard(ray_origin, ray_destination);
                let obstructions = pin_ray & self.get_all_pieces_on_one_bitboard();
                let friendly_pieces = self.get_same_color_pieces_on_one_bitboard(piece.color);
                if origin == 18 {
                    Bitboards::print_bitboard(pin_ray, "Pin Ray");
                    println!(
                        "pin origin {}, king destination {}",
                        ray_origin, ray_destination
                    );
                    Bitboards::print_bitboard(obstructions, "Obstructions in Pin Ray");
                    Bitboards::print_bitboard(friendly_pieces, "All Friendly Pieces");
                }

                // only piece obstructing pin is current piece, valid pin
                if obstructions == Bitboards::convert_to_bit(origin) {
                    let legal_pin_ray = Bitboards::get_ray_bitboard(ray_origin, ray_destination)
                        | (1u64 << ray_origin)
                        | (1u64 << ray_destination);

                    if Bitboards::convert_to_bit(destination) & legal_pin_ray == 0 {
                        //trying to escape pin, invalid move
                        return false;
                    }
                }
                sliding_bb &= sliding_bb - 1;
            }
        }
        true
    }

    /// Checks if origin & destination are within bounds, if origin is occupied by given piece.
    fn is_valid_attack(&self, origin: u8, destination: u8, bitboard: &u64) -> bool {
        if origin < 0 || destination < 0 || origin > 63 || destination > 63 {
            //out of bounds
            return false;
        }
        if Bitboards::convert_to_bit(origin) & *bitboard == 0 {
            //piece to move's bitboard doesn't have 1 at origin
            return false;
        }
        true
    }

    fn get_rays_from_bitboards(origins: u64, destination: u64) -> u64 {
        if destination.count_ones() != 1 {
            //TODO: Handle this better

            panic!(
                "Destination bitboard must have exactly one bit set, received {:#?}",
                destination as u8
            );
        }

        let destination_index = destination.trailing_zeros() as u8;
        let mut rays_bitboard = 0u64;

        let mut origin_bits = origins;
        while origin_bits != 0 {
            let origin_index = origin_bits.trailing_zeros() as u8;
            origin_bits &= origin_bits - 1; // clears the lowest/least significant bit

            rays_bitboard |= Bitboards::get_ray_bitboard(origin_index, destination_index);
        }

        rays_bitboard
    }

    fn get_ray_bitboard(origin: u8, destination: u8) -> u64 {
        let from_file = origin as i8 % 8;
        let to_file = destination as i8 % 8;
        let from_rank = origin as i8 / 8;
        let to_rank = destination as i8 / 8;
        let file_diff = to_file - from_file;
        let rank_diff = to_rank - from_rank;
        let (file_step, rank_step) = match (file_diff, rank_diff) {
            (0, _) => (0, rank_diff.signum()), // vertical
            (_, 0) => (file_diff.signum(), 0), // horizontal
            _ if file_diff.abs() == rank_diff.abs() => (
                // diagonal
                file_diff.signum(),
                rank_diff.signum(),
            ),
            _ => return 0, // invalid
        };

        let mut bitboard = 0u64;
        let mut current_rank = from_rank + rank_step;
        let mut current_file = from_file + file_step;

        //while not at destination
        while current_rank != to_rank || current_file != to_file {
            let square = (current_rank * 8 + current_file) as u8;
            bitboard |= 1u64 << square;

            current_rank += rank_step;
            current_file += file_step;
        }

        bitboard
    }

    fn update_checking_pieces_bitboards(
        &mut self,
        attacker_square: u8,
        attacked_square: u8,
        attacking_piece: Piece,
    ) {
        if self.get_occupant(attacked_square).is_some_and(|occupant| {
            occupant.color == Piece::get_opposite_color(attacking_piece.color)
                && occupant.group == PieceGroup::King
        }) {
            //is checker
            self.checking_pieces[Piece::color_to_index(attacking_piece.color)] |=
                1u64 << attacker_square;
        }
    }

    fn is_square_occupied_by_color(&self, square: u8, piece_color: PieceColor) -> bool {
        match piece_color {
            PieceColor::White => {
                for i in 0..6 {
                    if Bitboards::is_square_occupied_by_bitboard(square, self.all_pieces[i]) {
                        return true;
                    }
                }
                false
            }
            PieceColor::Black => {
                for i in 6..12 {
                    if Bitboards::is_square_occupied_by_bitboard(square, self.all_pieces[i]) {
                        return true;
                    }
                }
                false
            }
        }
    }

    fn is_square_occupied_by_bitboard(square: u8, bitboard: u64) -> bool {
        Bitboards::convert_to_bit(square) & bitboard != 0
    }

    fn convert_bitboard_to_indexes(mut bitboard: u64) -> Vec<u8> {
        let mut indexes = Vec::new();

        while bitboard != 0 {
            //lsb = least significant bit
            let lsb_index = bitboard.trailing_zeros() as u8;
            indexes.push(lsb_index);
            bitboard &= bitboard - 1; // clear the least significant bit
        }

        indexes
    }

    pub fn convert_to_bit(num: u8) -> u64 {
        1u64 << num
    }
}
