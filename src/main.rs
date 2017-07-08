mod ricracroe;

use std::io::{self, Write};
use std::fmt;
use std::error;

use std::num;

extern crate term_cursor;

#[derive(Debug)]
enum RicracroeError {
    Io(io::Error),
    BadParse(num::ParseIntError),
}

impl fmt::Display for RicracroeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            RicracroeError::Io(ref err)       => write!(f, "IO error: {}", err),
            RicracroeError::BadParse(ref err) => write!(f, "Parse error: {}", err),
        }
    }
}

impl error::Error for RicracroeError {
    fn description(&self) -> &str {
        match *self {
            RicracroeError::Io(ref err)       => err.description(),
            RicracroeError::BadParse(ref err) => err.description(),
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            RicracroeError::Io(ref err)       => Some(err),
            RicracroeError::BadParse(ref err) => Some(err),
        }
    }
}

impl From<io::Error> for RicracroeError {
    fn from(err: io::Error) -> RicracroeError {
        RicracroeError::Io(err)
    }
}

impl From<num::ParseIntError> for RicracroeError {
    fn from(err: num::ParseIntError) -> RicracroeError {
        RicracroeError::BadParse(err)
    }
}

/* TODO: templatize? closure for input validation? */
fn get_usize_with_prompt(prompt: &str) -> Result<usize, RicracroeError> {
    let mut buffer = String::new();
    /* TODO: repeat prompt/parse until valid input? */
    print!("{}", prompt);
    try!(io::stdout().flush());
    try!(io::stdin().read_line(&mut buffer));
    /* parse<usize> yields Result<usize, std::num::ParseIntError> */
    buffer.trim().parse::<usize>().map_err(RicracroeError::BadParse)
}

fn main() {
    let mut game = ricracroe::RRRGame::new_anysize(3);

    loop {
        let player = game.get_turn();
        print!("{}", term_cursor::Clear);
        println!("Welcome to Ric Rac Roe!");

        print!("{}", term_cursor::Goto(0,4));
        println!("{}", game.get_board());

        print!("{}", term_cursor::Goto(0,15));
        println!("It's {}'s turn.", player);

        print!("{}", term_cursor::Goto(0,16));
        let row = get_usize_with_prompt("Row:    ").unwrap();

        print!("{}", term_cursor::Goto(0,17));
        let col = get_usize_with_prompt("Column: ").unwrap();

        if let Err(e) = game.take_turn(col, row) {
            println!("{} attempted to play in {}, {}", player, col, row);
            println!("Something went wrong: {}", e);
        }

        if let Some(winner) = game.outcome {
            println!("{}", winner);
            break;
        }
    }
}
