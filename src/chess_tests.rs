use std::{collections::HashSet, hash::Hash};

use crate::{ Color::{self, *}, EndStates, Move, Piece, PieceTypes::{self, *}, Place, Position};

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
    let setup = vec![Piece { piece_type: Rook, place: Place { row: 0, file: 0 }, color: Black, last_moved: 0 }, Piece { piece_type: Knight, place: Place { row: 0, file: 1 }, color: Black, last_moved: 0 }, Piece { piece_type: Bishop, place: Place { row: 0, file: 2 }, color: Black, last_moved: 0 }, Piece { piece_type: Queen, place: Place { row: 0, file: 3 }, color: Black, last_moved: 0 }, Piece { piece_type: King, place: Place { row: 0, file: 4 }, color: Black, last_moved: 0 }, Piece { piece_type: Bishop, place:Place { row: 0, file: 5 }, color: Black, last_moved: 0 }, Piece { piece_type: Knight, place: Place { row: 0, file: 6 }, color: Black, last_moved: 0 }, Piece { piece_type: Rook, place: Place { row: 0, file: 7 }, color: Black, last_moved: 0 }, Piece { piece_type: Pawn { passantable: false }, place: Place { row: 1, file: 0 }, color: Black, last_moved: 0 }, Piece { piece_type: Pawn { passantable: false }, place: Place { row: 1, file: 1 }, color: Black, last_moved: 0 }, Piece { piece_type: Pawn { passantable: false }, place: Place { row: 1, file: 2 }, color: Black, last_moved: 0 }, Piece { piece_type: Pawn { passantable: false }, place: Place { row: 1, file: 3 }, color: Black, last_moved: 0 }, Piece { piece_type: Pawn { passantable: false }, place: Place { row: 1, file: 4 }, color: Black, last_moved: 0 }, Piece { piece_type: Pawn { passantable: false }, place: Place { row: 1, file: 5 }, color: Black, last_moved: 0 }, Piece { piece_type: Pawn { passantable: false }, place: Place { row: 1, file: 6 }, color: Black, last_moved: 0 }, Piece { piece_type: Pawn { passantable: false }, place: Place { row: 1, file: 7 }, color: Black, last_moved: 0 }, Piece { piece_type: Pawn { passantable: false }, place: Place { row: 6, file: 0 }, color: White, last_moved: 0 }, Piece { piece_type: Pawn { passantable: false }, place: Place { row: 6, file: 1 }, color: White, last_moved: 0 }, Piece { piece_type: Pawn { passantable: false }, place: Place { row: 6, file: 2 }, color: White, last_moved: 0 },Piece { piece_type: Pawn { passantable: false }, place: Place { row: 6, file: 3 }, color: White, last_moved: 0 }, Piece { piece_type: Pawn { passantable: false }, place: Place { row: 6, file: 4 }, color: White, last_moved: 0 }, Piece { piece_type: Pawn { passantable: false }, place: Place { row: 6, file: 5 }, color: White, last_moved: 0 }, Piece { piece_type: Pawn { passantable: false }, place: Place { row: 6, file: 6 }, color: White, last_moved: 0 }, Piece { piece_type: Pawn { passantable: false }, place: Place { row: 6, file: 7 }, color: White, last_moved: 0 }, Piece { piece_type: Rook, place: Place { row: 7, file: 0 }, color: White, last_moved: 0 }, Piece { piece_type: Knight, place: Place { row: 7, file: 1 }, color: White, last_moved: 0 }, Piece { piece_type: Bishop, place: Place { row: 7, file: 2 },color: White, last_moved: 0 }, Piece { piece_type: Queen, place: Place { row: 7, file: 3 }, color: White, last_moved: 0 }, Piece { piece_type: King, place: Place { row: 7, file: 4 }, color: White, last_moved: 0 }, Piece { piece_type: Bishop, place: Place { row: 7, file: 5 }, color: White, last_moved: 0 }, Piece { piece_type: Knight, place: Place { row: 7, file: 6 }, color: White, last_moved: 0 }, Piece {piece_type: Rook, place: Place { row: 7, file: 7 }, color: White, last_moved: 0 }];
    let default = Position::default();
    assert!(same_unique_elements(default.pieces.as_slice(), setup.as_slice()));
}

