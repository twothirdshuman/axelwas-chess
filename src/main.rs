mod chess;

fn main() {
    let position = chess::Position::default();
    position.print_position();
    //println!("{:?}", position);
    println!("");
    println!("");
    let moves = position.all_moves_from(chess::Color::White);
    for m in moves {
        println!("{:?}", m);
        position.clone().execute_move(m).unwrap().print_position();
        println!();

    }
}
