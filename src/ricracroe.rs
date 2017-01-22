extern crate std;

use std::collections::HashMap;
use std::collections::HashSet;
use std::vec::Vec;

// Allow default debug output display
#[derive(Debug)]
// Allow us to do equality tests on enum members, needed for hashing
#[derive(PartialEq, Eq)]
// Allow hashing of RRRCell
#[derive(Hash)]
// Give it copy semantics
#[derive(Clone, Copy)]
pub enum RRRCell {
    Clear,
    X,
    O,
}

impl std::fmt::Display for RRRCell {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            RRRCell::Clear => write!(f, "·"),
            RRRCell::X     => write!(f, "X"),
            RRRCell::O     => write!(f, "O"),
        }
    }
}

// Allow default debug output display
#[derive(Debug)]
// Allow us to do equality tests on enum members
#[derive(PartialEq, Eq)]
// Give it copy semantics
#[derive(Clone)]
pub enum RRROutcome {
    Draw,
    XWins { winning_cells: Vec<(usize, usize)> },
    OWins { winning_cells: Vec<(usize, usize)> },
}

impl std::fmt::Display for RRROutcome {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            RRROutcome::Draw => write!(f, "It's a draw!"),
            RRROutcome::XWins{ winning_cells: _ } => write!(f, "X Wins!"),
            RRROutcome::OWins{ winning_cells: _ } => write!(f, "O Wins!"),
        }
    }
}

#[derive(Debug)]
pub struct RRRBoard {
    cells: HashMap<(usize, usize),RRRCell>,
    size: usize,
}

// Box drawing chars from https://en.wikipedia.org/wiki/Box-drawing_character
impl std::fmt::Display for RRRBoard {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let size = self.get_size();

        for y in 0..size {
            try!(write!(f, "  "));
            if y == 0 {
                // Display the x coordinates at the top
                for x in 0..size {
                    if x == 0 { try!(write!(f, " ")); }
                    else      { try!(write!(f, " ")); }
                    try!(write!(f, "{}", x+1));
                }
                try!(write!(f, "\n"));
                try!(write!(f, "  "));
                // Display the top border
                for x in 0..size {
                    if x == 0 { try!(write!(f, "╭")); }
                    else      { try!(write!(f, "┬")); }
                    try!(write!(f, "─"));
                }
                try!(write!(f, "╮"));
            } else {
                // Display interstitial borders
                for x in 0..size {
                    if x == 0 { try!(write!(f, "├")); }
                    else      { try!(write!(f, "┼")); }
                    try!(write!(f, "─"));
                }
                try!(write!(f, "┤"));
            }
            try!(write!(f, "\n"));

            // Display one row of board cells
            try!(write!(f, "{} ", y+1));
            for x in 0..size {
                try!(write!(f, "│"));
                // TODO: Come back and add error handling
                try!(write!(f, "{}", self.fetch(x, y).unwrap()));
            }
            try!(write!(f, "│"));
            try!(write!(f, "\n"));
        }

        // Display the bottom border
        try!(write!(f, "  "));
        for x in 0..size {
            if x == 0 { try!(write!(f, "╰")); }
            else      { try!(write!(f, "┴")); }
            try!(write!(f, "─"));
        }
        try!(write!(f, "╯"));
        write!(f, "\n")
    }
}

impl RRRBoard {
    pub fn get_size(&self) -> usize {
        self.size
    }

