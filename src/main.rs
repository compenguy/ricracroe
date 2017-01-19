// Allow default debug output display
#[derive(Debug)]
// Allow us to do equality tests on enum members
#[derive(PartialEq)]
// Give it copy semantics
#[derive(Clone, Copy)]
enum RRRCell {
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

#[derive(Debug)]
struct RRRBoard {
    cells: std::collections::HashMap<(u32, u32),RRRCell>,
    xsize: u32,
    ysize: u32,
}

// Box drawing chars from https://en.wikipedia.org/wiki/Box-drawing_character
impl std::fmt::Display for RRRBoard {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let xsize = self.get_xsize();
        let ysize = self.get_ysize();

        for y in 0..ysize {
            try!(write!(f, "  "));
            if y == 0 {
                // Display the top border
                for x in 0..xsize {
                    if x == 0 { try!(write!(f, "╭")); }
                    else      { try!(write!(f, "┬")); }
                    try!(write!(f, "─"));
                }
                try!(write!(f, "╮"));
            } else {
                // Display interstitial borders
                for x in 0..xsize {
                    if x == 0 { try!(write!(f, "├")); }
                    else      { try!(write!(f, "┼")); }
                    try!(write!(f, "─"));
                }
                try!(write!(f, "┤"));
            }
            try!(write!(f, "\n"));

            // Display one row of board cells
            try!(write!(f, "{} ", y));
            for x in 0..xsize {
                try!(write!(f, "│"));
                // TODO: Come back and add error handling
                try!(write!(f, "{}", self.fetch(x, y).unwrap()));
            }
            try!(write!(f, "│"));
            try!(write!(f, "\n"));
        }

        // Display the bottom border
        try!(write!(f, "  "));
        for x in 0..xsize {
            if x == 0 { try!(write!(f, "╰")); }
            else      { try!(write!(f, "┴")); }
            try!(write!(f, "─"));
        }
        try!(write!(f, "╯"));
        try!(write!(f, "\n"));

        try!(write!(f, "  "));
        for x in 0..xsize {
            if x == 0 { try!(write!(f, " ")); }
            else      { try!(write!(f, " ")); }
            try!(write!(f, "{}", x));
        }
        write!(f, "\n")
    }
}

impl RRRBoard {
    fn get_xsize(&self) -> u32 {
        self.xsize
    }

    fn get_ysize(&self) -> u32 {
        self.ysize
    }

    fn fetch(&self, x: u32, y: u32) -> Result<RRRCell, &'static str> {
        match self.cells.get(&(x, y)) {
            Some(cell) => Ok(*cell),
            None => Err("Invalid cell position."),
        }
    }

    fn set(&mut self, x: u32, y: u32, new_state: RRRCell) -> Result<RRRCell, &'static str> {
        if x <= self.xsize && y <= self.ysize {
            self.cells.insert((x, y), new_state);
            Ok(new_state)
        } else {
            Err("Invalid cell position.")
        }
    }

    fn make_move(&mut self, x: u32, y: u32, new_state: RRRCell) -> Result<RRRCell, &'static str> {
        // Block moves to cells that have already been used
        let cur_val = try!(self.fetch(x, y));
        if cur_val == RRRCell::X || cur_val == RRRCell::O {
            return Err("Cheater!");
        }

        self.set(x, y, new_state)
    }

    fn init(&mut self) {
        // reset the board
        self.cells.clear();
        self.cells.reserve((self.xsize*self.ysize) as usize);

        for x in 0..self.xsize {
            for y in 0..self.ysize {
                // Sure, go ahead and panic - I can't see how this could possibly fail
                self.set(x, y, RRRCell::Clear).unwrap();
            }
        }
    }

    fn new_anysize(xsize: u32, ysize: u32) -> Self {
        let mut _self = RRRBoard {
            cells: std::collections::HashMap::new(),
            xsize: xsize,
            ysize: ysize
        };
        _self.init();
        _self

    }

    fn new() -> Self {
        RRRBoard::new_anysize(3, 3)
    }

}

fn main() {
    let mut board = RRRBoard::new();
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
}
