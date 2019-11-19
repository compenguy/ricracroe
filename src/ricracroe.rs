use std::collections::HashMap;
use std::collections::HashSet;
use std::error;
use std::fmt;
use std::vec::Vec;

use log::error;

use crate::coord::Coord;

#[derive(Debug)]
pub enum RRRError {
    InvalidCellPosition(Coord),
    CellAlreadySet(Coord, RRRCell),
    NoActivePlayer,
}

impl fmt::Display for RRRError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            RRRError::InvalidCellPosition(coord) => {
                write!(f, "{} is an invalid cell position.", coord)
            }
            RRRError::CellAlreadySet(coord, plyr) => {
                write!(f, "{} has already been played in by {}.", coord, plyr)
            }
            RRRError::NoActivePlayer => write!(f, "The current game has no active player."),
        }
    }
}

impl error::Error for RRRError {
    fn description(&self) -> &str {
        match *self {
            RRRError::InvalidCellPosition(_) => "invalid cell position",
            RRRError::CellAlreadySet(_, _) => "cell already played in",
            RRRError::NoActivePlayer => "no active player",
        }
    }

    fn cause(&self) -> Option<&dyn error::Error> {
        match *self {
            RRRError::InvalidCellPosition(_) => None,
            RRRError::CellAlreadySet(_, _) => None,
            RRRError::NoActivePlayer => None,
        }
    }
}

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

impl fmt::Display for RRRCell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            RRRCell::Clear => write!(f, "·"),
            RRRCell::X => write!(f, "X"),
            RRRCell::O => write!(f, "O"),
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
    XWins { winning_cells: Vec<Coord> },
    OWins { winning_cells: Vec<Coord> },
}

impl fmt::Display for RRROutcome {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            RRROutcome::Draw => write!(f, "It's a draw!"),
            RRROutcome::XWins { .. } => write!(f, "X Wins!"),
            RRROutcome::OWins { .. } => write!(f, "O Wins!"),
        }
    }
}

#[derive(Debug)]
pub struct RRRBoard {
    cells: HashMap<Coord, RRRCell>,
    size: usize,
}

// Box drawing chars from https://en.wikipedia.org/wiki/Box-drawing_character
impl fmt::Display for RRRBoard {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let size = self.get_size();
        let indent: String = "    ".to_string();

        // Write column numbers
        let mut line = indent.clone();
        for x in 0..size {
            line.push(' ');
            line.push_str(x.to_string().as_str());
        }
        writeln!(f, "{}", line)?;

        for y in 0..size {
            if y == 0 {
                // Draw the top of the board
                line = indent.clone();
                line.push_str(self.render_board_top().as_str());
                writeln!(f, "{}", line)?;
            } else {
                // Draw a row separator
                line = indent.clone();
                line.push_str(self.render_board_row_sep().as_str());
                writeln!(f, "{}", line)?;
            }

            // print row of board cells
            // indent + row number
            line = format!("{: >3} ", y);
            line.push_str(self.render_board_row(y).as_str());
            writeln!(f, "{}", line)?;
        }

        // Draw the bottom of the board
        line = indent.clone();
        line.push_str(self.render_board_bottom().as_str());
        writeln!(f, "{}", line)
    }
}

impl RRRBoard {
    pub fn render_board_top(&self) -> String {
        // starting and ending char, a char for each board cell, and a char separating each board
        // cell
        let mut line = String::with_capacity(2 + (self.get_size() * 2) - 1);
        for x in 0..self.get_size() {
            if x == 0 {
                line.push('╭');
            } else {
                line.push('┬');
            }
            line.push('─');
        }
        line.push('╮');
        line
    }

    pub fn render_board_bottom(&self) -> String {
        // starting and ending char, a char for each board cell, and a char separating each board
        // cell
        let mut line = String::with_capacity(2 + (self.get_size() * 2) - 1);
        for x in 0..self.get_size() {
            if x == 0 {
                line.push('╰');
            } else {
                line.push('┴');
            }
            line.push('─');
        }
        line.push('╯');
        line
    }

