use crate::get_all_moves;
use crate::models::piece::{Piece, PieceColor, PieceGroup};
use crate::models::position::Positions;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Board {
    pub squares: Vec<Option<Piece>>,
    pub turn_color: PieceColor,
}

impl Board {
    pub fn get_legal_moves(&self, pos: Positions, color: PieceColor) -> Vec<Positions> {
        let Some(piece) = self.squares[pos.to_index() as usize] else {
            return vec![];
        };
        if piece.color != color {
            return vec![];
        }

        match piece.group {
            PieceGroup::Pawn => self.get_pawn_moves(pos, color),
            PieceGroup::Rook => self.get_rook_moves(pos, color),
            PieceGroup::Knight => self.get_knight_moves(pos, color),
            PieceGroup::Bishop => self.get_bishop_moves(pos, color),
            PieceGroup::Queen => self.get_queen_moves(pos, color),
            PieceGroup::King => self.get_king_moves(pos, color),
        }
    }

    pub fn move_piece(&mut self, origin: u8, destination: u8) -> () {
        if let (Some(origin_pos), Some(destination_pos)) = (
            Positions::from_index(origin),
            Positions::from_index(destination),
        ) {
            if self.is_valid_move(origin_pos, destination_pos) {
                self.squares[destination as usize] = self.squares[origin as usize].clone();
                self.squares[origin as usize] = None;
                self.toggle_turn_color();
            }
        }
    }

    fn toggle_turn_color(&mut self) -> () {
        match self.turn_color {
            PieceColor::White => self.turn_color = PieceColor::Black,
            PieceColor::Black => self.turn_color = PieceColor::White,
        }
    }

    fn is_valid_move(&self, origin: Positions, destination: Positions) -> bool {
        let Some(piece) = self.squares[origin.to_index() as usize] else {
            return false;
        };
        match piece.color == self.turn_color {
            true => {
                return self
                    .get_legal_moves(origin, self.turn_color)
                    .contains(&destination);
            }
            false => return false,
        }
    }

    fn get_pawn_moves(&self, pos: Positions, color: PieceColor) -> Vec<Positions> {
        let mut moves = vec![];
        let idx = pos.to_index();
        let one_forward = match color {
            PieceColor::White => idx.checked_add(8), //add 8 to move forward 1 square for white
            PieceColor::Black => idx.checked_sub(8), //subtract 8 to move forward 1 square for black
        };

        //second rank
        let starting_range = match color {
            PieceColor::White => (8, 15),
            PieceColor::Black => (48, 55),
        };
        if let Some(one_forward) = one_forward {
            if one_forward < 64 && self.squares[one_forward as usize].is_none() {
                //in bounds, advance one square
                if let Some(one_forward) = Positions::from_index(one_forward) {
                    moves.push(one_forward);
                }
                if idx >= starting_range.0 && idx <= starting_range.1 {
                    //pawn is on start square, advance two squares
                    //can also en passant if on start squares
                    let two_forward = match color {
                        PieceColor::White => Positions::from_index(idx + 16),
                        PieceColor::Black => Positions::from_index(idx - 16),
                    };
                    if let Some(two_forward) = two_forward {
                        moves.push(two_forward);
                    }
                }
            }
        }

        //diagonal forward
        //7, 9 for white
        //-7, -9 for black
        let (left_forward, right_forward) = match color {
            PieceColor::White => (idx.checked_add(7), idx.checked_add(9)),
            PieceColor::Black => (idx.checked_sub(7), idx.checked_sub(9)),
        };

        if let (Some(left), Some(right)) = (left_forward, right_forward) {
            if left < 64 && self.squares[left as usize].is_some_and(|piece| piece.color != color) {
                //in bounds, occupied by opposite color piece
                if let Some(left) = Positions::from_index(left) {
                    moves.push(left);
                }
                //en passant to come!
                // if idx >= starting_range.0 && idx <= starting_range.1 {
                //     //pawn is on start square, advance two squares
                //     //can also en passant if on start squares
                //     let two_forward = match color {
                //         PieceColor::White => Positions::from_index(idx + 16),
                //         PieceColor::Black => Positions::from_index(idx - 16),
                //     };
                //     if let Some(two_forward) = two_forward {
                //         moves.push(two_forward);
                //     }
                // }
            }
            if right < 64 && self.squares[right as usize].is_some_and(|piece| piece.color != color)
            {
                //in bounds and occupied
                if let Some(right) = Positions::from_index(right) {
                    moves.push(right);
                }
            }
        }

        moves
    }

