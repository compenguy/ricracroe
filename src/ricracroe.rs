extern crate std;

use std::collections::HashMap;
use std::collections::HashSet;
use std::error;
use std::fmt;
use std::vec::Vec;

use azul::prelude::*;

#[derive(Debug)]
pub enum RRRError {
    InvalidCellPosition(usize, usize),
    CellAlreadySet(usize, usize, RRRCell),
    NoActivePlayer,
}

impl fmt::Display for RRRError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            RRRError::InvalidCellPosition(x, y) => {
                write!(f, "{}, {} is an invalid cell position.", x, y)
            }
            RRRError::CellAlreadySet(x, y, plyr) => {
                write!(f, "{}, {} has already been played in by {}.", x, y, plyr)
            }
            RRRError::NoActivePlayer => write!(f, "The current game has no active player."),
        }
    }
}

impl error::Error for RRRError {
    fn description(&self) -> &str {
        match *self {
            RRRError::InvalidCellPosition(_, _) => "invalid cell position",
            RRRError::CellAlreadySet(_, _, _) => "cell already played in",
            RRRError::NoActivePlayer => "no active player",
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            RRRError::InvalidCellPosition(_, _) => None,
            RRRError::CellAlreadySet(_, _, _) => None,
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
    XWins { winning_cells: Vec<(usize, usize)> },
    OWins { winning_cells: Vec<(usize, usize)> },
}

impl fmt::Display for RRROutcome {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            RRROutcome::Draw => write!(f, "It's a draw!"),
            RRROutcome::XWins { winning_cells: _ } => write!(f, "X Wins!"),
            RRROutcome::OWins { winning_cells: _ } => write!(f, "O Wins!"),
        }
    }
}

#[derive(Debug)]
pub struct RRRBoard {
    cells: HashMap<(usize, usize), RRRCell>,
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
        try!(write!(f, "{}\n", line));

        for y in 0..size {
            if y == 0 {
                // Draw the top of the board
                line = indent.clone();
                line.push_str(self.render_board_top().as_str());
                try!(write!(f, "{}\n", line));
            } else {
                // Draw a row separator
                line = indent.clone();
                line.push_str(self.render_board_row_sep().as_str());
                try!(write!(f, "{}\n", line));
            }

            // print row of board cells
            // indent + row number
            line = format!("{: >3} ", y);
            line.push_str(self.render_board_row(y).as_str());
            try!(write!(f, "{}\n", line));
        }

