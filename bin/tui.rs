use crossterm::{
    event::{poll, read, Event, KeyCode, KeyEvent, KeyModifiers},
    terminal::disable_raw_mode,
    terminal::enable_raw_mode,
};
use nf_rated::{
    data::build_sorted_filtered_query, data::build_sorted_query, data::Db, data::CAST_COLUMN,
    data::COUNTRY_COLUMN, data::GENRE_COLUMN, data::LANGUAGE_COLUMN, data::PLOT_COLUMN,
    data::TITLE_COLUMN, render::maybe_render_item_details, render::render_admin,
    render::render_log, render::render_rows_summary, render::App, render::Log,
};
use std::{error::Error, io::stdout, time::Duration};
use tui::{
    backend::Backend, backend::CrosstermBackend, layout::Constraint, layout::Direction,
    layout::Layout, layout::Rect, Frame, Terminal,
};

const PAGE_MARGIN_HEIGHT: i32 = 3;

fn render_summary_and_admin<B>(f: &mut Frame<B>, app: &mut App, container: Rect) -> (Rect, Rect)
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

    (admin_container, summary_container)
}

fn exec_query(app: &mut App, db: &Db) -> Result<(), Box<dyn Error>> {
    let rows = if app.has_any_query() {
        let q = build_sorted_filtered_query(
            vec![
                (GENRE_COLUMN, &app.genre_query).into(),
                (TITLE_COLUMN, &app.title_query).into(),
                (CAST_COLUMN, &app.cast_query).into(),
                (COUNTRY_COLUMN, &app.country_query).into(),
                (LANGUAGE_COLUMN, &app.language_query).into(),
                (PLOT_COLUMN, &app.plot_query).into(),
            ],
            &app.item_type,
        );
        app.logs.push(Log::Debug(q.to_string()));

        match db.get_no_params_query_result(&q) {
            Ok(rows) => Ok(rows),
            Err(err) => {
                app.logs.push(Log::Error(err.to_string()));
                db.get_synced_rows_sorted()
            }
        }
    } else {
        let q = build_sorted_query(&app.item_type);
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

    enable_raw_mode()?;

    let stdout = stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let db = Db::new()?;
    let all_rows = db.get_synced_rows_sorted()?;
    let mut app = App::new(all_rows);
    app.items.state.select(Some(0));

    let mut current_summary_size: Rect = Default::default();
    let constraints = if _show_log {
        vec![
            Constraint::Percentage(60),
            Constraint::Percentage(25),
            Constraint::Percentage(15),
        ]
    } else {
        vec![Constraint::Percentage(70), Constraint::Percentage(30)]
    };

    terminal.clear()?;
    loop {
        terminal.draw(|mut f| {
            let main_container = Layout::default()
                .direction(Direction::Vertical)
                .constraints(constraints.as_ref())
                .split(f.size());

            let (summary_and_config_container, item_details_container, log_container) = if _show_log
            {
                (main_container[0], main_container[1], main_container[2])
            } else {
                (main_container[0], main_container[1], main_container[1])
            };

            let (_, summary_container) =
                render_summary_and_admin(&mut f, &mut app, summary_and_config_container);
            current_summary_size = summary_container;

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

        if poll(Duration::from_millis(200))? {
            let event = read()?;
            match event {
                //
                // Quit
                //
                Event::Key(KeyEvent {
                    modifiers: KeyModifiers::NONE,
                    code: KeyCode::Esc,
                })
                | Event::Key(KeyEvent {
                    modifiers: KeyModifiers::CONTROL,
                    code: KeyCode::Char('c'),
                }) => {
                    break;
                }

                //
                // Navigate list by item
                //
                Event::Key(KeyEvent {
                    modifiers: KeyModifiers::CONTROL,
                    code: KeyCode::Char('n'),
                })
                | Event::Key(KeyEvent {
                    modifiers: KeyModifiers::NONE,
                    code: KeyCode::Down,
                }) => {
                    app.items.next();
                }
                Event::Key(KeyEvent {
                    modifiers: KeyModifiers::CONTROL,
                    code: KeyCode::Char('p'),
                })
                | Event::Key(KeyEvent {
                    modifiers: KeyModifiers::NONE,
                    code: KeyCode::Up,
                }) => {
                    app.items.previous();
                }

                //
                // Navigate list by page
                //
                Event::Key(KeyEvent {
                    modifiers: KeyModifiers::CONTROL,
                    code: KeyCode::Char('d'),
                }) => {
                    app.items.next_page(
                        (current_summary_size.height as i32 - PAGE_MARGIN_HEIGHT).max(1),
                    );
                }
                Event::Key(KeyEvent {
                    modifiers: KeyModifiers::CONTROL,
                    code: KeyCode::Char('u'),
                }) => {
                    app.items.previous_page(
                        (current_summary_size.height as i32 - PAGE_MARGIN_HEIGHT).max(1),
                    );
                }

                //
                // Configure item type
                //
                Event::Key(KeyEvent {
                    modifiers: KeyModifiers::CONTROL,
                    code: KeyCode::Char('o'),
                }) => {
                    app.next_item_type();
                    exec_query(&mut app, &db)?;
                }

                //
                // Navigate filter inputs
                //
                Event::Key(KeyEvent {
                    modifiers: KeyModifiers::NONE,
                    code: KeyCode::Tab,
                }) => {
                    app.next_query_field();
                }
                Event::Key(KeyEvent {
                    modifiers: KeyModifiers::NONE,
                    code: KeyCode::BackTab,
                }) => {
                    app.logs.push(Log::Info("left".to_string()));
                    app.prev_query_field();
                }

                //
                // Enter query
                //
                Event::Key(KeyEvent {
                    modifiers: KeyModifiers::NONE,
                    code: KeyCode::Backspace,
                }) => {
                    app.pop_off_query();
                    exec_query(&mut app, &db)?;
                }
                Event::Key(KeyEvent {
                    modifiers: KeyModifiers::NONE,
                    code: KeyCode::Char(c),
                })
                | Event::Key(KeyEvent {
                    modifiers: KeyModifiers::SHIFT,
                    code: KeyCode::Char(c),
                }) => {
                    app.push_onto_query(c);
                    exec_query(&mut app, &db)?;
                }
                Event::Key(KeyEvent {
                    modifiers: KeyModifiers::CONTROL,
                    code: KeyCode::Char('e'),
                }) => {
                    app.logs.push(Log::Info("clearing all queries".to_string()));
                    app.clear_all_queries();
                    exec_query(&mut app, &db)?;
                }
                _ => {}
            }
        }
    }

    terminal.clear()?;
    terminal.set_cursor(0, 0)?;

    disable_raw_mode()?;
    Ok(())
}
