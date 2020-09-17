use tui::{
    backend::Backend, layout::Constraint, layout::Direction, layout::Layout, layout::Rect,
    style::Color, style::Style, text::Span, text::Spans, widgets::Block, widgets::Borders,
    widgets::Paragraph, Frame,
};

use crate::{
    data::ItemType,
    render::{App, InputMode},
};
use tui::widgets::BorderType;

pub fn render_admin<B>(f: &mut Frame<B>, app: &App, container: Rect)
where
    B: Backend,
{
    let query = render_query(&app.query, app.input_mode.clone());

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Percentage(80)].as_ref())
        .split(container);
    let query_container = chunks[0];
    f.render_widget(query, query_container);

    f.set_cursor(
        // Put cursor past the end of the input text
        query_container.x + app.query.len() as u16 + 1,
        // Move one line down, from the border to the input line
        query_container.y + 1,
    );
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

fn _render_config(item_type: &ItemType, _container: Rect) -> Spans {
    let value = match item_type {
        ItemType::Movie => "movie",
        ItemType::Series => "series",
        ItemType::Both => "both",
    };
    let value_style = Style::default().fg(Color::LightBlue);
    let value_span = Span::styled(value, value_style);

    Spans(vec![value_span])
}
