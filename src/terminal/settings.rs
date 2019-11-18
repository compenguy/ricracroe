use crate::coord::Coord;

pub struct RenderSettings {
    board_size: usize,
    board_padding: usize,
}

impl RenderSettings {
    pub fn new(board_size: usize, board_padding: usize) -> Self {
        RenderSettings {
            board_size,
            board_padding,
        }
    }

    pub fn get_board_size(&self) -> usize {
        self.board_size
    }

    pub fn term_coord_to_cell_coord(&self, term_coord: &Coord) -> Coord {
        let term_rel_coord: Coord = *term_coord - self.get_board_origin();
        Coord {
            x: (term_rel_coord.x - 1) / 2,
            y: (term_rel_coord.y - 1) / 2,
        }
    }

    pub fn cell_coord_to_term_coord(&self, cell_coord: &Coord) -> Coord {
        let term_rel_coord = Coord {
            x: (2 * cell_coord.x) + 1,
            y: (2 * cell_coord.y) + 1,
        };
        self.get_board_origin() + term_rel_coord
    }

    pub fn get_rendered_board_height(&self) -> usize {
        (1 + (self.board_size + 2)) as usize
    }

    pub fn get_title_origin(&self) -> Coord {
        Coord { x: 0, y: 0 }
    }

    pub fn get_board_origin(&self) -> Coord {
        self.get_title_origin()
            + Coord {
                x: self.board_padding,
                y: self.board_padding,
            }
    }

    pub fn get_status_origin(&self) -> Coord {
        Coord {
            x: 0,
            y: self.get_board_origin().y + self.get_rendered_board_height() + self.board_padding,
        }
    }
}
