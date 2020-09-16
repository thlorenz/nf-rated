use nf_rated::{
    data::Db, render::maybe_render_item_details, render::render_admin, render::render_rows_summary,
    render::App, render::Event, render::Events,
};
use std::{error::Error, io};
use termion::{event::Key, input::MouseTerminal, raw::IntoRawMode, screen::AlternateScreen};
use tui::{
    backend::Backend, backend::TermionBackend, layout::Constraint, layout::Direction,
    layout::Layout, layout::Rect, Frame, Terminal,
};
use nf_rated::render::StatefulList;

const PAGE_MARGIN_HEIGHT: i32 = 3;

fn render_summary_and_admin<B>(f: &mut Frame<B>, app: &mut App, container: Rect)
where
    B: Backend,
{
    let items = render_rows_summary(&app.items.items);

    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(40), Constraint::Percentage(60)].as_ref())
        .split(container);

    let admin_container = chunks[0];
    let summary_container = chunks[1];

    render_admin(f, app, admin_container);
    let list_state = &mut app.items.state;
    f.render_stateful_widget(items, summary_container, list_state);
}

fn exec_query(app: &mut App, db: &Db) -> Result<(), Box<dyn Error>> {
    let rows =  match app.query.len() {
        0 => db.get_synced_rows_sorted()?,
        _ => db.get_synced_rows_for_genre_sorted(&app.query)?,
    };
    app.items.unselect();
    app.items.items = rows;
    if !app.items.items.is_empty() {
        app.items.next()
    }
    Ok(())
}


fn main() -> Result<(), Box<dyn Error>> {
    let stdout = io::stdout().into_raw_mode()?;
    let stdout = MouseTerminal::from(stdout);
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    let events = Events::new();

    let db = Db::new()?;
    let all_rows = db.get_synced_rows_sorted()?;
    let mut app = App::new(all_rows);
    app.items.state.select(Some(0));

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

            render_summary_and_admin(&mut f, &mut app, summary_and_config_container);

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
                Key::Backspace => {
                    app.query.pop();
                    exec_query(&mut app, &db)?;
                },
                Key::Char(c) => {
                    app.query.push(c);
                    exec_query(&mut app, &db)?;
                }
                _ => { }

            },
            _ => {}
        }
    }

    Ok(())
}
