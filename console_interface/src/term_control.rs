use crossterm::event::{poll, read, Event};
use crossterm::terminal::enable_raw_mode;
use std::io::{Error, Stdout};
use std::sync::mpsc::Sender;
use std::thread::{self, JoinHandle};
use std::time::{Duration, Instant};
use tui::backend::CrosstermBackend;
use tui::layout::Constraint;
use tui::layout::Direction;
use tui::layout::Layout;
use tui::style::{Color, Modifier, Style};
use tui::text::{Span, Spans};
use tui::widgets::{self, canvas::*, Tabs};
use tui::widgets::{Block, Borders};
use tui::Terminal;

pub fn construct_terminal() -> Result<Terminal<CrosstermBackend<Stdout>>, Error> {
    //Don't require Enter from the user
    let raw_mode = enable_raw_mode();
    if raw_mode.is_err() {
        panic!("Unable to enable raw mode! Check terminal support!");
    }

    let stdout = std::io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;

    Ok(terminal)
}

//TODO: Finish drawing the interface
pub fn draw_interface(term: &mut Terminal<CrosstermBackend<Stdout>>) -> Result<(), std::io::Error> {
    let size = term.size().unwrap();
    let vert_constrains = [
        Constraint::Length(3),
        Constraint::Min(2),
        Constraint::Length(3),
        // Constraint::Ratio(70, 20),
    ]
    .as_ref();
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints(vert_constrains)
        .split(size);

    let menu_titles = vec!["Friends", "Quit"];
    let menu_bar = menu_titles
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

    let tabs = Tabs::new(menu_bar)
        .block(Block::default().title("Menu").borders(Borders::ALL))
        .style(Style::default().fg(Color::White))
        .highlight_style(Style::default().fg(Color::Blue))
        .divider(Span::raw("|"));

    let chat_panel = tui::widgets::Paragraph::new("Shaba daba dub dub")
        .style(Style::default().fg(Color::White))
        .alignment(tui::layout::Alignment::Left)
        .block(
            Block::default()
                .border_style(Style::default())
                .borders(Borders::ALL)
                .title("Type here")
                .border_type(widgets::BorderType::Rounded),
        );

    term.draw(|f| {
        f.render_widget(chat_panel, chunks[2]);
        f.render_widget(tabs, chunks[0]);
    })?;

    // let menu_titles = vec!["People", "Quit"];
    // let mut active_menu_item = MenuItem::Home;

    Ok(())
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
