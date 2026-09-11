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
        if self.row > 8 {
            return true;
        }
        if self.file > 8 {
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
            if diff_x != 0 || diff_y != 0 {
                return vec![];
            }
        }

        let to_add_x: isize = match (diff_x, self.from.file.checked_sub(self.to.file)) {
            (0, _) => 0,
            (_, Some(_)) => 1,
            (_, None) => -1
        };

        let to_add_y: isize = match (diff_y, self.from.row.checked_sub(self.to.row)) {
            (0, _) => 0,
            (_, Some(_)) => 1,
            (_, None) => -1
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

#[derive(Clone, Debug)]
pub enum Color {
    White,
    Black
}

impl Default for Color {
    fn default() -> Self {
        Self::White
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

#[derive(Debug)]
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

    fn king_moves(&self) -> Vec<Place> {
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
    
    fn pawn_specials(&self, position: Position) {
        
    }
    
    fn king_specials(&self, position: Position) {

    }

    pub fn moves(&self, position: Position) {
        let nonspecial_moves = match self.piece_type {
            PieceTypes::Bishop => self.bishop_moves(),
            PieceTypes::Rock => self.rock_moves(),
            PieceTypes::Queen => self.queen_moves(),
            PieceTypes::Pawn {..} => self.pawn_moves(),
            PieceTypes::Knight => self.knight_moves(),
            PieceTypes::King => self.king_moves()
        };

        // special pawn and king
        
        let specials = match self.piece_type {
            PieceTypes::Pawn {..} => self.pawn_specials(position),
            PieceTypes::King => self.king_specials(position),
            _ => Vec::new()
        };

        let obstacles_filtered = match (nonspecial_moves, &self.piece_type) {
            (m, PieceTypes::Bishop) => m.into_iter().filter(|pl| position.theres_obstacle(&self.place, pl)).collect(),
            (m, PieceTypes::Rock) => m.into_iter().filter(|pl| position.theres_obstacle(&self.place, pl)).collect(),
            (m, PieceTypes::Queen) => m.into_iter().filter(|pl| position.theres_obstacle(&self.place, pl)).collect(),
            (m, PieceTypes::Pawn {..}) => m.into_iter().filter(|pl| position.theres_obstacle(&self.place, pl)).collect(),
            (m, PieceTypes::King) => m.into_iter().filter(|pl| position.theres_obstacle(&self.place, pl)).collect(),
            (m, PieceTypes::Knight) => m,
        };

        // filter moving onto oneself
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

#[derive(Debug)]
pub struct Position {
    turn: Color,
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

    /// This function is made in honor of @ecogreen123
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
            Some((c, t)) => Some(Piece::create(t, Place { row: i % 8, file: i / 8 }, c))
        }).collect();

        Some(Position {
            turn,
            pieces
        })
    }
}

impl Default for Position {
    fn default() -> Self {
        Self::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap()
    }
}