#[test]

fn default_moves() {
    let moves = [Move { from: Place { row: 6, file: 0 }, to: Place { row: 5, file: 0 }, promotion: None }, Move { from: Place { row: 6, file: 0 }, to: Place { row: 4, file: 0 }, promotion: None }, Move { from: Place { row: 6, file: 1 }, to: Place { row: 5, file: 1 }, promotion: None }, Move { from: Place { row: 6, file: 1 }, to: Place { row: 4, file: 1 }, promotion: None }, Move { from: Place { row: 6, file: 2 }, to: Place { row: 5, file: 2 }, promotion: None }, Move { from: Place { row: 6, file: 2 }, to: Place { row: 4, file: 2 }, promotion: None }, Move { from: Place { row: 6, file: 3 }, to: Place { row: 5, file: 3 }, promotion: None }, Move { from: Place { row: 6, file: 3 }, to: Place { row: 4, file: 3 }, promotion: None }, Move { from: Place { row: 6, file: 4 }, to: Place { row: 5, file: 4 }, promotion: None }, Move { from: Place { row: 6, file: 4 }, to: Place { row: 4, file: 4 }, promotion: None }, Move { from: Place { row: 6, file: 5 }, to: Place { row: 5, file: 5 }, promotion: None }, Move { from: Place { row: 6, file: 5 }, to: Place { row: 4, file: 5 }, promotion: None }, Move { from: Place { row: 6, file: 6 }, to: Place { row: 5, file: 6 }, promotion: None }, Move { from: Place { row: 6, file: 6 }, to: Place { row: 4, file: 6 }, promotion: None }, Move { from: Place { row: 6, file: 7 }, to: Place { row: 5, file: 7 }, promotion: None }, Move { from: Place { row: 6, file: 7 }, to: Place { row: 4, file: 7 }, promotion: None }, Move { from: Place { row: 7, file: 1 }, to: Place { row: 5, file: 0 }, promotion: None }, Move { from: Place { row: 7, file: 1 }, to: Place { row: 5, file: 2 }, promotion: None }, Move { from: Place { row: 7, file: 6 }, to: Place { row: 5, file: 5 }, promotion: None }, Move { from: Place { row: 7, file: 6 }, to: Place { row: 5, file: 7 }, promotion: None }];
    let pos = Position::default();
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

    let pos = pos.execute_move_checked( Place::try_from("e2").unwrap().goto(&Place::try_from("e4").unwrap()));
    let pos = pos.execute_move_checked(Place::try_from("g8").unwrap().goto(&Place::try_from("f6").unwrap()));
    let pos = pos.execute_move_checked(Place::try_from("e4").unwrap().goto(&Place::try_from("e5").unwrap()));
    let pos = pos.execute_move_checked(Place::try_from("d7").unwrap().goto(&Place::try_from("d5").unwrap()));
    let pos = pos.execute_move_checked(Place::try_from("e5").unwrap().goto(&Place::try_from("d6").unwrap()));

    let correct = [Piece { piece_type: Rook, place: Place { row: 0, file: 0 }, color: Black, last_moved: 0 }, Piece { piece_type: Knight, place: Place { row: 0, file: 1 }, color: Black, last_moved: 0 }, Piece { piece_type: Bishop, place: Place { row: 0, file: 2 }, color: Black, last_moved: 0 }, Piece { piece_type: Queen, place: Place { row: 0, file: 3 }, color: Black, last_moved: 0 }, Piece { piece_type: King, place: Place { row: 0, file: 4 }, color: Black, last_moved: 0 }, Piece { piece_type: Bishop, place: Place { row: 0, file: 5 }, color: Black, last_moved: 0 }, Piece { piece_type: Knight, place: Place { row: 2, file: 5 }, color: Black, last_moved: 2 }, Piece { piece_type: Rook, place: Place { row: 0, file: 7 }, color: Black, last_moved: 0 }, Piece { piece_type: Pawn { passantable: false }, place: Place { row: 1, file: 0 }, color: Black, last_moved: 0 }, Piece { piece_type: Pawn { passantable: false }, place: Place { row: 1, file: 1 }, color: Black, last_moved: 0 }, Piece { piece_type: Pawn { passantable: false }, place: Place { row: 1, file: 2 }, color: Black, last_moved: 0 }, Piece { piece_type: Pawn { passantable: false }, place: Place { row: 1, file: 4 }, color: Black, last_moved: 0 }, Piece { piece_type: Pawn { passantable: false }, place: Place { row: 1, file: 5 }, color: Black, last_moved: 0 }, Piece { piece_type: Pawn { passantable: false }, place: Place { row: 1, file: 6 }, color: Black, last_moved: 0 }, Piece { piece_type: Pawn { passantable: false }, place: Place { row: 1, file: 7 }, color: Black, last_moved: 0 }, Piece { piece_type: Pawn { passantable: false }, place: Place { row: 6, file: 0 }, color: White, last_moved: 0 }, Piece { piece_type: Pawn { passantable: false }, place: Place { row: 6, file: 1 }, color: White, last_moved: 0 }, Piece { piece_type: Pawn { passantable: false }, place: Place { row: 6, file: 2 }, color: White, last_moved: 0 }, Piece { piece_type: Pawn { passantable: false }, place: Place { row: 6, file: 3 }, color: White, last_moved: 0 }, Piece { piece_type: Pawn { passantable: false }, place: Place { row: 2, file: 3 }, color: White, last_moved: 5 }, Piece { piece_type: Pawn { passantable: false }, place: Place { row: 6, file: 5 }, color: White, last_moved: 0 }, Piece { piece_type: Pawn { passantable: false }, place: Place { row: 6, file: 6 }, color: White, last_moved: 0 }, Piece { piece_type: Pawn { passantable: false }, place: Place { row: 6, file: 7 }, color: White, last_moved: 0 }, Piece { piece_type: Rook, place: Place { row: 7, file: 0 }, color: White, last_moved: 0 }, Piece { piece_type: Knight, place: Place { row: 7, file: 1 }, color: White, last_moved: 0 }, Piece { piece_type: Bishop, place: Place { row: 7, file: 2 }, color: White, last_moved: 0 }, Piece { piece_type: Queen, place: Place { row: 7, file: 3 }, color: White, last_moved: 0 }, Piece { piece_type: King, place: Place { row: 7, file: 4 }, color: White, last_moved: 0 }, Piece { piece_type: Bishop, place: Place { row: 7, file: 5 }, color: White, last_moved: 0 }, Piece { piece_type: Knight, place: Place { row: 7, file: 6 }, color: White, last_moved: 0 }, Piece { piece_type: Rook, place: Place { row: 7, file: 7 }, color: White, last_moved: 0 }];
    assert!(same_unique_elements(&correct, &pos.pieces));
}