    pub fn render_board_row(&self, y: usize) -> String {
        // starting and ending char, a char for each board cell, and a char separating each board
        // cell
        let mut line = String::with_capacity(2 + (self.get_size() * 2) - 1);
        for x in 0..self.get_size() {
            line.push('│');
            match self.fetch(&Coord { x, y }) {
                Ok(RRRCell::X) => line.push('X'),
                Ok(RRRCell::O) => line.push('O'),
                Ok(RRRCell::Clear) => line.push(' '),
                Err(_) => line.push('!'),
            }
        }
        line.push('│');
        line
    }

    pub fn render_board_row_sep(&self) -> String {
        let mut line = String::with_capacity(2 + (self.get_size() * 2) - 1);
        for x in 0..self.get_size() {
            if x == 0 {
                line.push('├');
            } else {
                line.push('┼');
            }
            line.push('─');
        }
        line.push('┤');
        line
    }

    pub fn get_size(&self) -> usize {
        self.size
    }

    pub fn fetch(&self, coord: &Coord) -> Result<RRRCell, RRRError> {
        match self.cells.get(coord) {
            Some(cell) => Ok(*cell),
            None => Err(RRRError::InvalidCellPosition(*coord)),
        }
    }

    pub fn set(&mut self, coord: &Coord, new_state: RRRCell) -> Result<RRRCell, RRRError> {
        if coord.x <= self.size && coord.y <= self.size {
            self.cells.insert(*coord, new_state);
            Ok(new_state)
        } else {
            Err(RRRError::InvalidCellPosition(*coord))
        }
    }

    pub fn make_move(&mut self, coord: &Coord, new_state: RRRCell) -> Result<RRRCell, RRRError> {
        // Block moves to cells that have already been used
        let cur_val = self.fetch(coord)?;
        if cur_val == RRRCell::X || cur_val == RRRCell::O {
            Err(RRRError::CellAlreadySet(*coord, cur_val))
        } else {
            self.set(coord, new_state)
        }
    }

    // maybe_winning_cells is the sub-hashmap of the cells we should be considering
    fn test_winner(&self, maybe_winning_cells: &HashMap<&Coord, &RRRCell>) -> Option<RRROutcome> {
        assert!(maybe_winning_cells.len() == self.get_size());
        let vals: HashSet<&RRRCell> = maybe_winning_cells.values().cloned().collect();

        // If the set of all values being considered is exactly one, and it's not Clear
        // then we have a winner
        if vals.len() == 1 && !vals.contains(&RRRCell::Clear) {
            match vals.iter().next() {
                Some(&&RRRCell::X) => Some(RRROutcome::XWins {
                    winning_cells: maybe_winning_cells.keys().cloned().cloned().collect(),
                }),
                Some(&&RRRCell::O) => Some(RRROutcome::OWins {
                    winning_cells: maybe_winning_cells.keys().cloned().cloned().collect(),
                }),
                Some(&&RRRCell::Clear) => None,
                None => None,
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
            maybe_winning_cells = self
                .cells
                .iter()
                .filter(|(&coord, &_)| coord.x == boardslice)
                .collect();
            if let Some(winning_column) = self.test_winner(&maybe_winning_cells) {
                return Some(winning_column);
            }
            maybe_winning_cells.clear();

            maybe_winning_cells = self
                .cells
                .iter()
                .filter(|&(&coord, &_)| coord.y == boardslice)
                .collect();
            if let Some(winning_row) = self.test_winner(&maybe_winning_cells) {
                return Some(winning_row);
            }
            maybe_winning_cells.clear()
        }
        // positive slope diagonal
        maybe_winning_cells = self
            .cells
            .iter()
            .filter(|&(&coord, &_)| coord.x == coord.y)
            .collect();
        if let Some(winning_positive_diagonal) = self.test_winner(&maybe_winning_cells) {
            return Some(winning_positive_diagonal);
        }
        maybe_winning_cells.clear();

        // negative slope diagonal
        maybe_winning_cells = self
            .cells
            .iter()
            .filter(|&(&coord, &_)| coord.x == self.get_size() - coord.y - 1)
            .collect();
        if let Some(winning_negative_diagonal) = self.test_winner(&maybe_winning_cells) {
            return Some(winning_negative_diagonal);
        }
        maybe_winning_cells.clear();

        // no winners, look for draw
        // If no cell is RRRCell::Clear, it's not a draw yet
        if !self.cells.values().any(|&x| x == RRRCell::Clear) {
            Some(RRROutcome::Draw)
        } else {
            None
        }
    }

    pub fn init(&mut self) {
        // reset the board
        self.cells.clear();
        self.cells.reserve((self.size * self.size) as usize);

        for x in 0..self.size {
            for y in 0..self.size {
                // Sure, go ahead and panic - I can't see how this could possibly fail
                self.set(&Coord { x, y }, RRRCell::Clear).unwrap();
            }
        }
    }

    pub fn new_anysize(size: usize) -> Self {
        let mut _self = RRRBoard {
            cells: HashMap::new(),
            size,
        };
        _self.init();
        _self
    }

    #[allow(dead_code)]
    pub fn new() -> Self {
        RRRBoard::new_anysize(3)
    }
}

pub struct RRRGame {
    pub board: RRRBoard,
    pub player: RRRCell,
    pub outcome: Option<RRROutcome>,
}

impl RRRGame {
    #[allow(dead_code)]
    pub fn get_board(&self) -> &RRRBoard {
        &self.board
    }

