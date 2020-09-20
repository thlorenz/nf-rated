use tui::{
    backend::Backend,
    layout::Constraint,
    layout::Direction,
    layout::Layout,
    layout::Rect,
    style::{Color, Style},
    text::Span,
    widgets::Block,
    widgets::BorderType,
    widgets::Borders,
    widgets::Paragraph,
    widgets::Row,
    widgets::Table,
    Frame,
};

use crate::{data::ItemType, render::App, render::QueryField};

pub fn query_offset(query_field: &QueryField) -> u16 {
    3 * match query_field {
        QueryField::Genre => 0,
        QueryField::Title => 1,
        QueryField::Cast => 2,
        QueryField::Country => 3,
        QueryField::Language => 4,
        QueryField::Plot => 5,
    } + 2
}

pub fn render_admin<B>(f: &mut Frame<B>, app: &App, container: Rect)
where
    B: Backend,
{
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(100)].as_ref())
        .split(container);
    let config_container = chunks[0];

    f.set_cursor(
        // Put cursor past the end of the input text
        config_container.x + app.get_query().len() as u16 + 1,
        // Move one line down, from the border to the input line
        config_container.y + query_offset(&app.query_field),
    );

    render_config(f, &app, config_container);
}

pub fn render_config<B>(f: &mut Frame<B>, app: &App, container: Rect)
where
    B: Backend,
{
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Length(1),
                Constraint::Length(3),
                Constraint::Length(3),
                Constraint::Length(3),
                Constraint::Length(3),
                Constraint::Length(3),
                Constraint::Length(3),
                Constraint::Length(10),
                Constraint::Length(10),
            ]
            .as_ref(),
        )
        .split(container);

    let item_type_container = chunks[0];
    let genre_query_container = chunks[1];
    let title_query_container = chunks[2];
    let cast_query_container = chunks[3];
    let country_query_container = chunks[4];
    let language_query_container = chunks[5];
    let plot_query_container = chunks[6];
    let keyboard_shortcuts_container = chunks[7];
    let queries_container = chunks[8];

    let item_type_ui = render_item_type(&app.item_type);
    f.render_widget(item_type_ui, item_type_container);

    let genre_query_ui = render_query(
        "Genre",
        &app.genre_query,
        &app.query_field == &QueryField::Genre,
    );
    f.render_widget(genre_query_ui, genre_query_container);

    let title_query_ui = render_query(
        "Title",
        &app.title_query,
        &app.query_field == &QueryField::Title,
    );
    f.render_widget(title_query_ui, title_query_container);

    let cast_query_ui = render_query(
        "Cast",
        &app.cast_query,
        &app.query_field == &QueryField::Cast,
    );
    f.render_widget(cast_query_ui, cast_query_container);

    let country_query_ui = render_query(
        "Country",
        &app.country_query,
        &app.query_field == &QueryField::Country,
    );
    f.render_widget(country_query_ui, country_query_container);

    let language_query_ui = render_query(
        "Language",
        &app.language_query,
        &app.query_field == &QueryField::Language,
    );
    f.render_widget(language_query_ui, language_query_container);

    let plot_query_ui = render_query(
        "Plot",
        &app.plot_query,
        &app.query_field == &QueryField::Plot,
    );
    f.render_widget(plot_query_ui, plot_query_container);

    render_keyboard_shortcuts(f, keyboard_shortcuts_container);
    render_queries_legend(f, queries_container);
}

fn render_keyboard_shortcuts<B>(f: &mut Frame<B>, container: Rect)
where
    B: Backend,
{
    let shortcut_style = Style::default().fg(Color::Gray);
    let table_style = Style::default().fg(Color::DarkGray);

    let header = ["Shortcut", "Description"];
    let items = vec![
        vec!["<Tab>", "Cycle Filters"],
        vec!["↓/↑ | Ctrl-N/Ctrl-P", "Select Next/Previous Show"],
        vec!["Ctrl-D/Ctrl-U", "Select Next/Previous Page"],
        vec!["Ctrl-E", "Erase all Filters"],
        vec!["Ctrl-O", "Change Type of Show"],
    ];

    let rows = items
        .iter()
        .map(|i| Row::StyledData(i.iter(), shortcut_style));
    let t = Table::new(header.iter(), rows)
        .style(table_style)
        .block(Block::default().borders(Borders::ALL))
        .widths(&[Constraint::Percentage(30), Constraint::Percentage(70)]);

    f.render_widget(t, container);
}

fn render_queries_legend<B>(f: &mut Frame<B>, container: Rect)
where
    B: Backend,
{
    let shortcut_style = Style::default().fg(Color::Gray);
    let table_style = Style::default().fg(Color::DarkGray);

    let header = ["Query", "Description"];
    let items = vec![
        vec![
            "drama adventure",
            "Match all that has 'drama' or 'adventure'",
        ],
        vec!["!comedy", "Match all that does not have 'comedy'"],
    ];

    let rows = items
        .iter()
        .map(|i| Row::StyledData(i.iter(), shortcut_style));
    let t = Table::new(header.iter(), rows)
        .style(table_style)
        .block(Block::default().borders(Borders::ALL))
        .widths(&[Constraint::Percentage(30), Constraint::Percentage(70)]);

    f.render_widget(t, container);
}

fn render_item_type(item_type: &ItemType) -> Paragraph {
    let value = match item_type {
        ItemType::Movie => "Movies",
        ItemType::Series => "Series",
        ItemType::Both => "Movies and Series",
    };
    let value_style = Style::default().fg(Color::LightBlue);
    let value_span = Span::styled(value, value_style);

    Paragraph::new(value_span)
}

fn render_query<'a>(label: &'a str, query: &'a str, selected: bool) -> Paragraph<'a> {
    let label_style = Style::default().fg(Color::Gray);
    let border_style = if selected {
        Style::default().fg(Color::Yellow)
    } else {
        Style::default().fg(Color::White)
    };

    let query_style = Style::default().fg(Color::White);
    let query_span = Span::styled(query, query_style);

    Paragraph::new(query_span).block(
        Block::default()
            .borders(Borders::ALL)
            .border_style(border_style)
            .border_type(BorderType::Rounded)
            .style(label_style)
            .title(label),
    )
}
