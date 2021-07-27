mod core;
mod db_access;
mod term_control;

use crossterm::terminal::enable_raw_mode;
use std::io::Stdout;
use tui::backend::CrosstermBackend;
use tui::Terminal;

fn main() -> Result<(), i32> {
    //Don't require Enter from the user
    enable_raw_mode();

    let mut term: Terminal<CrosstermBackend<Stdout>> = term_control::construct_terminal().unwrap();

    core::application_cycle(&mut term)?;

    Ok(())
}
