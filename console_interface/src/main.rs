mod core;
mod db_access;
mod term_control;
use std::io::Stdout;
use tui::backend::CrosstermBackend;
use tui::Terminal;

pub enum WinState {
    Chatting,
    Menubar,
    Friends, //Future proof

    Error,
    None,
}

pub struct AppState<'a> {
    pub title: &'a str,
    pub should_quit: bool,
    pub typed_text: String,
    pub window_state: WinState,
    pub chat_friend: String,
    pub username: String,
}

impl<'a> AppState<'a> {
    pub fn new() -> Self {
        AppState {
            title: "SHAT",
            should_quit: false,
            typed_text: String::from(""),
            window_state: WinState::None,
            chat_friend: String::from("dummy"),
            username: String::from("The cool dude"), //Fetch from config file
        }
    }
}

pub fn main() -> Result<(), i32> {
    let mut term: Terminal<CrosstermBackend<Stdout>> = term_control::construct_terminal().unwrap();
    let mut app_state: AppState = AppState::new();

    core::application_cycle(&mut term, &mut app_state)?;

    Ok(())
}
