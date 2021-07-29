use crossterm::event::{poll, read, Event};
use crossterm::terminal::enable_raw_mode;
use std::io::Stdout;
use std::sync::mpsc::Sender;
use std::thread::{self, JoinHandle};
use std::time::{Duration, Instant};
use tui::backend::CrosstermBackend;
use tui::layout::Direction;
use tui::layout::Layout;
use tui::layout::{Constraint, Rect};
use tui::style::{Color, Modifier, Style};
use tui::text::{Span, Spans};
use tui::widgets::{self, Paragraph, Tabs};
use tui::widgets::{Block, Borders};
use tui::Terminal;

pub fn construct_terminal() -> Result<Terminal<CrosstermBackend<Stdout>>, ()> {
    //Don't require Enter from the user
    let raw_mode = enable_raw_mode();
    if raw_mode.is_err() {
        panic!("Unable to enable raw mode! Check terminal support!");
    }

    let stdout = std::io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend).unwrap();
    let _cleared = terminal.clear();

    Ok(terminal)
}

//TODO: Finish drawing the interface
pub fn draw_interface(
    term: &mut Terminal<CrosstermBackend<Stdout>>,
    typed_text: &String,
    chat_text: &String,
) {
    let chunks: Vec<Rect> = config_chunks(&term);
    let menu_bar: Vec<Spans> = config_menu_bar();

    let tabs: Tabs = config_tabs(menu_bar);
    let chat: Paragraph = config_chat(chat_text);
    let typing_area = config_typing_area(typed_text.to_string());

    let _result = term.draw(|f| {
        // Top to bottom -> 0 to chunks.len()
        f.render_widget(tabs, chunks[0]);
        f.render_widget(chat, chunks[1]);
        f.render_widget(typing_area, chunks[2]);
    });
}

pub fn create_input_thread(tx: Sender<Event>) -> JoinHandle<()> {
    thread::spawn(move || {
        let mut keyboard_timer = Instant::now();

        loop {
            let poll = poll(Duration::from_millis(200));

            if poll.is_ok() {
                let event = read();

                if event.is_ok() {
                    if let Event::Key(key) = event.unwrap() {
                        let duration = Instant::now().duration_since(keyboard_timer).as_millis();

                        if duration >= 20 {
                            let sended = tx.send(Event::Key(key));

                            if sended.is_err() {
                                break;
                            }

                            keyboard_timer = Instant::now();
                        }
                    }
                }
            }
        }
    })
}

fn config_chat(chat_text: &String) -> Paragraph {
    return Paragraph::new(chat_text.as_str())
        .style(Style::default().add_modifier(Modifier::BOLD))
        .block(
            Block::default()
                .border_style(Style::default())
                .borders(Borders::ALL),
        );
}

fn config_typing_area(text: String) -> Paragraph<'static> {
    return Paragraph::new(text)
        .style(Style::default().fg(Color::White))
        .alignment(tui::layout::Alignment::Left)
        .block(
            Block::default()
                .border_style(Style::default())
                .borders(Borders::ALL)
                .title("Type here")
                .border_type(widgets::BorderType::Rounded),
        );
}

fn config_chunks(term: &&mut Terminal<CrosstermBackend<Stdout>>) -> Vec<Rect> {
    let size = term.size().unwrap();
    let vert_constrains = [
        Constraint::Length(3), //Menu bar
        Constraint::Min(2),    //Chat
        Constraint::Length(3), //Typing area
    ]
    .as_ref();

    return Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints(vert_constrains)
        .split(size);
}

fn config_tabs(menu_bar: Vec<Spans>) -> Tabs {
    return Tabs::new(menu_bar)
        .block(Block::default().title("Menu").borders(Borders::ALL))
        .style(Style::default().fg(Color::White))
        .highlight_style(Style::default().fg(Color::Blue))
        .divider(Span::raw("|"));
}

fn config_menu_bar() -> Vec<Spans<'static>> {
    let menu_titles = vec!["Friends", "Quit"];

    return menu_titles
        .iter()
        .map(|t| {
            let (first, rest) = t.split_at(1);
            Spans::from(vec![
                Span::styled(
                    first,
                    Style::default()
                        .fg(Color::White)
                        .add_modifier(Modifier::UNDERLINED),
                ),
                Span::styled(rest, Style::default().fg(Color::White)),
            ])
        })
        .collect();
}
