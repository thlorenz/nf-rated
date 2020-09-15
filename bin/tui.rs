use nf_rated::{data::Db, render::Event, render::Events, render::StatefulList, RatedRow};
use std::{error::Error, io};
use termion::{event::Key, input::MouseTerminal, raw::IntoRawMode, screen::AlternateScreen};
use tui::{
    backend::TermionBackend, layout::Constraint, layout::Direction, layout::Layout, style::Color,
    style::Modifier, style::Style, text::Span, text::Spans, widgets::Block, widgets::Borders,
    widgets::List, widgets::ListItem, Terminal,
};

struct App {
    items: StatefulList<RatedRow>,
}

impl App {
    fn new(rows: Vec<RatedRow>) -> Self {
        Self {
            items: StatefulList::with_items(rows),
        }
    }
}

fn render_row(row: &RatedRow) -> ListItem {
    assert!(
        row.imdb_rating.is_some(),
        "cannot render row without a rating",
    );
    let rating = row.imdb_rating.unwrap();
    let rating_style = match rating {
        n if n >= 90 => Style::default().fg(Color::LightGreen),
        n if n >= 80 => Style::default().fg(Color::Green),
        n if n >= 70 => Style::default().fg(Color::LightYellow),
        n if n >= 60 => Style::default().fg(Color::Yellow),
        n if n >= 50 => Style::default().fg(Color::LightBlue),
        n if n >= 40 => Style::default().fg(Color::LightRed),
        _ => Style::default().fg(Color::LightRed),
    };
    let rating_span = Span::styled(format!("{:2.1}", rating as f32 / 10.0), rating_style);

    let title_style = Style::default().fg(Color::White);
    let title_span = Span::styled(&row.title, title_style);
    let header = Spans::from(vec![rating_span, Span::raw(" | "), title_span]);

    ListItem::new(vec![header])
}

fn main() -> Result<(), Box<dyn Error>> {
    let stdout = io::stdout().into_raw_mode()?;
    let stdout = MouseTerminal::from(stdout);
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    let events = Events::new();

    let db = Db::new()?;
    let all_rows = db.get_synced_rows_sorted_by_rating()?;
    let mut app = App::new(all_rows);

    loop {
        terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(100)].as_ref())
                .split(f.size());

            let rendered_rows: Vec<ListItem> =
                app.items.items.iter().map(|row| render_row(row)).collect();

            let items = List::new(rendered_rows)
                .block(Block::default().borders(Borders::ALL))
                .highlight_style(
                    Style::default()
                        .bg(Color::DarkGray)
                        .add_modifier(Modifier::BOLD),
                );
            f.render_stateful_widget(items, chunks[0], &mut app.items.state);
        })?;

        match events.next()? {
            Event::Input(input) => match input {
                Key::Esc => {
                    break;
                }
                Key::Down | Key::Ctrl('n') => {
                    app.items.next();
                }
                Key::Up | Key::Ctrl('p') => {
                    app.items.previous();
                }
                Key::Ctrl('d') => {
                    app.items.next_page();
                }
                Key::Ctrl('u') => {
                    app.items.previous_page();
                }
                _ => {}
            },
            Event::Tick => {
                // TODO: advance app?
            }
        }
    }

    Ok(())
}
