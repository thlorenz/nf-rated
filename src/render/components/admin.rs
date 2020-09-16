use tui::{
    backend::Backend, layout::Constraint, layout::Direction, layout::Layout, layout::Rect,
    style::Color, style::Style, widgets::Block, widgets::Borders, widgets::Paragraph, Frame,
};

use crate::render::{App, InputMode};

/*
 * pub fn render_config() -> Paragraph { }
 */

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
}

fn render_query(query: &str, input_mode: InputMode) -> Paragraph {
    let input = Paragraph::new(query)
        .style(match input_mode {
            InputMode::Configuring => Style::default(),
            InputMode::Querying => Style::default().fg(Color::Yellow),
        })
        .block(Block::default().borders(Borders::ALL).title("Query"));
    input
}