    fn get_rook_moves(&self, pos: Positions, color: PieceColor) -> Vec<Positions> {
        let mut moves = vec![];
        let start = pos.to_index();
        let deltas = [1, -1, 8, -8];

        for delta in deltas {
            let mut current = start as i8;

            for _ in 0..7 {
                current += delta;

                if current < 0 || current > 63 {
                    break; //out of vertical bounds
                }

                let from_file = start as i8 % 8;
                let to_file = current % 8;

                if (delta == 1 && to_file <= from_file) || (delta == -1 && to_file >= from_file) {
                    break; //out of horizontal bounds
                }

                match self.squares[current as usize] {
                    //capture logic here
                    Some(occupant) => match occupant.color == color {
                        true => {
                            //same color, can't capture
                            break;
                        }
                        false => {
                            //diff color, can capture
                            if let Some(current) = Positions::from_index(current as u8) {
                                moves.push(current);
                            }
                            break;
                        }
                    },
                    None => {}
                }

                if let Some(current) = Positions::from_index(current as u8) {
                    moves.push(current);
                }
            }
        }

        moves
    }

    fn get_knight_moves(&self, pos: Positions, color: PieceColor) -> Vec<Positions> {
        let mut moves = vec![];
        let idx = pos.to_index();
        let deltas = [6, 10, 15, 17, -6, -10, -15, -17];

        for delta in deltas {
            let current = idx as i8 + delta;

            if current < 0 || current > 63 {
                continue; //out of vertical bounds
            }

            let from_file = idx as i8 % 8;
            let to_file = current % 8;
            let from_rank = idx as i8 / 8;
            let to_rank = current / 8;
            let file_diff = (from_file - to_file).abs();
            let rank_diff = (from_rank - to_rank).abs();

            if !((file_diff == 1 && rank_diff == 2) || (file_diff == 2 && rank_diff == 1)) {
                continue; //out of horizontal bounds
            }

            match self.squares[current as usize] {
                //capture logic here
                Some(occupant) => match occupant.color == color {
                    true => {
                        //same color, can't capture
                        continue;
                    }
                    false => {
                        //diff color, can capture
                        if let Some(current) = Positions::from_index(current as u8) {
                            moves.push(current);
                        }
                        continue;
                    }
                },
                None => {}
            }

            if let Some(current) = Positions::from_index(current as u8) {
                moves.push(current);
            }
        }

        moves
    }

    fn get_bishop_moves(&self, pos: Positions, color: PieceColor) -> Vec<Positions> {
        let mut moves = vec![];
        let start = pos.to_index();
        let deltas = [7, 9, -7, -9];

        for delta in deltas {
            let mut current = start as i8;

            for _ in 0..7 {
                current += delta;

                if current < 0 || current > 63 {
                    break; //out of vertical bounds
                }

                let from_file = start as i8 % 8;
                let to_file = current % 8;
                let from_rank = start as i8 / 8;
                let to_rank = current / 8;
                let file_diff = (from_file - to_file).abs();
                let rank_diff = (from_rank - to_rank).abs();

                if !(file_diff == rank_diff) {
                    break; //out of horizontal bounds
                }

                match self.squares[current as usize] {
                    //capture logic here
                    Some(occupant) => match occupant.color == color {
                        true => {
                            //same color, can't capture
                            break;
                        }
                        false => {
                            //diff color, can capture
                            if let Some(current) = Positions::from_index(current as u8) {
                                moves.push(current);
                            }
                            break;
                        }
                    },
                    None => {}
                }

                if let Some(current) = Positions::from_index(current as u8) {
                    moves.push(current);
                }
            }
        }

        moves
    }