#[test]
fn test_promotion() {
    let pos = Position::default();

    let pos = pos.execute_move_checked(Move::try_from("e2e4").unwrap());
    let pos = pos.execute_move_checked(Move::try_from("d7d5").unwrap());
    let pos = pos.execute_move_checked(Move::try_from("e4d5").unwrap());
    let pos = pos.execute_move_checked(Move::try_from("b8c6").unwrap());
    let pos = pos.execute_move_checked(Move::try_from("d5c6").unwrap());
    let pos = pos.execute_move_checked(Move::try_from("e7e5").unwrap());
    let pos = pos.execute_move_checked(Move::try_from("c6b7").unwrap());
    let pos = pos.execute_move_checked(Move::try_from("e5e4").unwrap());
    let pos = pos.execute_move_checked(Move {from: Place::try_from("b7").unwrap(), to: Place::try_from("a8").unwrap(), promotion: Some(PieceTypes::Queen)});
    
    let correct = [Piece { piece_type: Bishop, place: Place { row: 0, file: 2 }, color: Black, last_moved: 0 }, Piece { piece_type: Queen, place: Place { row: 0, file: 3 }, color: Black, last_moved: 0 }, Piece { piece_type: King, place: Place { row: 0, file: 4 }, color: Black, last_moved: 0 }, Piece { piece_type: Bishop, place: Place { row: 0, file: 5 }, color: Black, last_moved: 0 }, Piece { piece_type: Knight, place: Place { row: 0, file: 6 }, color: Black, last_moved: 0 }, Piece { piece_type: Rook, place: Place { row: 0, file: 7 }, color: Black, last_moved: 0 }, Piece { piece_type: Pawn { passantable: false }, place: Place { row: 1, file: 0 }, color: Black, last_moved: 0 }, Piece { piece_type: Pawn { passantable: false }, place: Place { row: 1, file: 2 }, color: Black, last_moved: 0 }, Piece { piece_type: Pawn { passantable: false }, place: Place { row: 4, file: 4 }, color: Black, last_moved: 8 }, Piece { piece_type: Pawn { passantable: false }, place: Place { row: 1, file: 5 }, color: Black, last_moved: 0 }, Piece { piece_type: Pawn { passantable: false }, place: Place { row: 1, file: 6 }, color: Black, last_moved: 0 }, Piece { piece_type: Pawn { passantable: false }, place: Place { row: 1, file: 7 }, color: Black, last_moved: 0 }, Piece { piece_type: Pawn { passantable: false }, place: Place { row: 6, file: 0 }, color: White, last_moved: 0 }, Piece { piece_type: Pawn { passantable: false }, place: Place { row: 6, file: 1 }, color: White, last_moved: 0 }, Piece { piece_type: Pawn { passantable: false }, place: Place { row: 6, file: 2 }, color: White, last_moved: 0 }, Piece { piece_type: Pawn { passantable: false }, place: Place { row: 6, file: 3 }, color: White, last_moved: 0 }, Piece { piece_type: Queen, place: Place { row: 0, file: 0 }, color: White, last_moved: 9 }, Piece { piece_type: Pawn { passantable: false }, place: Place { row: 6, file: 5 }, color: White, last_moved: 0 }, Piece { piece_type: Pawn { passantable: false }, place: Place { row: 6, file: 6 }, color: White, last_moved: 0 }, Piece { piece_type: Pawn { passantable: false }, place: Place { row: 6, file: 7 }, color: White, last_moved: 0 }, Piece { piece_type: Rook, place: Place { row: 7, file: 0 }, color: White, last_moved: 0 }, Piece { piece_type: Knight, place: Place { row: 7, file: 1 }, color: White, last_moved: 0 }, Piece { piece_type: Bishop, place: Place { row: 7, file: 2 }, color: White, last_moved: 0 }, Piece { piece_type: Queen, place: Place { row: 7, file: 3 }, color: White, last_moved: 0 }, Piece { piece_type: King, place: Place { row: 7, file: 4 }, color: White, last_moved: 0 }, Piece { piece_type: Bishop, place: Place { row: 7, file: 5 }, color: White, last_moved: 0 }, Piece { piece_type: Knight, place: Place { row: 7, file: 6 }, color: White, last_moved: 0 }, Piece { piece_type: Rook, place: Place { row: 7, file: 7 }, color: White, last_moved: 0 }];
    assert!(same_unique_elements(&correct, &pos.pieces));
}

