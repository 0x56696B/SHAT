use crate::db_access;
use crate::term_control;
use crossterm::event::Event;
use std::io::Stdout;
use std::sync::mpsc::channel;
use std::time::Duration;
use tui::backend::CrosstermBackend;
use tui::Terminal;

pub fn application_cycle(term: &mut Terminal<CrosstermBackend<Stdout>>) -> Result<(), i32> {
    let (tx, rx) = channel::<Event>();
    //Potentially read from config
    let tick_rate = Duration::from_millis(200);

    loop {
        term_control::draw_interface(term);
    }
}

//TODO: Implement
// Returns: An error code indicating state of the execution
// -1: Exit program
fn execute_command(input: &str) -> i32 {
    match input {
        //Commands:
        ":exit" | ":e" | ":q" => -1,
        _ => 0,
    }
}
