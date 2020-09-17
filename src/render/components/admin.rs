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
    Frame,
};

use crate::{
    data::ItemType,
    render::{App, InputMode},
};

pub fn render_admin<B>(f: &mut Frame<B>, app: &App, container: Rect)
where
    B: Backend,
{
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Percentage(80)].as_ref())
        .split(container);
    let query_container = chunks[0];
    let config_container = chunks[1];

    let query = render_query(&app.query, app.input_mode.clone());
    f.render_widget(query, query_container);
    f.set_cursor(
        // Put cursor past the end of the input text
        query_container.x + app.query.len() as u16 + 1,
        // Move one line down, from the border to the input line
        query_container.y + 1,
    );

    render_config(f, &app, config_container);
}

fn render_query(query: &str, input_mode: InputMode) -> Paragraph {
    let input = Paragraph::new(query)
        .style(match input_mode {
            InputMode::Configuring => Style::default(),
            InputMode::Querying => Style::default().fg(Color::Yellow),
        })
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .title("Query"),
        );
    input
}

pub fn render_config<B>(f: &mut Frame<B>, app: &App, container: Rect)
where
    B: Backend,
{
    let item_type_ui = render_item_type(&app.item_type);
    f.render_widget(item_type_ui, container);
}

fn render_item_type(item_type: &ItemType) -> Paragraph {
    let value = match item_type {
        ItemType::Movie => "Movie",
        ItemType::Series => "Series",
        ItemType::Both => "Movie and Series",
    };
    let value_style = Style::default().fg(Color::LightBlue);
    let value_span = Span::styled(value, value_style);

    Paragraph::new(value_span).block(
        Block::default()
            .borders(Borders::NONE)
            .title("Type of Show <Ctrl-O>"),
    )
}
