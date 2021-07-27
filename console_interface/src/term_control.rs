use std::io::{Error, Stdout};
use tui::backend::CrosstermBackend;
use tui::Terminal;

pub fn construct_terminal() -> Result<Terminal<CrosstermBackend<Stdout>>, Error> {
    let stdout = std::io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;

    Ok(terminal)
}

pub fn draw_interface(term: &mut Terminal<CrosstermBackend<Stdout>>) {
    todo!()
}