#[test]
fn test_castling() {
    let pos = Position::default();

    let pos = pos.execute_move_checked(Move::try_from("e2e4").unwrap());
    let pos = pos.execute_move_checked(Move::try_from("d7d5").unwrap());
    let pos = pos.execute_move_checked(Move::try_from("e4d5").unwrap());
    let pos = pos.execute_move_checked(Move::try_from("d8d5").unwrap());
    let pos = pos.execute_move_checked(Move::try_from("f1d3").unwrap());
    let pos = pos.execute_move_checked(Move::try_from("c8d7").unwrap());
    let pos = pos.execute_move_checked(Move::try_from("g1f3").unwrap());
    let pos = pos.execute_move_checked(Move::try_from("b8c6").unwrap());
    let pos = pos.execute_move_checked(Move::try_from("e1g1").unwrap());
    let pos = pos.execute_move_checked(Move::try_from("e8c8").unwrap());
    
    let correct = [Piece { piece_type: Rook, place: Place { row: 0, file: 3 }, color: Black, last_moved: 9 }, Piece { piece_type: Knight, place: Place { row: 2, file: 2 }, color: Black, last_moved: 8 }, Piece { piece_type: Bishop, place: Place { row: 1, file: 3 }, color: Black, last_moved: 6 }, Piece { piece_type: Queen, place: Place { row: 3, file: 3 }, color: Black, last_moved: 4 }, Piece { piece_type: King, place: Place { row: 0, file: 2 }, color: Black, last_moved: 10 }, Piece { piece_type: Bishop, place: Place { row: 0, file: 5 }, color: Black, last_moved: 0 }, Piece { piece_type: Knight, place: Place { row: 0, file: 6 }, color: Black, last_moved: 0 }, Piece { piece_type: Rook, place: Place { row: 0, file: 7 }, color: Black, last_moved: 0 }, Piece { piece_type: Pawn { passantable: false }, place: Place { row: 1, file: 0 }, color: Black, last_moved: 0 }, Piece { piece_type: Pawn { passantable: false }, place: Place { row: 1, file: 1 }, color: Black, last_moved: 0 }, Piece { piece_type: Pawn { passantable: false }, place: Place { row: 1, file: 2 }, color: Black, last_moved: 0 }, Piece { piece_type: Pawn { passantable: false }, place: Place { row: 1, file: 4 }, color: Black, last_moved: 0 }, Piece { piece_type: Pawn { passantable: false }, place: Place { row: 1, file: 5 }, color: Black, last_moved: 0 }, Piece { piece_type: Pawn { passantable: false }, place: Place { row: 1, file: 6 }, color: Black, last_moved: 0 }, Piece { piece_type: Pawn { passantable: false }, place: Place { row: 1, file: 7 }, color: Black, last_moved: 0 }, Piece { piece_type: Pawn { passantable: false }, place: Place { row: 6, file: 0 }, color: White, last_moved: 0 }, Piece { piece_type: Pawn { passantable: false }, place: Place { row: 6, file: 1 }, color: White, last_moved: 0 }, Piece { piece_type: Pawn { passantable: false }, place: Place { row: 6, file: 2 }, color: White, last_moved: 0 }, Piece { piece_type: Pawn { passantable: false }, place: Place { row: 6, file: 3 }, color: White, last_moved: 0 }, Piece { piece_type: Pawn { passantable: false }, place: Place { row: 6, file: 5 }, color: White, last_moved: 0 }, Piece { piece_type: Pawn { passantable: false }, place: Place { row: 6, file: 6 }, color: White, last_moved: 0 }, Piece { piece_type: Pawn { passantable: false }, place: Place { row: 6, file: 7 }, color: White, last_moved: 0 }, Piece { piece_type: Rook, place: Place { row: 7, file: 0 }, color: White, last_moved: 0 }, Piece { piece_type: Knight, place: Place { row: 7, file: 1 }, color: White, last_moved: 0 }, Piece { piece_type: Bishop, place: Place { row: 7, file: 2 }, color: White, last_moved: 0 }, Piece { piece_type: Queen, place: Place { row: 7, file: 3 }, color: White, last_moved: 0 }, Piece { piece_type: King, place: Place { row: 7, file: 6 }, color: White, last_moved: 9 }, Piece { piece_type: Bishop, place: Place { row: 5, file: 3 }, color: White, last_moved: 5 }, Piece { piece_type: Knight, place: Place { row: 5, file: 5 }, color: White, last_moved: 7 }, Piece { piece_type: Rook, place: Place { row: 7, file: 5 }, color: White, last_moved: 8 }];

    assert!(same_unique_elements(&correct, &pos.pieces));
}