    fn get_queen_moves(&self, pos: Positions, color: PieceColor) -> Vec<Positions> {
        let mut moves = vec![];
        let start = pos.to_index();
        let deltas = [1, 8, 7, 9, -1, -8, -7, -9];

        for delta in deltas {
            let mut current = start as i8;

            for _ in 0..7 {
                current += delta;

                if current < 0 || current > 63 {
                    break; //out of vertical bounds
                }

                let from_file = start as i8 % 8;
                let to_file = current % 8;
                let from_rank = start as i8 / 8;
                let to_rank = current / 8;
                let file_diff = (from_file - to_file).abs();
                let rank_diff = (from_rank - to_rank).abs();

                match delta {
                    1 if to_file <= from_file => break,
                    -1 if to_file >= from_file => break,
                    7 | -7 | 9 | -9 if file_diff != rank_diff => break,
                    _ => {}
                }

                match self.squares[current as usize] {
                    //capture logic here
                    Some(occupant) => match occupant.color == color {
                        true => {
                            //same color, can't capture
                            break;
                        }
                        false => {
                            //diff color, can capture
                            if let Some(current) = Positions::from_index(current as u8) {
                                moves.push(current);
                            }
                            break;
                        }
                    },
                    None => {}
                }

                if let Some(current) = Positions::from_index(current as u8) {
                    moves.push(current);
                }
            }
        }

        moves
    }

    fn get_king_moves(&self, pos: Positions, color: PieceColor) -> Vec<Positions> {
        let mut moves = vec![];
        let start = pos.to_index();
        let deltas = [1, -1, 8, -8, 7, -7, 9, -9];
        println!("get_king_moves for pos {:?}", pos);
        for delta in deltas {
            let destination = start as i8 + delta;

            if destination < 0 || destination > 63 {
                continue; //out of vertical bounds
            }

            let from_file = start as i8 % 8;
            let to_file = destination % 8;
            let from_rank = start as i8 / 8;
            let to_rank = destination / 8;
            let file_diff = (from_file - to_file).abs();
            let rank_diff = (from_rank - to_rank).abs();

            if file_diff > 1 || rank_diff > 1 || (file_diff == 0 && rank_diff == 0) {
                continue;
            }

            let opponent_color = match self.turn_color {
                PieceColor::White => PieceColor::Black,
                PieceColor::Black => PieceColor::White,
            };

            match self.squares[destination as usize] {
                //capture logic here
                Some(occupant) => match occupant.color == color {
                    true => {
                        println!("{:?}", occupant);
                        //same color, can't capture
                        continue;
                    }
                    false => {
                        //diff color, can capture
                        //TODO: check if piece is defended
                        //TODO: check if in check

                        //destination is occupied by opposing color
                        //if opponent has any pieces that can move to destination square on their turn, it's defended and can't be captured
                        //if opponent has any pieces that can move to empty destination square on next turn, king can't move there
                        //get_all_moves() isn't showing defending pieces' moves bc that square is currently occupied by one of their same colored piece allies

                        if let Some(destination_pos) = Positions::from_index(destination as u8) {
                            let opponent_color = match self.turn_color {
                                PieceColor::White => PieceColor::Black,
                                PieceColor::Black => PieceColor::White,
                            };
                            println!("destination: {:?}", destination);
                            println!("occupant: {:?}", occupant);
                            //temporarily rm piece from target square, check opp_moves, then add it back to target_square
                            let mut updated_board = Board {
                                squares: self.squares.clone(),
                                turn_color: self.turn_color,
                            };
                            updated_board.squares[destination as usize] = None;
                            let opponent_moves = get_all_moves(&updated_board, opponent_color);

                            println!("opp moves: {:?}", opponent_moves);
                            if opponent_moves
                                .iter()
                                .flatten()
                                .any(|&pos| pos == destination as u8)
                            {
                                //square is defended, can't capture
                                continue;
                            } else {
                                moves.push(destination_pos);
                            }
                        }
                        continue;
                    }
                },
                None => {}
            }

            if let Some(destination) = Positions::from_index(destination as u8) {
                moves.push(destination);
            }
        }

        moves
    }
}
