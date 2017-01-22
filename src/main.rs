mod ricracroe;

use std::io::{self, Write};

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
        Ok(_) => println!("{} played in {}, {}:\n{}", player, x+1, y+1, board),
        Err(e) => {
            println!("{} attempted to play in {}, {}:\n{}", player, x+1, y+1, board);
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

/* TODO: templatize? */
fn get_usize_with_prompt(prompt: &str) -> Result<usize, &'static str> {
    let mut buffer = String::new();
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut buffer).unwrap();
    /* parse<usize> yields std::num::ParseIntError s */
    match buffer.trim().parse::<usize>() {
        Ok(ret) => Ok(ret),
        Err(_) => Err("I'm sorry, I didn't catch that."),
    }
}

fn main() {
    let mut board = ricracroe::RRRBoard::new_anysize(3);
    let mut current_player = ricracroe::RRRCell::X;
    let mut outcome = None;

    println!("Welcome to Ric Rac Roe!\n{}", board);
    while outcome == None {
        println!("It's {}'s turn.", current_player);
        /* we print the board with rows and columns 1-indexed */
        let row = get_usize_with_prompt("Row:    ").unwrap() - 1;
        let col = get_usize_with_prompt("Column: ").unwrap() - 1;
        outcome = take_turn(&mut board, col, row, &mut current_player);
    }
}
