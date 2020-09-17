use nf_rated::{
    data::build_sorted_filtered_query, data::build_sorted_query, data::Db,
    render::maybe_render_item_details, render::render_admin, render::render_log,
    render::render_rows_summary, render::App, render::Event, render::Events, render::Log,
};
use std::{error::Error, io};
use termion::{event::Key, input::MouseTerminal, raw::IntoRawMode, screen::AlternateScreen};
use tui::{
    backend::Backend, backend::TermionBackend, layout::Constraint, layout::Direction,
    layout::Layout, layout::Rect, Frame, Terminal,
};

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
    let rows = if app.genre_query.is_empty() {
        let q = build_sorted_query(&app.item_type);
        app.logs.push(Log::Debug(q.to_string()));

        match db.get_no_params_query_result(&q) {
            Ok(rows) => Ok(rows),
            Err(err) => {
                app.logs.push(Log::Error(err.to_string()));
                db.get_synced_rows_sorted()
            }
        }
    } else {
        let q = build_sorted_filtered_query(&app.column, &app.genre_query, &app.item_type);
        app.logs.push(Log::Debug(q.to_string()));

        match db.get_no_params_query_result(&q) {
            Ok(rows) => Ok(rows),
            Err(err) => {
                app.logs.push(Log::Error(err.to_string()));
                db.get_synced_rows_sorted()
            }
        }
    }?;

    app.items.unselect();
    app.items.items = rows;
    if !app.items.items.is_empty() {
        app.items.next()
    }
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let _show_log: bool = false;
    #[cfg(feature = "log")]
    let _show_log: bool = true;

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
    let constraints = if _show_log {
        vec![
            Constraint::Percentage(60),
            Constraint::Percentage(25),
            Constraint::Percentage(15),
        ]
    } else {
        vec![Constraint::Percentage(70), Constraint::Percentage(30)]
    };

    loop {
        terminal.draw(|mut f| {
            current_size = f.size();
            let main_container = Layout::default()
                .direction(Direction::Vertical)
                .constraints(constraints.as_ref())
                .split(current_size);

            let (summary_and_config_container, item_details_container, log_container) = if _show_log
            {
                (main_container[0], main_container[1], main_container[2])
            } else {
                (main_container[0], main_container[1], main_container[1])
            };

            render_summary_and_admin(&mut f, &mut app, summary_and_config_container);

            let selected_idx = app.items.state.selected();
            let item_details = if selected_idx.is_none() {
                maybe_render_item_details(None)
            } else {
                maybe_render_item_details(app.items.items.get(selected_idx.unwrap()))
            };
            f.render_widget(item_details, item_details_container);

            if _show_log {
                f.render_widget(render_log(&app.logs), log_container)
            };
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
                Key::Ctrl('o') => {
                    app.next_item_type();
                    exec_query(&mut app, &db)?;
                }
                Key::Right => {
                    app.next_query_field();
                }
                Key::Left => {
                    app.prev_query_field();
                }
                Key::Char(c) => {
                    app.push_onto_query(c);
                    exec_query(&mut app, &db)?;
                }
                Key::Backspace => {
                    app.pop_off_query();
                    exec_query(&mut app, &db)?;
                }
                _ => {}
            },
            _ => {}
        }
    }

    Ok(())
}
