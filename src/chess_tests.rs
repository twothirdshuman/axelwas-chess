use std::{collections::HashSet, hash::Hash};

use crate::chess::{self, Color::{self, *}, Move, Piece, PieceTypes::{self, *}, Place, Position};

#[test]
fn setup() {
    assert_eq!(2 + 2, 4);
}

fn same_unique_elements<T: Hash + Eq>(a: &[T], b: &[T]) -> bool {
    let set_a: HashSet<_> = a.iter().collect();
    let set_b: HashSet<_> = b.iter().collect();
    set_a == set_b
}

#[test]
fn default_position() {
    let setup = vec![Piece { piece_type: Rock, place: Place { row: 0, file: 0 }, color: Black, last_moved: 0 }, Piece { piece_type: Knight, place: Place { row: 0, file: 1 }, color: Black, last_moved: 0 }, Piece { piece_type: Bishop, place: Place { row: 0, file: 2 }, color: Black, last_moved: 0 }, Piece { piece_type: Queen, place: Place { row: 0, file: 3 }, color: Black, last_moved: 0 }, Piece { piece_type: King, place: Place { row: 0, file: 4 }, color: Black, last_moved: 0 }, Piece { piece_type: Bishop, place:Place { row: 0, file: 5 }, color: Black, last_moved: 0 }, Piece { piece_type: Knight, place: Place { row: 0, file: 6 }, color: Black, last_moved: 0 }, Piece { piece_type: Rock, place: Place { row: 0, file: 7 }, color: Black, last_moved: 0 }, Piece { piece_type: Pawn { passantable: false }, place: Place { row: 1, file: 0 }, color: Black, last_moved: 0 }, Piece { piece_type: Pawn { passantable: false }, place: Place { row: 1, file: 1 }, color: Black, last_moved: 0 }, Piece { piece_type: Pawn { passantable: false }, place: Place { row: 1, file: 2 }, color: Black, last_moved: 0 }, Piece { piece_type: Pawn { passantable: false }, place: Place { row: 1, file: 3 }, color: Black, last_moved: 0 }, Piece { piece_type: Pawn { passantable: false }, place: Place { row: 1, file: 4 }, color: Black, last_moved: 0 }, Piece { piece_type: Pawn { passantable: false }, place: Place { row: 1, file: 5 }, color: Black, last_moved: 0 }, Piece { piece_type: Pawn { passantable: false }, place: Place { row: 1, file: 6 }, color: Black, last_moved: 0 }, Piece { piece_type: Pawn { passantable: false }, place: Place { row: 1, file: 7 }, color: Black, last_moved: 0 }, Piece { piece_type: Pawn { passantable: false }, place: Place { row: 6, file: 0 }, color: White, last_moved: 0 }, Piece { piece_type: Pawn { passantable: false }, place: Place { row: 6, file: 1 }, color: White, last_moved: 0 }, Piece { piece_type: Pawn { passantable: false }, place: Place { row: 6, file: 2 }, color: White, last_moved: 0 },Piece { piece_type: Pawn { passantable: false }, place: Place { row: 6, file: 3 }, color: White, last_moved: 0 }, Piece { piece_type: Pawn { passantable: false }, place: Place { row: 6, file: 4 }, color: White, last_moved: 0 }, Piece { piece_type: Pawn { passantable: false }, place: Place { row: 6, file: 5 }, color: White, last_moved: 0 }, Piece { piece_type: Pawn { passantable: false }, place: Place { row: 6, file: 6 }, color: White, last_moved: 0 }, Piece { piece_type: Pawn { passantable: false }, place: Place { row: 6, file: 7 }, color: White, last_moved: 0 }, Piece { piece_type: Rock, place: Place { row: 7, file: 0 }, color: White, last_moved: 0 }, Piece { piece_type: Knight, place: Place { row: 7, file: 1 }, color: White, last_moved: 0 }, Piece { piece_type: Bishop, place: Place { row: 7, file: 2 },color: White, last_moved: 0 }, Piece { piece_type: Queen, place: Place { row: 7, file: 3 }, color: White, last_moved: 0 }, Piece { piece_type: King, place: Place { row: 7, file: 4 }, color: White, last_moved: 0 }, Piece { piece_type: Bishop, place: Place { row: 7, file: 5 }, color: White, last_moved: 0 }, Piece { piece_type: Knight, place: Place { row: 7, file: 6 }, color: White, last_moved: 0 }, Piece {piece_type: Rock, place: Place { row: 7, file: 7 }, color: White, last_moved: 0 }];
    let default = chess::Position::default();
    assert!(same_unique_elements(default.pieces.as_slice(), setup.as_slice()));
}

