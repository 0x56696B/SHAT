use crate::term_control;
use crate::AppState;
use crossterm::event::Event;
use crossterm::event::KeyCode;
use crossterm::event::KeyEvent;
use crossterm::event::KeyModifiers;
use std::io::Stdout;
use std::sync::mpsc::channel;
use std::thread;
use std::time::Duration;
use tui::backend::CrosstermBackend;
use tui::Terminal;

pub fn application_cycle(
    term: &mut Terminal<CrosstermBackend<Stdout>>,
    app_state: &mut AppState,
) -> Result<(), i32> {
    let (tx, rx) = channel::<Event>();
    let _input_thread = term_control::create_input_thread(tx.clone());

    loop {
        //Draw interface
        let _drawing_state = term_control::draw_interface(term, &mut app_state.typed_text);

        //TODO: Make asynchronous
        //Check for input
        let receive_event = rx.try_recv();

        //Process input
        if receive_event.is_ok() {
            match receive_event.unwrap() {
                Event::Key(key) => process_key_event(key, app_state),
                Event::Resize(..) => continue,
                Event::Mouse(_) => continue,
            }
        }

        //Terminate if the global state is to not run
        if app_state.should_quit {
            break;
        }

        let sleep_duration = Duration::from_millis(5);
        thread::sleep(sleep_duration);
    }

    Ok(())
}

fn process_key_event(key: KeyEvent, app_state: &mut AppState) -> () {
    if key.modifiers == KeyModifiers::CONTROL && key.code == KeyCode::Char('c') {
        app_state.should_quit = true;
        return;
    }

    match key.code {
        KeyCode::Esc => app_state.typed_text = "".to_string(),
        KeyCode::Char(c) => app_state.typed_text.push(c),
        KeyCode::Backspace => {
            let _popped_char = app_state.typed_text.pop();
            ()
        }
        _ => (),
    }
}
