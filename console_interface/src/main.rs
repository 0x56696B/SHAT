mod core;
mod term_control;
use std::io::Stdout;
use tui::backend::CrosstermBackend;
use tui::Terminal;

pub struct AppState<'a> {
    pub title: &'a str,
    pub should_quit: bool,
    pub typed_text: String,
}

impl<'a> AppState<'a> {
    pub fn new() -> Self {
        AppState {
            title: "SHAT",
            should_quit: false,
            typed_text: String::from(""),
        }
    }
}

pub fn main() -> Result<(), i32> {
    let mut term: Terminal<CrosstermBackend<Stdout>> = term_control::construct_terminal().unwrap();
    let mut app_state: AppState = AppState::new();

    core::application_cycle(&mut term, &mut app_state)?;

    Ok(())
}
