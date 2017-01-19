// Allow us to do equality tests on enum members
// Give it copy semantics
#[derive(PartialEq)]
#[derive(Clone, Copy)]
enum RRRCell {
    Clear,
    X,
    O,
}

struct RRRBoard {
    cells: std::collections::HashMap<(u32, u32),RRRCell>,
    xsize: u32,
    ysize: u32,
}

impl RRRBoard {
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
    // Right now, just being lazy and allowing errors to panic
    board.make_move(0, 0, RRRCell::X).unwrap();
}
