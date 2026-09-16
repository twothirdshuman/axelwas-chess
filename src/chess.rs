use std::{iter, slice::Iter};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Place {
    row: usize,
    file: usize
}

impl Default for Place {
    fn default() -> Self {
        Place { row: 0, file: 0 }
    }
}

impl Place {
    fn outside_board(&self) -> bool {
        if self.row >= 8 {
            return true;
        }
        if self.file >= 8 {
            return true;
        }
        false
    }
    fn goto(&self, to: &Self) -> Move {
        Move {
            to: *to,
            from: *self,
            promotion: None
        }
    }
}

#[derive(Clone, Debug, Copy, PartialEq, Eq)]
pub struct Move {
    from: Place,
    to: Place,
    promotion: Option<PieceTypes>
}

impl Move {
    fn in_between_squares(&self) -> Vec<Place> {
        let diff_x = self.from.file.abs_diff(self.to.file);
        let diff_y = self.from.row.abs_diff(self.to.row);

        if diff_x != diff_y {
            if !(diff_x == 0 || diff_y == 0) {
                return vec![];
            }
        }

        let to_add_x: isize = match (diff_x, self.from.file.checked_sub(self.to.file)) {
            (0, _) => 0,
            (_, Some(_)) => -1,
            (_, None) => 1
        };

        let to_add_y: isize = match (diff_y, self.from.row.checked_sub(self.to.row)) {
            (0, _) => 0,
            (_, Some(_)) => -1,
            (_, None) => 1
        };

        let mut ret = Vec::new();
        let to = diff_x.max(diff_y).try_into().unwrap_or(isize::MAX);
        for n in 1..to {
            ret.push(Place { 
                row: self.from.row.saturating_add_signed(to_add_y * n), 
                file: self.from.file.saturating_add_signed(to_add_x * n),
            });
        }
            
        ret
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Color {
    White,
    Black
}

impl Default for Color {
    fn default() -> Self {
        Self::White
    }
}

impl Color {
    pub fn opposite(&self) -> Self {
        match self {
            Color::Black => Color::White,
            Color::White => Color::Black
        }
    } 
}

#[derive(Clone, Debug, Copy, PartialEq, Eq)]
pub enum PieceTypes {
    Pawn {passantable: bool} ,
    Rock,
    Bishop,
    Knight,
    Queen,
    King
}

impl Default for PieceTypes {
    fn default() -> Self {
        PieceTypes::Pawn {passantable: false} 
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Piece {
    piece_type: PieceTypes,
    place: Place,
    color: Color,
    last_moved: usize
}

impl Default for Piece {
    fn default() -> Self {
        Self { 
            piece_type: PieceTypes::default(), 
            place: Place::default(), 
            color: Color::default(), 
            last_moved: 0 
        }
    }
}

impl Piece {
    fn create(piece_type: PieceTypes, place: Place, color: Color) -> Self {
        Self {
            last_moved: 0,
            piece_type,
            place,
            color
        }
    }

    fn into_char(&self) -> char {
        match (self.color, self.piece_type) {
            (Color::White, PieceTypes::King) => '♔',
            (Color::White, PieceTypes::Queen) => '♕',
            (Color::White, PieceTypes::Rock) => '♖',
            (Color::White, PieceTypes::Bishop) => '♗',
            (Color::White, PieceTypes::Knight) => '♘',
            (Color::White, PieceTypes::Pawn { .. }) => '♙',
            (Color::Black, PieceTypes::King) => '♚',
            (Color::Black, PieceTypes::Queen) => '♛',
            (Color::Black, PieceTypes::Rock) => '♜',
            (Color::Black, PieceTypes::Bishop) => '♝',
            (Color::Black, PieceTypes::Knight) => '♞',
            (Color::Black, PieceTypes::Pawn { .. }) => '♟',
        }
    }

    fn into_ascii(&self) -> char {
        match (self.color, self.piece_type) {
            (Color::White, PieceTypes::Pawn { .. }) => 'P',
            (Color::White, PieceTypes::Rock) => 'R',
            (Color::White, PieceTypes::Bishop) => 'B',
            (Color::White, PieceTypes::Knight) => 'N',
            (Color::White, PieceTypes::Queen) => 'Q',
            (Color::White, PieceTypes::King) => 'K',
            (Color::Black, PieceTypes::Pawn { .. }) => 'p',
            (Color::Black, PieceTypes::Rock) => 'r',
            (Color::Black, PieceTypes::Bishop) => 'b',
            (Color::Black, PieceTypes::Knight) => 'n',
            (Color::Black, PieceTypes::Queen) => 'q',
            (Color::Black, PieceTypes::King) => 'k',
        }
    }

    fn bishop_moves(&self) -> Vec<Place> {
        let mut moves = Vec::new();
        let own_row = self.place.row;
        let own_file = self.place.file;


        moves.extend(self.place.goto(&Place { row: own_row.saturating_sub(8), file: own_file.saturating_sub(8)}).in_between_squares());
        moves.extend(self.place.goto(&Place { row: own_row.saturating_sub(8), file: own_file.saturating_add(8)}).in_between_squares());
        moves.extend(self.place.goto(&Place { row: own_row.saturating_add(8), file: own_file.saturating_add(8)}).in_between_squares());
        moves.extend(self.place.goto(&Place { row: own_row.saturating_add(8), file: own_file.saturating_sub(8)}).in_between_squares());
        
        moves.into_iter().filter(|s| !s.outside_board()).collect()
    }

    fn rock_moves(&self) -> Vec<Place> {
        let mut moves = Vec::new();
        let own_row = self.place.row;
        let own_file = self.place.file;
        
        moves.extend(self.place.goto(&Place { row: own_row.saturating_sub(8), file: own_file}).in_between_squares());
        moves.extend(self.place.goto(&Place { row: own_row, file: own_file.saturating_add(8)}).in_between_squares());
        moves.extend(self.place.goto(&Place { row: own_row.saturating_add(8), file: own_file}).in_between_squares());
        moves.extend(self.place.goto(&Place { row: own_row, file: own_file.saturating_sub(8)}).in_between_squares());
        
        moves.into_iter().filter(|s| !s.outside_board()).collect()
    }

    fn queen_moves(&self) -> Vec<Place> {
        let mut rock = self.rock_moves();
        rock.extend(self.bishop_moves());
        rock
    }

    fn king_moves_basic(&self) -> Vec<Place> {
        let own_row = self.place.row;
        let own_file = self.place.file;

        let moves = vec![Place {
            row: own_row + 1,
            file: own_file + 1
        },
        Place {
            row: own_row + 1,
            file: own_file
        },
        Place {
            row: own_row + 1,
            file: own_file.wrapping_sub(1)
        },
        Place {
            row: own_row,
            file: own_file.wrapping_sub(1)
        },
        Place {
            row: own_row.wrapping_sub(1),
            file: own_file.wrapping_sub(1)
        },
        Place {
            row: own_row.wrapping_sub(1),
            file: own_file
        },
        Place {
            row: own_row.wrapping_sub(1),
            file: own_file + 1
        },
        Place {
            row: own_row,
            file: own_file + 1
        }];

        moves.into_iter().filter(|s| !s.outside_board()).collect()
    }

    fn knight_moves(&self) -> Vec<Place> {
        let own_row = self.place.row;
        let own_file = self.place.file;

        let moves = vec![Place {
            row: own_row + 2,
            file: own_file + 1
        },
        Place {
            row: own_row + 1,
            file: own_file + 2
        },
        Place {
            row: own_row.wrapping_sub(2),
            file: own_file.wrapping_sub(1)
        },
        Place {
            row: own_row.wrapping_sub(1),
            file: own_file.wrapping_sub(2)
        },
        Place {
            row: own_row + 1,
            file: own_file.wrapping_sub(2)
        },
        Place {
            row: own_row + 2,
            file: own_file.wrapping_sub(1)
        },
        Place {
            row: own_row.wrapping_sub(1),
            file: own_file + 2
        },
        Place {
            row: own_row.wrapping_sub(2),
            file: own_file + 1
        }];

        moves.into_iter().filter(|s| !s.outside_board()).collect()
    }

    fn pawn_moves(&self) -> Vec<Place> {
        let new_row = match self.color {
            Color::Black => self.place.row + 1,
            Color::White => self.place.row.saturating_sub(1),
        };

        if new_row >= 8 || new_row == 0{
            return vec![];
        }
        return vec![Place {
            row: new_row,
            file: self.place.file
        }];
    }

    fn pawn_specials(&self, position: &Position) -> Vec<Move> {
        let mut candidates: Vec<Place> = Vec::new();
        
        let forward: isize = match self.color {
            Color::Black => 1,
            Color::White => -1
        };

        let determine_pawn_capture = |towards: Place| {
            if position.piece_on(&towards).is_some_and(|f| f.color != self.color) {
                return true;
            }

            let passant_square = Place {
                row: towards.row.saturating_sub_signed(forward),
                file: towards.file
            };

            position.piece_on(&passant_square).is_some_and(|p| match p.piece_type {
                PieceTypes::Pawn { passantable: true } => true,
                _ => false
            })
        };

        let capture_right = Place {
            file: self.place.file + 1,
            row: self.place.row.saturating_add_signed(forward)
        };

        let capture_left = Place {
            file: self.place.file.wrapping_sub(1),
            row: self.place.row.saturating_add_signed(forward)
        };

        if determine_pawn_capture(capture_right) {
            candidates.push(capture_right);
        }

        if determine_pawn_capture(capture_left) {
            candidates.push(capture_left);
        }

        let just_forward = Place {
            file: self.place.file,
            row: self.place.row.saturating_add_signed(forward)
        };

        if just_forward.row == 0 || just_forward.row == 7 {
            candidates.push(just_forward);
        }

        let mut moves = Vec::new();

        for candidate in candidates {
            if candidate.row == 0 || candidate.row == 7 {
                moves.push(Move { 
                    from: self.place, 
                    to: candidate, 
                    promotion: Some(PieceTypes::Bishop) 
                });
                moves.push(Move { 
                    from: self.place, 
                    to: candidate, 
                    promotion: Some(PieceTypes::Knight) 
                });
                moves.push(Move { 
                    from: self.place, 
                    to: candidate, 
                    promotion: Some(PieceTypes::Queen) 
                });
                moves.push(Move { 
                    from: self.place, 
                    to: candidate, 
                    promotion: Some(PieceTypes::Rock) 
                });
                continue;
            }
            moves.push(Move {
                from: self.place,
                to: candidate,
                promotion: None,
            });
        }

        if self.last_moved != 0 {
            return moves;
        }
        
        let candidate_move = Place {
            file: self.place.file,
            row: self.place.row.saturating_add_signed(forward.saturating_mul(2)),
        };

        if !(position.theres_obstacle(&self.place, &candidate_move) || position.piece_on(&candidate_move).is_some()) {
            moves.push(Move {
                from: self.place,
                to: candidate_move,
                promotion: None
            })
        }

        moves.into_iter().filter(|s| !s.to.outside_board()).collect()
    }


    fn king_moves(&self, position: &Position) -> Vec<Move> {
        let mut moves = self.king_moves_basic();

        if self.last_moved != 0 {
            let left_rock = position.piece_on(&Place { row: self.place.row, file: 0 });
            if left_rock.is_some_and(|p| p.color == self.color && p.piece_type == PieceTypes::Rock && p.last_moved == 0) {
                moves.push(Place { row: self.place.row, file: self.place.file - 2 });
            }
            let right_rock = position.piece_on(&Place { row: self.place.row, file: 7 });
            if right_rock.is_some_and(|p| p.color == self.color && p.piece_type == PieceTypes::Rock && p.last_moved == 0) {
                moves.push(Place { row: self.place.row, file: self.place.file + 2 });
            }
        }
        
        let mut ret = vec![];

        let attacking: Vec<Place> = position.moves_from_without_king(self.color.opposite()).into_iter().map(|m| m.to).collect();
        for pl in moves {
            if let Some(extra) = self.place.goto(&pl).in_between_squares().pop() {
                if attacking.contains(&extra) {
                    continue;
                }
                if position.piece_on(&extra).is_some() {
                    continue;
                }
            }
            if !attacking.contains(&pl) {
                ret.push(Move {
                    from: self.place,
                    to: pl,
                    promotion: None
                });
            }
        }

        ret.into_iter().filter(|m| !position.piece_on(&m.to).is_some_and(|pi| pi.color == self.color)).collect()
    }

    pub fn moves(&self, position: &Position) -> Vec<Move>{
        let nonspecial_moves = match self.piece_type {
            PieceTypes::Bishop => self.bishop_moves(),
            PieceTypes::Rock => self.rock_moves(),
            PieceTypes::Queen => self.queen_moves(),
            PieceTypes::Pawn {..} => self.pawn_moves(),
            PieceTypes::Knight => self.knight_moves(),
            PieceTypes::King => { return self.king_moves(&position); }
        };

        // special pawn and king
        
        let specials = match self.piece_type {
            PieceTypes::Pawn {..} => self.pawn_specials(position),
            _ => Vec::new()
        };

        let obstacles_filtered = match (nonspecial_moves, &self.piece_type) {
            (m, PieceTypes::Knight) => m,
            (m, _) => m.into_iter().filter(|pl| !position.theres_obstacle(&self.place, pl)).collect(),
        };

        // filter moving onto oneself

        obstacles_filtered
            .into_iter()
            .filter(|pl| !position.piece_on(pl).is_some_and(|p| p.color == self.color))
            .map(|pl| Move { from: self.place, to: pl, promotion: None})
            .chain(specials.into_iter())
            .collect()
    }
}
pub enum Square {
    Empty,
    Piece(Piece)
}

impl Default for Square {
    fn default() -> Self {
        Self::Empty
    }
}

#[derive(Debug, Clone)]
pub struct Position {
    turn: Color,
    move_number: usize,
    pieces: Vec<Piece>
}



impl Position {

    fn theres_obstacle(&self, from: &Place, to: &Place) -> bool {
        let between = from.goto(to).in_between_squares();

        for sq in between {
            if let Some(_) = self.piece_on(&sq) {
                return true;
            }
        }

        false
    }

    fn piece_on(&self, square: &Place) -> Option<&Piece> {
        for piece in &self.pieces {
            if piece.place == *square {
                return Some(piece);
            }
        }
        None
    }

    fn moves_from_without_king(&self, color: Color) -> Vec<Move>{
        let mut moves = Vec::new();
        
        for piece in &self.pieces {
            if piece.color != color {
                continue;
            }
            if piece.piece_type == PieceTypes::King {
                continue;
            }
            moves.extend(piece.moves(self));
        }

        moves
    }

    pub fn all_moves_from(&self, color: Color) -> Vec<Move> {
        let mut moves = Vec::new();
        for piece in &self.pieces {
            if piece.color != color {
                continue;
            }

            moves.extend(piece.moves(self));
        }
        
        moves
    }

    // returns Ok(Self) if move could be implemented and Err(Self) if not possible, then it returns itself
    pub fn execute_move(mut self, to_move: Move) -> Result<Self, Self> {
        // naive implmentation

        let moving_piece = self.pieces.iter().enumerate().find(|(_, p)| p.place == to_move.from);
        let (index, _) = match moving_piece {
            None => return Err(self),
            Some(p) => p
        };

        if let Some((index, _capturing_piece)) = self.pieces.iter().enumerate().find(|(_, p)| p.place == to_move.to) {
            self.pieces.remove(index);
        }

        let moving_piece = self.pieces.get_mut(index).expect("impossible");
        self.move_number = self.move_number + 1;
        moving_piece.last_moved = self.move_number;
        moving_piece.place = to_move.to;


        self.turn = self.turn.opposite();

        Ok(self)
    }

    /// This function is made in honor of @ecogreen123 (context: was joking around in VC while coding)
    pub fn from_fen(fen: &str) -> Option<Self> {
        let turn = match fen.contains("b") {
            true => Color::Black,
            false => Color::White
        };

        let board_rep = match fen.split(" ").next() {
            None => return None,
            Some(s) => s
        };

        let flat_board: Vec<_> = board_rep.chars().flat_map(|c| match c {
            '1' => vec![None],
            '2' => vec![None; 2],
            '3' => vec![None; 3],
            '4' => vec![None; 4],
            '5' => vec![None; 5],
            '6' => vec![None; 6],
            '7' => vec![None; 7],
            '8' => vec![None; 8],
            'r' => vec![Some((Color::Black, PieceTypes::Rock))],
            'n' => vec![Some((Color::Black, PieceTypes::Knight))],
            'b' => vec![Some((Color::Black, PieceTypes::Bishop))],
            'q' => vec![Some((Color::Black, PieceTypes::Queen))],
            'k' => vec![Some((Color::Black, PieceTypes::King))],
            'p' => vec![Some((Color::Black, PieceTypes::Pawn {passantable: false}))],
            'R' => vec![Some((Color::White, PieceTypes::Rock))],
            'N' => vec![Some((Color::White, PieceTypes::Knight))],
            'B' => vec![Some((Color::White, PieceTypes::Bishop))],
            'Q' => vec![Some((Color::White, PieceTypes::Queen))],
            'K' => vec![Some((Color::White, PieceTypes::King))],
            'P' => vec![Some((Color::White, PieceTypes::Pawn {passantable: false}))],
            _ => vec![],
        }).collect();

        println!("{}",flat_board.len());

        if flat_board.len() != 64 {
            return None;
        }

        let pieces = flat_board.into_iter().enumerate().filter_map(|(i, p)| match p {
            None => None,
            Some((c, t)) => Some(Piece::create(t, Place { row: i / 8, file: i % 8 }, c))
        }).collect();

        dbg!(Some(Position {
            turn,
            move_number: 0,
            pieces
        }))
    }

    pub fn print_position(&self) {
        
        for row in 0..8 {
            for file in 0..8 {
                print!("{}", self.pieces
                    .iter()
                    .filter(|p| p.place == Place { row, file })
                    .map(|p| p.into_char())
                    .next()
                    .unwrap_or('.'));
                
            }
            println!();
        }
    }
}

impl Default for Position {
    fn default() -> Self {
        Self::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap()
    }
}
