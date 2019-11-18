use log::debug;

use crate::coord::Coord;
use crate::errors::Result;
use crate::ricracroe::RRRGame;

mod settings;
use settings::RenderSettings;

mod cxterm;
use cxterm::CxTerm;

pub enum GameAction {
    TakeTurn(Coord),
    Quit,
}

pub fn play_game() -> Result<()> {
    let board_size: usize = 3;
    let mut game = RRRGame::new_anysize(board_size);

    let mut term = CxTerm::new(RenderSettings::new(board_size, 4), std::io::stdout())?;
    debug!("Resetting display");
    term.reset_display()?;

    debug!("Starting game...");

    loop {
        let player = game.get_turn();
        debug!("Player turn: {}", player);

        term.write_title("Welcome to Ric Rac Roe!")?;

        for board_row in 0..board_size {
            let rendered_board_row: usize = board_row * 2;
            debug!(
                "Rendering board row: {} ({})",
                board_row, rendered_board_row
            );
            if board_row == 0 {
                term.write_rendered_board_row(rendered_board_row, &game.board.render_board_top())?;
            } else {
                term.write_rendered_board_row(
                    rendered_board_row,
                    &game.board.render_board_row_sep(),
                )?;
            }
            term.write_rendered_board_row(
                rendered_board_row + 1,
                &game.board.render_board_row(board_row),
            )?;
        }
        term.write_rendered_board_row(board_size * 2, &game.board.render_board_bottom())?;

        term.write_status(&format!("It's {}'s turn.", player))?;
        term.commit()?;

        match term.get_game_action()? {
            GameAction::TakeTurn(coord) => {
                if let Err(e) = game.take_turn(&coord) {
                    term.write_status(&format!("{} cannot play in {} ({})", player, coord, e))?;
                }
            }
            GameAction::Quit => return Ok(()),
        }

        if let Some(winner) = game.outcome {
            term.write_status(&format!("{} won!", winner))?;
            return Ok(());
        }
    }
}
