use tui::{
    style::Color, style::Modifier, style::Style, text::Span, text::Spans, widgets::Block,
    widgets::Borders, widgets::List, widgets::ListItem,
};

use crate::RatedRow;

use super::util::get_rating_style;

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

const N_A: &str = "N/A";

fn render_row_summary(row: &RatedRow) -> Vec<ListItem> {
    let bar = Span::raw(" | ");
    let spc = Span::raw(" ");

    assert!(
        row.imdb_rating.is_some(),
        "cannot render row without an imdb rating",
    );
    assert!(
        row.imdb_id.is_some(),
        "cannot render row without an imdb id",
    );
    let rating = row.imdb_rating.unwrap();
    let rating_style = get_rating_style(rating);
    let rating_span = Span::styled(format!(" {:2.1}", rating as f32 / 10.0), rating_style);

    let title_style = Style::default().fg(Color::White);
    let title_span = Span::styled(&row.title, title_style);
    let year_style = Style::default().fg(Color::DarkGray);
    let year_span = Span::styled(format!("({})", row.year), year_style);
    let duration_style = Style::default().fg(Color::DarkGray);
    let duration_span = Span::styled(&row.duration, duration_style);

    let genre_style = Style::default().fg(Color::LightBlue);
    let genre_span = match &row.genre {
        Some(x) => Span::styled(x, genre_style),
        None => Span::styled(N_A, genre_style),
    };

    let cast_style = Style::default().fg(Color::Blue);
    let cast_span = Span::styled(&row.cast, cast_style);

    let plot_style = Style::default().fg(Color::White);
    let plot_span = Span::styled(&row.plot, plot_style);

    let imdblink_style = Style::default().fg(Color::Blue);
    let imdblink_span = Span::styled(
        format!(
            "https://www.imdb.com/title/{}/",
            &row.imdb_id.as_ref().unwrap()
        ),
        imdblink_style,
    );
    let netflixlink_style = Style::default().fg(Color::Blue);
    let netflixlink_span = Span::styled(
        format!("https://www.netflix.com/watch/{:?}/", &row.id),
        netflixlink_style,
    );

    let country_style = Style::default().fg(Color::LightGreen);
    let country_span = Span::styled(&row.country, country_style);

    // TODO: how/where can we add the plot as paragraph instead of a list item
    // let plot_para = Paragraph::new(plot_span).wrap(Wrap { trim: true });

    vec![
        ListItem::new(Spans(vec![
            rating_span,
            bar.clone(),
            duration_span,
            bar.clone(),
            title_span,
            spc.clone(),
            year_span,
        ])),
        ListItem::new(Spans(vec![])),
        ListItem::new(Spans(vec![genre_span, bar, country_span])),
        ListItem::new(cast_span),
        ListItem::new(Spans(vec![])),
        ListItem::new(plot_span),
        ListItem::new(Spans(vec![])),
        ListItem::new(imdblink_span),
        ListItem::new(netflixlink_span),
    ]
}