        // Draw the bottom of the board
        line = indent.clone();
        line.push_str(self.render_board_bottom().as_str());
        write!(f, "{}\n", line)
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
            match self.fetch(x, y) {
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

    pub fn fetch(&self, x: usize, y: usize) -> Result<RRRCell, RRRError> {
        match self.cells.get(&(x, y)) {
            Some(cell) => Ok(*cell),
            None => Err(RRRError::InvalidCellPosition(x, y)),
        }
    }

    pub fn set(&mut self, x: usize, y: usize, new_state: RRRCell) -> Result<RRRCell, RRRError> {
        if x <= self.size && y <= self.size {
            self.cells.insert((x, y), new_state);
            Ok(new_state)
        } else {
            Err(RRRError::InvalidCellPosition(x, y))
        }
    }

    pub fn make_move(
        &mut self,
        x: usize,
        y: usize,
        new_state: RRRCell,
    ) -> Result<RRRCell, RRRError> {
        // Block moves to cells that have already been used
        let cur_val = try!(self.fetch(x, y));
        if cur_val == RRRCell::X || cur_val == RRRCell::O {
            Err(RRRError::CellAlreadySet(x, y, cur_val))
        } else {
            self.set(x, y, new_state)
        }
    }

    // maybe_winning_cells is the sub-hashmap of the cells we should be considering
    fn test_winner(
        &self,
        maybe_winning_cells: &HashMap<&(usize, usize), &RRRCell>,
    ) -> Option<RRROutcome> {
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
                .filter(|&(&(x, _), &_)| x == boardslice)
                .collect();
            if let Some(winning_column) = self.test_winner(&maybe_winning_cells) {
                return Some(winning_column);
            }
            maybe_winning_cells.clear();

            maybe_winning_cells = self
                .cells
                .iter()
                .filter(|&(&(_, y), &_)| y == boardslice)
                .collect();
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
        maybe_winning_cells = self
            .cells
            .iter()
            .filter(|&(&(x, y), &_)| x == self.get_size() - y - 1)
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

pub struct RRRGame {
    pub board: RRRBoard,
    pub player: RRRCell,
    pub outcome: Option<RRROutcome>,
}

impl RRRGame {
    pub fn get_board(&self) -> &RRRBoard {
        return &self.board;
    }

    pub fn get_turn(&self) -> RRRCell {
        return self.player;
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

    pub fn take_turn(&mut self, x: usize, y: usize) -> Result<RRRCell, RRRError> {
        match self.board.make_move(x, y, self.player) {
            Ok(_) => {
                self.outcome = self.board.outcome();
                if !self.over() {
                    self.next_player().unwrap();
                }
                Ok(self.player)
            }
            Err(e) => {
                println!(
                    "{} attempted to play in {}, {}:\n{}",
                    self.player,
                    x + 1,
                    y + 1,
                    self.board
                );
                println!("Something went wrong: {}", e);
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

impl Layout for RRRGame {
    fn layout(&self, _info: WindowInfo<Self>) -> Dom<Self> {
        Dom::new(NodeType::Div)
            .with_child(
                Dom::new(NodeType::Div)
                    .with_class("row")
                    .with_hit_test(On::MouseUp)
                    .with_child(
                        Dom::new(NodeType::Label(format!(
                            "{}",
                            self.get_board()
                                .fetch(0, 0)
                                .expect("Error updating board state")
                        ))).with_class("board-cell")
                        .with_hit_test(On::MouseUp),
                    ).with_child(
                        Dom::new(NodeType::Label(format!(
                            "{}",
                            self.get_board()
                                .fetch(0, 1)
                                .expect("Error updating board state")
                        ))).with_class("board-cell")
                        .with_hit_test(On::MouseUp),
                    ).with_child(
                        Dom::new(NodeType::Label(format!(
                            "{}",
                            self.get_board()
                                .fetch(0, 2)
                                .expect("Error updating board state")
                        ))).with_class("board-cell")
                        .with_hit_test(On::MouseUp),
                    ),
            ).with_child(
                Dom::new(NodeType::Div)
                    .with_class("row")
                    .with_hit_test(On::MouseUp)
                    .with_child(
                        Dom::new(NodeType::Label(format!(
                            "{}",
                            self.get_board()
                                .fetch(1, 0)
                                .expect("Error updating board state")
                        ))).with_class("board-cell")
                        .with_hit_test(On::MouseUp),
                    ).with_child(
                        Dom::new(NodeType::Label(format!(
                            "{}",
                            self.get_board()
                                .fetch(1, 1)
                                .expect("Error updating board state")
                        ))).with_class("board-cell")
                        .with_hit_test(On::MouseUp),
                    ).with_child(
                        Dom::new(NodeType::Label(format!(
                            "{}",
                            self.get_board()
                                .fetch(1, 2)
                                .expect("Error updating board state")
                        ))).with_class("board-cell")
                        .with_hit_test(On::MouseUp),
                    ),
            ).with_child(
                Dom::new(NodeType::Div)
                    .with_class("row")
                    .with_hit_test(On::MouseUp)
                    .with_child(
                        Dom::new(NodeType::Label(format!(
                            "{}",
                            self.get_board()
                                .fetch(2, 0)
                                .expect("Error updating board state")
                        ))).with_class("board-cell")
                        .with_hit_test(On::MouseUp),
                    ).with_child(
                        Dom::new(NodeType::Label(format!(
                            "{}",
                            self.get_board()
                                .fetch(2, 1)
                                .expect("Error updating board state")
                        ))).with_class("board-cell")
                        .with_hit_test(On::MouseUp),
                    ).with_child(
                        Dom::new(NodeType::Label(format!(
                            "{}",
                            self.get_board()
                                .fetch(2, 2)
                                .expect("Error updating board state")
                        ))).with_class("board-cell")
                        .with_hit_test(On::MouseUp),
                    ),
            ).with_callback(On::MouseUp, Callback(handle_mouseclick_board))
    }
}

fn handle_mouseclick_board(
    app_state: &mut AppState<RRRGame>,
    event: WindowEvent<RRRGame>,
) -> UpdateScreen {
    // Figure out which row was clicked
    let (clicked_row_idx, row_that_was_clicked) =
        match event.get_first_hit_child(event.hit_dom_node, On::MouseUp) {
            Some(s) => s,
            None => return UpdateScreen::DontRedraw,
        };

    let (clicked_col_idx, col_that_was_clicked) =
        match event.get_first_hit_child(row_that_was_clicked, On::MouseUp) {
            Some(s) => s,
            None => return UpdateScreen::DontRedraw,
        };

    app_state.data.modify(|board| {
        board.take_turn(clicked_row_idx, clicked_col_idx);
        {}
    });
    UpdateScreen::Redraw
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
        None => {}
    }
}
