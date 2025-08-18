use crate::models::{
    board::Board,
    piece::{Piece, PieceColor, PieceGroup},
    position::Positions,
};

pub type PieceBitboards = [u64; 12];
pub type AttackBitboards = [u64; 2];

#[derive(Clone)]
pub struct Bitboards {
    pub all_pieces: PieceBitboards, //idx order: wp, wr, wn, wb, wq, wk, bp, br, bn, bb, bq, bk
    attacks: AttackBitboards,
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
        }
    }

    ///Gets all legal moves and returns a vector containing bitboards for each position.
    pub fn get_all_legal_moves(&mut self) -> Vec<u64> {
        self.attacks = self.get_all_attacks();
        let mut all_legal_moves = vec![];

        for origin in 0..64 {
            all_legal_moves.push(self.get_legal_moves(origin));
        }

        all_legal_moves
    }

    ///Gets all attacks and returns a bitboard of attacked squares for each color.
    pub fn get_all_attacks(&mut self) -> AttackBitboards {
        let mut all_attacks: AttackBitboards = [0u64, 0u64];

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
                piece.color,
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

            //move piece by updating origin and destination on piece's bitboard
            let bitboard = &mut self.all_pieces[piece.to_index()];
            *bitboard = (*bitboard & !(1u64 << origin)) | (1u64 << destination);
            board.update_square(destination, Some(piece));
            board.update_square(origin, None);

            //print updated bitboard
            // Bitboards::print_bitboard(*bitboard, "bitboard");
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

    fn get_legal_moves(&mut self, origin: u8) -> u64 {
        if let Some(piece) = self.get_occupant(origin) {
            match piece.group {
                PieceGroup::Pawn => self.get_pawn_moves(origin, piece),
                PieceGroup::Rook => self.get_sliding_moves(origin, piece),
                PieceGroup::Knight => self.get_knight_moves(origin, piece),
                PieceGroup::Bishop => self.get_sliding_moves(origin, piece),
                PieceGroup::Queen => self.get_sliding_moves(origin, piece),
                PieceGroup::King => {
                    let is_checked = self.attacks
                        [Piece::color_to_index(Piece::get_opposite_color(piece.color))]
                        & Bitboards::convert_to_bit(origin)
                        != 0;
                    //check if is_checked before getting any legal moves
                    //you can check this by comparing king's bb to opposite color's attacking bb
                    //if is_checked, can only move king to undefended square OR move same-color piece to block
                    //BUT cannot move a piece that is alraedy blocking a check
                    self.get_king_moves(origin, piece)
                }
            }
        } else {
            0u64
        }
    }

    fn is_square_defended(&self, square: u8, defending_color: PieceColor) -> bool {
        let defending_bitboard = self.attacks[Piece::color_to_index(defending_color)];
        let combined_bitboard = Bitboards::convert_to_bit(square) & defending_bitboard;
        let is_defended = combined_bitboard != 0;
        println!(
            "Is square {} defended by {:?}? {}",
            square, defending_color, is_defended
        );
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
        //TODO let en_passant = origin + 0;
        let mut possible_moves = vec![];

        if self.get_occupant(one_forward).is_none() {
            possible_moves.push(one_forward);
        }

        if origin >= starting_range.0
            && origin <= starting_range.1
            && self.get_occupant(two_forward).is_none()
        {
            possible_moves.push(two_forward);
        }

        if self.is_square_occupied_by_color(left_diagonal, Piece::get_opposite_color(piece.color)) {
            possible_moves.push(left_diagonal);
        }

        if self.is_square_occupied_by_color(right_diagonal, Piece::get_opposite_color(piece.color))
        {
            possible_moves.push(right_diagonal);
        }

        self.create_legal_moves_bitboard(piece, possible_moves, origin)
    }

    fn get_black_pawn_moves(&mut self, origin: u8, piece: Piece) -> u64 {
        let starting_range = (48, 55);

        let one_forward = origin.checked_sub(8);
        let two_forward = origin.checked_sub(16);
        let left_diagonal = origin.checked_sub(7);
        let right_diagonal = origin.checked_sub(9);
        //TODO let en_passant = origin.checked_sub(0); //just left/right diagonals if en passant is enabled
        let mut possible_moves: Vec<u8> = vec![];

        if let Some(one_forward) = one_forward {
            if self.get_occupant(one_forward).is_none() {
                possible_moves.push(one_forward);
            }
        }

        if let Some(two_forward) = two_forward {
            if origin >= starting_range.0
                && origin <= starting_range.1
                && self.get_occupant(two_forward).is_none()
            {
                possible_moves.push(two_forward);
            }
        }

        if let Some(left_diagonal) = left_diagonal {
            if self
                .is_square_occupied_by_color(left_diagonal, Piece::get_opposite_color(piece.color))
            {
                possible_moves.push(left_diagonal);
            }
        }

        if let Some(right_diagonal) = right_diagonal {
            if self
                .is_square_occupied_by_color(right_diagonal, Piece::get_opposite_color(piece.color))
            {
                possible_moves.push(right_diagonal);
            }
        }
        //TOD: if en passant, push en_passant to possible_moves

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

        //check validity, shift bitboard if valid
        for possible_move in possible_moves {
            if self.is_valid_move(origin, possible_move, &piece_bitboard, piece.color) {
                moves_bitboard |= 1u64 << possible_move;
            }
        }

        // println!("moves_bitboard for {:#?}", piece);
        // Bitboards::print_bitboard(moves_bitboard, "bitboard");

        //return bitboard with 1s in all legal move positions
        moves_bitboard
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

                if let Some(piece_at_destination) = self.get_occupant(destination as u8) {
                    if piece_at_destination.color == Piece::get_opposite_color(piece.color) {
                        //is capture
                        possible_moves.push(destination as u8);
                    }
                    break;
                }

                possible_moves.push(destination as u8);
            }
        }

        let piece_bitboard = self.all_pieces[piece.to_index()];
        let mut moves_bitboard = 0u64;

        //check validity, shift bitboard if valid
        for possible_move in possible_moves {
            if self.is_valid_move(origin, possible_move, &piece_bitboard, piece.color) {
                moves_bitboard |= 1u64 << possible_move;
            }
        }

        moves_bitboard
    }

    fn get_knight_moves(&mut self, origin: u8, piece: Piece) -> u64 {
        let directions: [i8; 8] = [6, 10, 15, 17, -6, -10, -15, -17];
        let mut possible_moves = vec![];

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

            if let Some(piece_at_destination) = self.get_occupant(destination as u8) {
                if piece_at_destination.color == Piece::get_opposite_color(piece.color) {
                    //is capture
                    possible_moves.push(destination as u8);
                    continue;
                }
            }

            possible_moves.push(destination as u8);
        }

        let piece_bitboard = self.all_pieces[piece.to_index()];
        let mut moves_bitboard = 0u64;

        //check validity, shift bitboard if valid
        for possible_move in possible_moves {
            if self.is_valid_move(origin, possible_move, &piece_bitboard, piece.color) {
                moves_bitboard |= 1u64 << possible_move;
            }
        }

        moves_bitboard
    }

    fn get_king_moves(&mut self, origin: u8, piece: Piece) -> u64 {
        let directions: [i8; 8] = [1, 8, 7, 9, -1, -8, -7, -9];
        let mut possible_moves = vec![];

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
                continue;
            }

            if let Some(piece_at_destination) = self.get_occupant(destination as u8) {
                if piece_at_destination.color == Piece::get_opposite_color(piece.color) {
                    //is capture
                    if self.is_square_defended(destination as u8, piece_at_destination.color) {
                        //square is defended, king can't capture
                        println!(
                            "square {} is defended by color {:?}",
                            destination, piece_at_destination.color
                        );
                        continue;
                    }
                }
            } else if self
                .is_square_defended(destination as u8, Piece::get_opposite_color(piece.color))
            {
                println!(
                    "square {} is defended by color {:?}",
                    destination,
                    Piece::get_opposite_color(piece.color)
                );
                //square is defended, king can't capture
                continue;
            }

            possible_moves.push(destination as u8);
        }

        let piece_bitboard = self.all_pieces[piece.to_index()];
        let mut moves_bitboard = 0u64;

        //check validity, shift bitboard if valid
        for possible_move in possible_moves {
            if self.is_valid_move(origin, possible_move, &piece_bitboard, piece.color) {
                moves_bitboard |= 1u64 << possible_move;
            }
        }

        moves_bitboard
    }

    /* Attack Calculations */

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
        }

        if let Some(right_diagonal) = right_diagonal {
            possible_attacks.push(right_diagonal);
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
        }

        let piece_bitboard = self.all_pieces[piece.to_index()];
        let mut attacks_bitboard = 0u64;

        //check validity, shift bitboard if valid
        for possible_attack in possible_attacks {
            if self.is_valid_attack(origin, possible_attack, &piece_bitboard) {
                attacks_bitboard |= 1u64 << possible_attack;
            }
        }

        attacks_bitboard
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

                if let Some(_) = self.get_occupant(destination as u8) {
                    break;
                }
            }
        }

        let piece_bitboard = self.all_pieces[piece.to_index()];
        let mut attacks_bitboard = 0u64;

        //check validity, shift bitboard if valid
        for possible_attack in possible_attacks {
            if self.is_valid_attack(origin, possible_attack, &piece_bitboard) {
                attacks_bitboard |= 1u64 << possible_attack;
            }
        }

        attacks_bitboard
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
        }

        let piece_bitboard = self.all_pieces[piece.to_index()];
        let mut attacks_bitboard = 0u64;

        //check validity, shift bitboard if valid
        for possible_attack in possible_attacks {
            if self.is_valid_attack(origin, possible_attack, &piece_bitboard) {
                attacks_bitboard |= 1u64 << possible_attack;
            }
        }

        attacks_bitboard
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

    /// Checks if origin & destination are within bounds, if origin is occupied by given piece, if destination is not occupied by same color.
    fn is_valid_move(
        &self,
        origin: u8,
        destination: u8,
        bitboard: &u64,
        color: PieceColor,
    ) -> bool {
        if origin < 0 || destination < 0 || origin > 63 || destination > 63 {
            //out of bounds
            return false;
        }
        if Bitboards::convert_to_bit(origin) & *bitboard == 0 {
            //piece to move's bitboard doesn't have 1 at origin
            return false;
        }
        if self.is_square_occupied_by_color(destination, color) {
            //destination is occupied by same colored piece
            return false;
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

    pub fn convert_to_bit(num: u8) -> u64 {
        1u64 << num
    }
}
