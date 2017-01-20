mod ricracroe;

fn main() {
    let mut board = ricracroe::RRRBoard::new_anysize(8);
    // TODO: Right now, just being lazy and allowing errors to panic
    println!("Starting board:\n{}", board);
    board.make_move(0, 0, ricracroe::RRRCell::X).unwrap();
    println!("After single move:\n{}", board);
    board.make_move(0, 1, ricracroe::RRRCell::O).unwrap();
    println!("After next move:\n{}", board);
    board.make_move(2, 2, ricracroe::RRRCell::X).unwrap();
    println!("After next move:\n{}", board);
    match board.make_move(2, 2, ricracroe::RRRCell::O) {
        Ok(_) => println!("Well, that worked."),
        Err(e) => println!("Something went wrong: {}", e),
    }
    println!("After next move:\n{}", board);
}