#[test]
fn in_check() {
    let pos = Position::default();

    let pos = pos.execute_move_checked(Move::try_from("e2e4").unwrap());
    let pos = pos.execute_move_checked(Move::try_from("c7c6").unwrap());
    let pos = pos.execute_move_checked(Move::try_from("f1c4").unwrap());
    let pos = pos.execute_move_checked(Move::try_from("g8f6").unwrap());
    let pos = pos.execute_move_checked(Move::try_from("c4f7").unwrap());
    
    let moves = [Move { from: Place { row: 0, file: 4 }, to: Place { row: 1, file: 5 }, promotion: None }];
    assert!(same_unique_elements(&moves, pos.all_moves().as_slice()));


}

#[test]
fn checkmate() {
    let pos = Position::default();

    let pos = pos.execute_move_checked(Move::try_from("e2e4").unwrap());
    let pos = pos.execute_move_checked(Move::try_from("e7e5").unwrap());
    let pos = pos.execute_move_checked(Move::try_from("f1c4").unwrap());
    let pos = pos.execute_move_checked(Move::try_from("b8c6").unwrap());
    let pos = pos.execute_move_checked(Move::try_from("d1h5").unwrap());
    let pos = pos.execute_move_checked(Move::try_from("g8f6").unwrap());
    let pos = pos.execute_move_checked(Move::try_from("h5f7").unwrap());

    assert!(pos.game_end() == EndStates::Checkmate);
}

