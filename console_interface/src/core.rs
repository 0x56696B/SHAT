use crate::db_access;
use crate::term_control;
use crossterm::event::Event;
use crossterm::event::KeyCode;
use crossterm::event::KeyEvent;
use std::io::Stdout;
use std::sync::mpsc::channel;
use tui::backend::CrosstermBackend;
use tui::Terminal;

pub fn application_cycle(term: &mut Terminal<CrosstermBackend<Stdout>>) -> Result<(), i32> {
    let (tx, rx) = channel::<Event>();
    let _input_thread = term_control::create_input_thread(tx.clone());

    loop {
        //Draw interface
        let drawing_state = term_control::draw_interface(term);
        if drawing_state.is_err() {
            break;
        }

        //TODO: Make asynchronous
        //Check for input
        let receive_event = rx.recv();

        //Process input
        if receive_event.is_ok() {
            let mut command_code: i32 = 0;

            match receive_event.unwrap() {
                Event::Key(key) => command_code = process_key_event(key),
                Event::Resize(..) => continue,
                Event::Mouse(_) => continue,
            }

            //Command code processing
            if command_code == -1 {
                break;
            } else {
                execute_command(command_code);
            }
        }
    }

    Ok(())
}

//TODO: Implement
fn execute_command(command_code: i32) -> () {
    ()
}

fn process_key_event(key: KeyEvent) -> i32 {
    let mut command_code = 0;

    match key.code {
        KeyCode::Esc => command_code = -1,
        _ => command_code = 0,
    }

    command_code
}