    pub fn fetch(&self, x: usize, y: usize) -> Result<RRRCell, &'static str> {
        match self.cells.get(&(x, y)) {
            Some(cell) => Ok(*cell),
            None => Err("Invalid cell position."),
        }
    }

    pub fn set(&mut self, x: usize, y: usize, new_state: RRRCell) -> Result<RRRCell, &'static str> {
        if x <= self.size && y <= self.size {
            self.cells.insert((x, y), new_state);
            Ok(new_state)
        } else {
            Err("Invalid cell position.")
        }
    }

    pub fn make_move(&mut self, x: usize, y: usize, new_state: RRRCell) -> Result<RRRCell, &'static str> {
        // Block moves to cells that have already been used
        let cur_val = try!(self.fetch(x, y));
        if cur_val == RRRCell::X || cur_val == RRRCell::O {
            return Err("Cheater!");
        }

        self.set(x, y, new_state)
    }

    // maybe_winning_cells is the sub-hashmap of the cells we should be considering
    fn test_winner(&self, maybe_winning_cells: &HashMap<&(usize, usize),&RRRCell>) -> Option<RRROutcome> {
        assert!(maybe_winning_cells.len() == self.get_size());
        let vals: HashSet<&RRRCell> = maybe_winning_cells.values().cloned().collect();

        // If the set of all values being considered is exactly one, and it's not Clear
        // then we have a winner
        if vals.len() == 1 && !vals.contains(&RRRCell::Clear) {
            match vals.iter().next() {
                Some(&&RRRCell::X)     => Some(RRROutcome::XWins { winning_cells: maybe_winning_cells.keys().cloned().cloned().collect() }),
                Some(&&RRRCell::O)     => Some(RRROutcome::OWins { winning_cells: maybe_winning_cells.keys().cloned().cloned().collect() }),
                Some(&&RRRCell::Clear) => None,
                None                   => None,
            }
        } else {
            None
        }
    }

    #[allow(dead_code)]
    pub fn outcome(&self) -> Option<RRROutcome> {
        // look for winners
        let mut maybe_winning_cells;

        // TODO: Recognize when there's more than winning path
        // start with the rows and columns
        for boardslice in 0..self.get_size() {
            maybe_winning_cells = self.cells.iter().filter(|&(&(x, _), &_)| x == boardslice).collect();
            if let Some(winning_column) = self.test_winner(&maybe_winning_cells) {
                return Some(winning_column);
            }
            maybe_winning_cells.clear();

            maybe_winning_cells = self.cells.iter().filter(|&(&(_, y), &_)| y == boardslice).collect();
            if let Some(winning_row) = self.test_winner(&maybe_winning_cells) {
                return Some(winning_row);
            }
            maybe_winning_cells.clear()
        }
        // positive slope diagonal
        maybe_winning_cells = self.cells.iter().filter(|&(&(x, y), &_)| x == y).collect();
        if let Some(winning_positive_diagonal) = self.test_winner(&maybe_winning_cells) {
            return Some(winning_positive_diagonal);
        }
        maybe_winning_cells.clear();

        // negative slope diagonal
        maybe_winning_cells = self.cells.iter().filter(|&(&(x, y), &_)| x ==  self.get_size() - y - 1).collect();
        if let Some(winning_negative_diagonal) = self.test_winner(&maybe_winning_cells) {
            return Some(winning_negative_diagonal);
        }
        maybe_winning_cells.clear();

        // no winners, look for draw
        // If no cell is RRRCell::Clear, it's not a draw yet
        if self.cells.values().find(|&&x| x == RRRCell::Clear) != Some(&RRRCell::Clear) {
            Some(RRROutcome::Draw)
        } else {
            None
        }
    }

    pub fn init(&mut self) {
        // reset the board
        self.cells.clear();
        self.cells.reserve((self.size*self.size) as usize);

        for x in 0..self.size {
            for y in 0..self.size {
                // Sure, go ahead and panic - I can't see how this could possibly fail
                self.set(x, y, RRRCell::Clear).unwrap();
            }
        }
    }

    pub fn new_anysize(size: usize) -> Self {
        let mut _self = RRRBoard {
            cells: HashMap::new(),
            size: size,
        };
        _self.init();
        _self

    }

    #[allow(dead_code)]
    pub fn new() -> Self {
        RRRBoard::new_anysize(3)
    }

}

#[allow(dead_code)]
fn main() {
    let mut board = RRRBoard::new_anysize(6);
    // TODO: Right now, just being lazy and allowing errors to panic
    println!("Starting board:\n{}", board);
    board.make_move(0, 0, RRRCell::X).unwrap();
    println!("After single move:\n{}", board);
    board.make_move(0, 1, RRRCell::O).unwrap();
    println!("After next move:\n{}", board);
    board.make_move(2, 2, RRRCell::X).unwrap();
    println!("After next move:\n{}", board);
    match board.make_move(2, 2, RRRCell::O) {
        Ok(_) => println!("Well, that worked."),
        Err(e) => println!("Something went wrong: {}", e),
    }
    println!("After next move:\n{}", board);
    match board.outcome() {
        Some(RRROutcome::XWins { winning_cells: _ }) => println!("X Wins!"),
        Some(RRROutcome::OWins { winning_cells: _ }) => println!("O Wins!"),
        Some(RRROutcome::Draw) => println!("It's a draw!"),
        None => {},
    }
}
