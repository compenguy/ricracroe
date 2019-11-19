use log::debug;

use crate::coord::Coord;
use crate::errors::Result;
use crate::ricracroe::{RRRGame, RRROutcome};

mod settings;
use settings::RenderSettings;

mod cxterm;
use cxterm::CxTerm;

pub enum GameAction {
    TakeTurn(Coord),
    Quit,
}

const INSTRUCTIONS: &str = r#"Press 'Q', 'q', or <Esc> to quit.
To make a move:
1. Mouse click in square, or
2. Arrows to move and <Space> or <Enter> to select."#;

pub fn play_game() -> Result<()> {
    let board_size: usize = 3;
    let mut game = RRRGame::new_anysize(board_size);

    let mut term = CxTerm::new(RenderSettings::new(2, 4, board_size), std::io::stdout())?;
    debug!("Resetting display");
    term.reset_display()?;

    debug!("Starting game...");

    // We want this to be written once, and not refreshed with each loop
    term.write_msglog(INSTRUCTIONS)?;

    loop {
        let player = game.get_turn();
        debug!("Player turn: {}", player);

        // Redraw board state
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

        if let Some(outcome) = game.outcome {
            // Display game end condition
            match outcome {
                RRROutcome::Draw => term.write_status("It's a draw!")?,
                RRROutcome::XWins { .. } => term.write_status("X won!")?,
                RRROutcome::OWins { .. } => term.write_status("O won!")?,
            }
            term.write_msglog("Press any key to exit.")?;
            term.commit()?;
            term.get_input_event()?;
            return Ok(());
        } else {
            // Display game turn state
            term.write_status(&format!("It's {}'s turn.", player))?;
            term.commit()?;

            match term.get_game_action()? {
                GameAction::TakeTurn(coord) => {
                    if let Err(e) = game.take_turn(&coord) {
                        term.write_msglog(&format!("{} cannot play in {} ({})", player, coord, e))?;
                        term.commit()?;
                    } else {
                        term.clear_msglog()?;
                        term.commit()?;
                    }
                }
                GameAction::Quit => return Ok(()),
            }
        }
    }
}