#[test]
fn stalemate() {
    let pos = Position::from_fen("3k4/1R5P/3N4/4K3/2p2P2/p7/P7/8 w - - 0 51").unwrap();
    println!("{}", pos);
    let pos = pos.execute_move_checked(Move::try_from("d6c4").unwrap());
    println!("{}", pos);
    let pos = pos.execute_move_checked(Move::try_from("d8c8").unwrap());
    println!("{}", pos);
    let pos = pos.execute_move_checked(Move::try_from("c4d6").unwrap());
    println!("{}", pos);
    let pos = pos.execute_move_checked(Move::try_from("c8d8").unwrap());
    println!("{}", pos);
    let pos = pos.execute_move_checked(Move::try_from("e5e6").unwrap());

    println!("{}", pos);

    assert!(pos.game_end() == EndStates::Stalemate);
}

#[test]
fn rook_bishop() {
    let pos = Position::from_fen("4k3/8/8/8/8/8/8/B3K1RB w - - 0 1").unwrap();
    let moves = [Move { from: Place { row: 7, file: 0 }, to: Place { row: 6, file: 1 }, promotion: None }, Move { from: Place { row: 7, file: 0 }, to: Place { row: 5, file: 2 }, promotion: None }, Move { from: Place { row: 7, file: 0 }, to: Place { row: 4, file: 3 }, promotion: None }, Move { from: Place { row: 7, file: 0 }, to: Place { row: 3, file: 4 }, promotion: None }, Move { from: Place { row: 7, file: 0 }, to: Place { row: 2, file: 5 }, promotion: None }, Move { from: Place { row: 7, file: 0 }, to: Place { row: 1, file: 6 }, promotion: None }, Move { from: Place { row: 7, file: 0 }, to: Place { row: 0, file: 7 }, promotion: None }, Move { from: Place { row: 7, file: 0 }, to: Place { row: 0, file: 7 }, promotion: None }, Move { from: Place { row: 7, file: 4 }, to: Place { row: 7, file: 3 }, promotion: None }, Move { from: Place { row: 7, file: 4 }, to: Place { row: 6, file: 3 }, promotion: None }, Move { from: Place { row: 7, file: 4 }, to: Place { row: 6, file: 4 }, promotion: None }, Move { from: Place { row: 7, file: 4 }, to: Place { row: 6, file: 5 }, promotion: None }, Move { from: Place { row: 7, file: 4 }, to: Place { row: 7, file: 5 }, promotion: None }, Move { from: Place { row: 7, file: 6 }, to: Place { row: 6, file: 6 }, promotion: None }, Move { from: Place { row: 7, file: 6 }, to: Place { row: 5, file: 6 }, promotion: None }, Move { from: Place { row: 7, file: 6 }, to: Place { row: 4, file: 6 }, promotion: None }, Move { from: Place { row: 7, file: 6 }, to: Place { row: 3, file: 6 }, promotion: None }, Move { from: Place { row: 7, file: 6 }, to: Place { row: 2, file: 6 }, promotion: None }, Move { from: Place { row: 7, file: 6 }, to: Place { row: 1, file: 6 }, promotion: None }, Move { from: Place { row: 7, file: 6 }, to: Place { row: 0, file: 6 }, promotion: None }, Move { from: Place { row: 7, file: 6 }, to: Place { row: 7, file: 5 }, promotion: None }, Move { from: Place { row: 7, file: 7 }, to: Place { row: 6, file: 6 }, promotion: None }, Move { from: Place { row: 7, file: 7 }, to: Place { row: 5, file: 5 }, promotion: None }, Move { from: Place { row: 7, file: 7 }, to: Place { row: 4, file: 4 }, promotion: None }, Move { from: Place { row: 7, file: 7 }, to: Place { row: 3, file: 3 }, promotion: None }, Move { from: Place { row: 7, file: 7 }, to: Place { row: 2, file: 2 }, promotion: None }, Move { from: Place { row: 7, file: 7 }, to: Place { row: 1, file: 1 }, promotion: None }, Move { from: Place { row: 7, file: 7 }, to: Place { row: 0, file: 0 }, promotion: None }, Move { from: Place { row: 7, file: 7 }, to: Place { row: 0, file: 0 }, promotion: None }];
    assert!(same_unique_elements(&pos.all_moves(), &moves)) 

}

