use nf_rated::{
    data::Db, render::maybe_render_item_details, render::render_rows_summary, render::Event,
    render::Events, render::StatefulList, RatedRow,
};
use std::{error::Error, io};
use termion::{event::Key, input::MouseTerminal, raw::IntoRawMode, screen::AlternateScreen};
use tui::{
    backend::Backend, backend::TermionBackend, layout::Constraint, layout::Direction,
    layout::Layout, layout::Rect, widgets::Block, widgets::Borders, Frame, Terminal,
};

const PAGE_MARGIN_HEIGHT: i32 = 3;

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

fn render_summary_and_config<B>(f: &mut Frame<B>, app: &mut App, container: Rect)
where
    B: Backend,
{
    let items = render_rows_summary(&app.items.items);

    let config = Block::default().title(" Config ").borders(Borders::ALL);
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(40), Constraint::Percentage(60)].as_ref())
        .split(container);

    f.render_widget(config, chunks[0]);
    f.render_stateful_widget(items, chunks[1], &mut app.items.state);
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
    // app.items.state.select(Some(0));

    let mut current_size: Rect = Default::default();
    loop {
        terminal.draw(|mut f| {
            current_size = f.size();
            let main_container = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Percentage(70), Constraint::Percentage(30)].as_ref())
                .split(current_size);

            let summary_and_config_container = main_container[0];
            let item_details_container = main_container[1];

            render_summary_and_config(&mut f, &mut app, summary_and_config_container);

            let selected_idx = app.items.state.selected();
            let item_details = if selected_idx.is_none() {
                maybe_render_item_details(None)
            } else {
                maybe_render_item_details(app.items.items.get(selected_idx.unwrap()))
            };
            f.render_widget(item_details, item_details_container);
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
                    app.items
                        .next_page((current_size.height as i32 - PAGE_MARGIN_HEIGHT).max(1));
                }
                Key::Ctrl('u') => {
                    app.items
                        .previous_page((current_size.height as i32 - PAGE_MARGIN_HEIGHT).max(1));
                }
                _ => {}
            },
            _ => {}
        }
    }

    Ok(())
}
