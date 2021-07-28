mod core;
mod db_access;
mod term_control;
use std::io::Stdout;
use tui::backend::CrosstermBackend;
use tui::Terminal;

fn main() -> Result<(), i32> {
    let mut term: Terminal<CrosstermBackend<Stdout>> = term_control::construct_terminal().unwrap();

    core::application_cycle(&mut term)?;

    Ok(())
}