#[test]
fn place_default() {
    assert!(Place::default() == Place { row: 0, file: 0 })
}

#[test]
fn color_default() {
    assert!(Color::default() == Color::White);
}

#[test]
fn doc1() {
    let position = Position::default();
    
    let possible_moves = position.all_moves();
    let new_position = position.execute_move(possible_moves[0]).unwrap();

    println!("{}", new_position);
}

#[test]
fn doc3() {
    let position = Position::default();
    let position = position.execute_move(Move::try_from("e2e4").unwrap()).unwrap();

    println!("{}", position);
}

#[test]
fn doc4() {
    let position = Position::default();
    let piece = position.piece_on(Place::try_from("e2").unwrap());
    println!("{:?}", piece);

}

#[test]
fn doc5() {
    let position = Position::from_fen("3k4/1R5P/3N4/4K3/2p2P2/p7/P7/8 w - - 0 51").unwrap();
    println!("{}", position);
}

#[test]
fn doc6() {
    let position = Position::from_fen("r1bqk1nr/pppp1ppp/2n5/4p3/2B1P3/5N2/PPPP1bPP/RNBQ1RK1 w kq - 0 5").unwrap();

    println!("{:?}", position.in_check(Color::White));
    assert!(position.in_check(Color::White));
} 

#[test]
fn doc7() {
    let position = Position::default();

    println!("{:?}", position.pieces);
} 