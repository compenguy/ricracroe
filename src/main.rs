mod ricracroe;

fn next_player(player: &mut ricracroe::RRRCell) -> Result<ricracroe::RRRCell, &'static str> {
    match *player {
        ricracroe::RRRCell::X     => {
            *player = ricracroe::RRRCell::O;
            Ok(*player)
        }
        ricracroe::RRRCell::O     => {
            *player = ricracroe::RRRCell::X;
            Ok(*player)
        }
        ricracroe::RRRCell::Clear => Err("Game does not appear to be in progress."),
    }
}

fn take_turn(board: &mut ricracroe::RRRBoard, x: usize, y: usize, player: &mut ricracroe::RRRCell) -> Option<ricracroe::RRROutcome> {
    // TODO: Right now, just being lazy and allowing errors to panic
    match board.make_move(x, y, *player) {
        Ok(_) => println!("{} played in {}, {}:\n{}", player, x, y, board),
        Err(e) => {
            println!("{} attempted to play in {}, {}:\n{}", player, x, y, board);
            println!("Something went wrong: {}", e);
            return None;
        }
    }

    let outcome = board.outcome();
    match outcome {
        Some(ricracroe::RRROutcome::XWins{ winning_cells: _ }) => { println!("X Wins!"); },
        Some(ricracroe::RRROutcome::OWins{ winning_cells: _ }) => { println!("O Wins!"); },
        Some(ricracroe::RRROutcome::Draw) => { println!("It's a draw!"); },
        None => { next_player(player).unwrap(); },
    }
    outcome
}

fn main() {
    let mut board = ricracroe::RRRBoard::new_anysize(6);
    let mut current_player = ricracroe::RRRCell::X;

    println!("Starting board:\n{}", board);
    match take_turn(&mut board, 0, 0, &mut current_player) {
        Some(_) => return,
        None => {},
    }

    match take_turn(&mut board, 0, 1, &mut current_player) {
        Some(_) => return,
        None => {},
    }

    match take_turn(&mut board, 2, 2, &mut current_player) {
        Some(_) => return,
        None => {},
    }

    match take_turn(&mut board, 2, 2, &mut current_player) {
        Some(_) => return,
        None => {},
    }

    match take_turn(&mut board, 2, 1, &mut current_player) {
        Some(_) => return,
        None => {},
    }

    match take_turn(&mut board, 1, 1, &mut current_player) {
        Some(_) => return,
        None => {},
    }
}
