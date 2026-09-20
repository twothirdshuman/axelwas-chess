## Structs

This library has a few mains structs.

- ```Position``` which is the state of the board, with piece placements, turn number, and whose turn it is.
- ```Piece``` which is a piece on the board, has info like where it is situated, color, and it's ```PieceType```
- ```PieceTypes``` are the different types of pieces, Pawns, Rocks, King, Queens etc. The Pawn PieceType contains info wether or not it can be captured by en passant.
- ```Color``` The two chess colors black and white.
- ```Place``` A specific square on the board. Can be converted and created from standard chess notation i.e. "e4", "h1" and so on.
- ```Move``` a move from one square to another. Contains information about promotion. Does not contain info about captures which are always implied by the move. 
- ```EndStates``` is a enum of all ways a chess match can end. It either has ended, there's a stalemate or a checkmate.

## Usage

How to create the default setup of the pieces get all moves, make a move, and display the position afterwards.

```rust
fn main() {
    let position = Position::default();
    
    let possible_moves = position.all_moves();
    let new_position = position.execute_move(possible_moves[0]).unwrap();

    println!("{}", new_position);
}
```

To create the default setup of the pieces and then play a whole game provided a pick_move function.

```rust
fn pick_move(position: &Position) -> Move {
    ...
}

fn main() {
    let mut position = Position::default();
    
    while (position.game_end() != EndStates::None) {
        let to_move = pick_move(&position);
        position = position.execute_move(to_move).unwrap(); 
    }
}
```

It is also possible to manually enter moves by their squares. Do note that the execute_move function may not reject illegal Moves. If you want a function that panics on illegal Moves use execute_move_checked.

```rust
fn main() {
    let position = Position::default();
    let position = position.execute_move(Move::from_str("e2e4").unwrap()).unwrap();

    println!("{}", position);

}
```

If you wanna check what piece is on a specific square you can also do that.

```rust 
fn main() {
    let position = Position::default();
    let piece = position.piece_on(Place::from_str("e2").unwrap());
    println!("{:?}", piece);
}
```

Instead of having the standard position you can also use FEN to specify a position. Please note that the parsing of FEN strings isn't exactly implemented to the FEN specification.

```rust 
fn main() {
    let position = Position::from_fen("3k4/1R5P/3N4/4K3/2p2P2/p7/P7/8 w - - 0 51").unwrap();
    position.print_position();
}
```

You can also use this library to check if the king is in check.

```rust
fn main() {
    let position = Position::from_fen("r1bqk1nr/pppp1ppp/2n5/4p3/2B1P3/5N2/PPPP1bPP/RNBQ1RK1 w kq - 0 5").unwrap();

    println!("{}", position.in_check(Color::White));
}
```

If you need all pieces and don't wanna check each square individually with ```Position::piece_on``` you can access the vector pieces on Position.

```rust
fn main() {
    let position = Position::default();

    println!("{:?}", position.pieces);
}