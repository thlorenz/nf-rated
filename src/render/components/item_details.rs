use tui::{
    style::Color, style::Modifier, style::Style, text::Span, text::Spans, widgets::Block,
    widgets::Borders, widgets::List, widgets::ListItem,
};

use crate::RatedRow;

pub fn maybe_render_item_details(row: Option<&RatedRow>) -> List {
    // TODO: forget about the block for now but need to fix ASAP
    // let block = Block::default().title("Details").borders(Borders::ALL);
    let items: Vec<ListItem> = match row {
        Some(row) => render_row_summary(row),
        None => render_please_select_row(),
    };

    List::new(items)
        .block(Block::default().borders(Borders::ALL))
        .highlight_style(
            Style::default()
                .bg(Color::DarkGray)
                .add_modifier(Modifier::BOLD),
        )
}

fn render_please_select_row() -> Vec<ListItem<'static>> {
    let header = Spans::from(vec![Span::raw("Please select a Movie or Show")]);
    vec![ListItem::new(vec![header]).to_owned()]
}

fn render_row_summary(row: &RatedRow) -> Vec<ListItem> {
    // TODO: For now just rendering summary but need to render details
    let bar = Span::raw(" | ");

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
    let header = Spans::from(vec![rating_span, bar, title_span]);

    vec![ListItem::new(vec![header])]
}

