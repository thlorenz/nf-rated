use tui::{style::Color, style::Style, text::Span, text::Spans, widgets::ListItem};

use crate::RatedRow;

fn shortened_type(typ: &str) -> &str {
    let s = typ.to_lowercase();
    match &s[..] {
        "movie" => "M",
        "series" => "S",
        _ => "X",
    }
}

pub fn render_row_summary(row: &RatedRow) -> ListItem {
    let bar = Span::raw(" | ");
    let spc = Span::raw(" ");

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
    let typ_style = Style::default().fg(Color::Magenta);
    let typ_span = Span::styled(shortened_type(&row.typ), typ_style);
    let header = Spans::from(vec![typ_span, spc, rating_span, bar, title_span]);

    ListItem::new(vec![header])
}

