
pub struct Place {
    row: u8,
    file: u8
}

impl Default for Place {
    fn default() -> Self {
        Place { row: 0, file: 0 }
    }
}

pub enum Color {
    White,
    Black
}

impl Default for Color {
    fn default() -> Self {
        Self::White
    }
}

pub enum PieceTypes {
    Pawn,
    Rock,
    Bishop,
    Knight,
    Queen,
    King
}

impl Default for PieceTypes {
    fn default() -> Self {
        PieceTypes::Pawn
    }
}

pub struct Piece {
    piece_type: PieceTypes,
    place: Place,
    color: Color,
    last_moved: u64
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

pub enum Square {
    Empty,
    Piece(Piece)
}

impl Default for Square {
    fn default() -> Self {
        Self::Empty
    }
}

struct Position {
    turn: Color,
    squares: [[Square; 8]; 8]
}