#[test]

fn default_moves() {
    let moves = [Move { from: Place { row: 6, file: 0 }, to: Place { row: 5, file: 0 }, promotion: None }, Move { from: Place { row: 6, file: 0 }, to: Place { row: 4, file: 0 }, promotion: None }, Move { from: Place { row: 6, file: 1 }, to: Place { row: 5, file: 1 }, promotion: None }, Move { from: Place { row: 6, file: 1 }, to: Place { row: 4, file: 1 }, promotion: None }, Move { from: Place { row: 6, file: 2 }, to: Place { row: 5, file: 2 }, promotion: None }, Move { from: Place { row: 6, file: 2 }, to: Place { row: 4, file: 2 }, promotion: None }, Move { from: Place { row: 6, file: 3 }, to: Place { row: 5, file: 3 }, promotion: None }, Move { from: Place { row: 6, file: 3 }, to: Place { row: 4, file: 3 }, promotion: None }, Move { from: Place { row: 6, file: 4 }, to: Place { row: 5, file: 4 }, promotion: None }, Move { from: Place { row: 6, file: 4 }, to: Place { row: 4, file: 4 }, promotion: None }, Move { from: Place { row: 6, file: 5 }, to: Place { row: 5, file: 5 }, promotion: None }, Move { from: Place { row: 6, file: 5 }, to: Place { row: 4, file: 5 }, promotion: None }, Move { from: Place { row: 6, file: 6 }, to: Place { row: 5, file: 6 }, promotion: None }, Move { from: Place { row: 6, file: 6 }, to: Place { row: 4, file: 6 }, promotion: None }, Move { from: Place { row: 6, file: 7 }, to: Place { row: 5, file: 7 }, promotion: None }, Move { from: Place { row: 6, file: 7 }, to: Place { row: 4, file: 7 }, promotion: None }, Move { from: Place { row: 7, file: 1 }, to: Place { row: 5, file: 0 }, promotion: None }, Move { from: Place { row: 7, file: 1 }, to: Place { row: 5, file: 2 }, promotion: None }, Move { from: Place { row: 7, file: 6 }, to: Place { row: 5, file: 5 }, promotion: None }, Move { from: Place { row: 7, file: 6 }, to: Place { row: 5, file: 7 }, promotion: None }];
    let pos = chess::Position::default();
    assert!(same_unique_elements(pos.all_moves_from(Color::White).as_slice(), &moves));
}

#[test]
fn pawn_capture() {
    let pos = Position::from_fen("7k/8/8/5p2/6P1/8/8/7K w - - 0 1").expect("this is a valid fen");
    let moves = [Move { from: Place { row: 4, file: 6 }, to: Place { row: 3, file: 6 }, promotion: None }, Move { from: Place { row: 4, file: 6 }, to: Place { row: 3, file: 5 }, promotion: None }, Move { from: Place { row: 7, file: 7 }, to: Place { row: 7, file: 6 }, promotion: None }, Move { from: Place { row: 7, file: 7 }, to: Place { row: 6, file: 6 }, promotion: None }, Move { from: Place { row: 7, file: 7 }, to: Place { row: 6, file: 7 }, promotion: None }];

    assert!(same_unique_elements(&moves, pos.all_moves_from(Color::White).as_slice()));
}

