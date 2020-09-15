use crate::{render::render_row_summary, RatedRow};
use tui::{
    style::Color, style::Modifier, style::Style, widgets::Block, widgets::Borders, widgets::List,
    widgets::ListItem,
};

pub fn render_rows_summary(rows: &Vec<RatedRow>) -> List {
    let rendered_rows: Vec<ListItem> = rows.iter().map(|row| render_row_summary(row)).collect();

    List::new(rendered_rows)
        .block(Block::default().borders(Borders::ALL))
        .highlight_style(
            Style::default()
                .bg(Color::DarkGray)
                .add_modifier(Modifier::BOLD),
        )
}
