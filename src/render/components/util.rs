use tui::style::{Color, Style};

pub fn get_rating_style(rating: u32) -> Style {
    match rating {
        n if n >= 90 => Style::default().fg(Color::LightGreen),
        n if n >= 80 => Style::default().fg(Color::Green),
        n if n >= 70 => Style::default().fg(Color::LightYellow),
        n if n >= 60 => Style::default().fg(Color::Yellow),
        n if n >= 50 => Style::default().fg(Color::LightBlue),
        n if n >= 40 => Style::default().fg(Color::LightRed),
        _ => Style::default().fg(Color::LightRed),
    }
}