#[test]
fn test_passant() {
    let pos = Position::default();

    let pos = pos.execute_move_checked( Place::from_str("e2").unwrap().goto(&Place::from_str("e4").unwrap()));
    let pos = pos.execute_move_checked(Place::from_str("g8").unwrap().goto(&Place::from_str("f6").unwrap()));
    let pos = pos.execute_move_checked(Place::from_str("e4").unwrap().goto(&Place::from_str("e5").unwrap()));
    let pos = pos.execute_move_checked(Place::from_str("d7").unwrap().goto(&Place::from_str("d5").unwrap()));
    let pos = pos.execute_move_checked(Place::from_str("e5").unwrap().goto(&Place::from_str("d6").unwrap()));

    let correct = [Piece { piece_type: Rock, place: Place { row: 0, file: 0 }, color: Black, last_moved: 0 }, Piece { piece_type: Knight, place: Place { row: 0, file: 1 }, color: Black, last_moved: 0 }, Piece { piece_type: Bishop, place: Place { row: 0, file: 2 }, color: Black, last_moved: 0 }, Piece { piece_type: Queen, place: Place { row: 0, file: 3 }, color: Black, last_moved: 0 }, Piece { piece_type: King, place: Place { row: 0, file: 4 }, color: Black, last_moved: 0 }, Piece { piece_type: Bishop, place: Place { row: 0, file: 5 }, color: Black, last_moved: 0 }, Piece { piece_type: Knight, place: Place { row: 2, file: 5 }, color: Black, last_moved: 2 }, Piece { piece_type: Rock, place: Place { row: 0, file: 7 }, color: Black, last_moved: 0 }, Piece { piece_type: Pawn { passantable: false }, place: Place { row: 1, file: 0 }, color: Black, last_moved: 0 }, Piece { piece_type: Pawn { passantable: false }, place: Place { row: 1, file: 1 }, color: Black, last_moved: 0 }, Piece { piece_type: Pawn { passantable: false }, place: Place { row: 1, file: 2 }, color: Black, last_moved: 0 }, Piece { piece_type: Pawn { passantable: false }, place: Place { row: 1, file: 4 }, color: Black, last_moved: 0 }, Piece { piece_type: Pawn { passantable: false }, place: Place { row: 1, file: 5 }, color: Black, last_moved: 0 }, Piece { piece_type: Pawn { passantable: false }, place: Place { row: 1, file: 6 }, color: Black, last_moved: 0 }, Piece { piece_type: Pawn { passantable: false }, place: Place { row: 1, file: 7 }, color: Black, last_moved: 0 }, Piece { piece_type: Pawn { passantable: false }, place: Place { row: 6, file: 0 }, color: White, last_moved: 0 }, Piece { piece_type: Pawn { passantable: false }, place: Place { row: 6, file: 1 }, color: White, last_moved: 0 }, Piece { piece_type: Pawn { passantable: false }, place: Place { row: 6, file: 2 }, color: White, last_moved: 0 }, Piece { piece_type: Pawn { passantable: false }, place: Place { row: 6, file: 3 }, color: White, last_moved: 0 }, Piece { piece_type: Pawn { passantable: false }, place: Place { row: 2, file: 3 }, color: White, last_moved: 5 }, Piece { piece_type: Pawn { passantable: false }, place: Place { row: 6, file: 5 }, color: White, last_moved: 0 }, Piece { piece_type: Pawn { passantable: false }, place: Place { row: 6, file: 6 }, color: White, last_moved: 0 }, Piece { piece_type: Pawn { passantable: false }, place: Place { row: 6, file: 7 }, color: White, last_moved: 0 }, Piece { piece_type: Rock, place: Place { row: 7, file: 0 }, color: White, last_moved: 0 }, Piece { piece_type: Knight, place: Place { row: 7, file: 1 }, color: White, last_moved: 0 }, Piece { piece_type: Bishop, place: Place { row: 7, file: 2 }, color: White, last_moved: 0 }, Piece { piece_type: Queen, place: Place { row: 7, file: 3 }, color: White, last_moved: 0 }, Piece { piece_type: King, place: Place { row: 7, file: 4 }, color: White, last_moved: 0 }, Piece { piece_type: Bishop, place: Place { row: 7, file: 5 }, color: White, last_moved: 0 }, Piece { piece_type: Knight, place: Place { row: 7, file: 6 }, color: White, last_moved: 0 }, Piece { piece_type: Rock, place: Place { row: 7, file: 7 }, color: White, last_moved: 0 }];
    assert!(same_unique_elements(&correct, &pos.pieces));
}