    pub fn get_turn(&self) -> RRRCell {
        self.player
    }

    pub fn next_player(&mut self) -> Result<RRRCell, RRRError> {
        match self.player {
            RRRCell::X => {
                self.player = RRRCell::O;
                Ok(self.player)
            }
            RRRCell::O => {
                self.player = RRRCell::X;
                Ok(self.player)
            }
            RRRCell::Clear => Err(RRRError::NoActivePlayer),
        }
    }

    pub fn take_turn(&mut self, coord: &Coord) -> Result<RRRCell, RRRError> {
        match self.board.make_move(coord, self.player) {
            Ok(_) => {
                self.outcome = self.board.outcome();
                if !self.over() {
                    self.next_player().unwrap();
                }
                Ok(self.player)
            }
            Err(e) => {
                error!(
                    "{} attempted to play in {},{}:\n{}",
                    self.player,
                    coord.x + 1,
                    coord.y + 1,
                    self.board
                );
                error!("Something went wrong: {}", e);
                Err(e)
            }
        }
    }

    pub fn over(&self) -> bool {
        self.outcome.is_some()
    }

    pub fn new_anysize(size: usize) -> Self {
        RRRGame {
            board: RRRBoard::new_anysize(size),
            player: RRRCell::X,
            outcome: None,
        }
    }

    #[allow(dead_code)]
    pub fn new() -> Self {
        RRRGame::new_anysize(3)
    }
}

#[allow(dead_code)]
fn main() {
    let mut board = RRRBoard::new_anysize(6);
    // TODO: Right now, just being lazy and allowing errors to panic
    println!("Starting board:\n{}", board);
    board.make_move(&Coord { x: 0, y: 0 }, RRRCell::X).unwrap();
    println!("After single move:\n{}", board);
    board.make_move(&Coord { x: 0, y: 1 }, RRRCell::X).unwrap();
    println!("After next move:\n{}", board);
    board.make_move(&Coord { x: 2, y: 2 }, RRRCell::X).unwrap();
    println!("After next move:\n{}", board);
    match board.make_move(&Coord { x: 2, y: 2 }, RRRCell::O) {
        Ok(_) => println!("Well, that worked."),
        Err(e) => println!("Something went wrong: {}", e),
    }
    println!("After next move:\n{}", board);
    match board.outcome() {
        Some(RRROutcome::XWins { .. }) => println!("X Wins!"),
        Some(RRROutcome::OWins { .. }) => println!("O Wins!"),
        Some(RRROutcome::Draw) => println!("It's a draw!"),
        None => {}
    }
}
