use std::{iter, slice::Iter};

#[derive(Clone, Debug)]
pub struct Place {
    row: usize,
    file: usize
}

impl Default for Place {
    fn default() -> Self {
        Place { row: 0, file: 0 }
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

#[derive(Clone, Debug)]
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

#[derive(Debug)]
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

impl Piece {
    fn create(piece_type: PieceTypes, place: Place, color: Color) -> Self {
        Self {
            last_moved: 0,
            piece_type,
            place,
            color
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

#[derive(Debug)]
pub struct Position {
    turn: Color,
    pieces: Vec<Piece>
}



impl Position {

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
            'p' => vec![Some((Color::Black, PieceTypes::Pawn))],
            'R' => vec![Some((Color::White, PieceTypes::Rock))],
            'N' => vec![Some((Color::White, PieceTypes::Knight))],
            'B' => vec![Some((Color::White, PieceTypes::Bishop))],
            'Q' => vec![Some((Color::White, PieceTypes::Queen))],
            'K' => vec![Some((Color::White, PieceTypes::King))],
            'P' => vec![Some((Color::White, PieceTypes::Pawn))],
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