#[test]
fn test_promotion() {
    let pos = Position::default();

    let pos = pos.execute_move_checked(Move::from_str("e2e4").unwrap());
    let pos = pos.execute_move_checked(Move::from_str("d7d5").unwrap());
    let pos = pos.execute_move_checked(Move::from_str("e4d5").unwrap());
    let pos = pos.execute_move_checked(Move::from_str("b8c6").unwrap());
    let pos = pos.execute_move_checked(Move::from_str("d5c6").unwrap());
    let pos = pos.execute_move_checked(Move::from_str("e7e5").unwrap());
    let pos = pos.execute_move_checked(Move::from_str("c6b7").unwrap());
    let pos = pos.execute_move_checked(Move::from_str("e5e4").unwrap());
    let pos = pos.execute_move_checked(Move {from: Place::from_str("b7").unwrap(), to: Place::from_str("a8").unwrap(), promotion: Some(PieceTypes::Queen)});
    
    let correct = [Piece { piece_type: Bishop, place: Place { row: 0, file: 2 }, color: Black, last_moved: 0 }, Piece { piece_type: Queen, place: Place { row: 0, file: 3 }, color: Black, last_moved: 0 }, Piece { piece_type: King, place: Place { row: 0, file: 4 }, color: Black, last_moved: 0 }, Piece { piece_type: Bishop, place: Place { row: 0, file: 5 }, color: Black, last_moved: 0 }, Piece { piece_type: Knight, place: Place { row: 0, file: 6 }, color: Black, last_moved: 0 }, Piece { piece_type: Rock, place: Place { row: 0, file: 7 }, color: Black, last_moved: 0 }, Piece { piece_type: Pawn { passantable: false }, place: Place { row: 1, file: 0 }, color: Black, last_moved: 0 }, Piece { piece_type: Pawn { passantable: false }, place: Place { row: 1, file: 2 }, color: Black, last_moved: 0 }, Piece { piece_type: Pawn { passantable: false }, place: Place { row: 4, file: 4 }, color: Black, last_moved: 8 }, Piece { piece_type: Pawn { passantable: false }, place: Place { row: 1, file: 5 }, color: Black, last_moved: 0 }, Piece { piece_type: Pawn { passantable: false }, place: Place { row: 1, file: 6 }, color: Black, last_moved: 0 }, Piece { piece_type: Pawn { passantable: false }, place: Place { row: 1, file: 7 }, color: Black, last_moved: 0 }, Piece { piece_type: Pawn { passantable: false }, place: Place { row: 6, file: 0 }, color: White, last_moved: 0 }, Piece { piece_type: Pawn { passantable: false }, place: Place { row: 6, file: 1 }, color: White, last_moved: 0 }, Piece { piece_type: Pawn { passantable: false }, place: Place { row: 6, file: 2 }, color: White, last_moved: 0 }, Piece { piece_type: Pawn { passantable: false }, place: Place { row: 6, file: 3 }, color: White, last_moved: 0 }, Piece { piece_type: Queen, place: Place { row: 0, file: 0 }, color: White, last_moved: 9 }, Piece { piece_type: Pawn { passantable: false }, place: Place { row: 6, file: 5 }, color: White, last_moved: 0 }, Piece { piece_type: Pawn { passantable: false }, place: Place { row: 6, file: 6 }, color: White, last_moved: 0 }, Piece { piece_type: Pawn { passantable: false }, place: Place { row: 6, file: 7 }, color: White, last_moved: 0 }, Piece { piece_type: Rock, place: Place { row: 7, file: 0 }, color: White, last_moved: 0 }, Piece { piece_type: Knight, place: Place { row: 7, file: 1 }, color: White, last_moved: 0 }, Piece { piece_type: Bishop, place: Place { row: 7, file: 2 }, color: White, last_moved: 0 }, Piece { piece_type: Queen, place: Place { row: 7, file: 3 }, color: White, last_moved: 0 }, Piece { piece_type: King, place: Place { row: 7, file: 4 }, color: White, last_moved: 0 }, Piece { piece_type: Bishop, place: Place { row: 7, file: 5 }, color: White, last_moved: 0 }, Piece { piece_type: Knight, place: Place { row: 7, file: 6 }, color: White, last_moved: 0 }, Piece { piece_type: Rock, place: Place { row: 7, file: 7 }, color: White, last_moved: 0 }];
    assert!(same_unique_elements(&correct, &pos.pieces));
}

#[test]
fn test_castling() {
    let pos = Position::default();

    let pos = pos.execute_move_checked(Move::from_str("e2e4").unwrap());
    let pos = pos.execute_move_checked(Move::from_str("d7d5").unwrap());
    let pos = pos.execute_move_checked(Move::from_str("e4d5").unwrap());
    let pos = pos.execute_move_checked(Move::from_str("d8d5").unwrap());
    pos.print_position();
    let pos = pos.execute_move_checked(Move::from_str("f1d3").unwrap());
    let pos = pos.execute_move_checked(Move::from_str("c8d7").unwrap());
    let pos = pos.execute_move_checked(Move::from_str("g1f3").unwrap());
    let pos = pos.execute_move_checked(Move::from_str("b8c6").unwrap());
    let pos = pos.execute_move_checked(Move::from_str("e1g1").unwrap());
    let pos = pos.execute_move_checked(Move::from_str("e8c8").unwrap());
    
    pos.print_position();
}