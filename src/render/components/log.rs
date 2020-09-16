use tui::{
    style::Color,
    style::Style,
    text::Span,
    text::Spans,
    widgets::Block,
    widgets::Borders,
    widgets::{List, ListItem},
};

use crate::render::Log;

pub fn render_log(logs: &Vec<Log>) -> List {
    let items: Vec<ListItem> = logs.iter().rev().map(render_log_entry).collect();
    List::new(items).block(Block::default().borders(Borders::ALL))
}

fn render_log_entry(entry: &Log) -> ListItem {
    let (prefix, msg, style) = match entry {
        Log::Error(msg) => ("ERR ", msg, Style::default().fg(Color::Red)),
        Log::Warn(msg) => ("WARN", msg, Style::default().fg(Color::Blue)),
        Log::Info(msg) => ("INFO", msg, Style::default().fg(Color::Yellow)),
        Log::Debug(msg) => ("DEBG", msg, Style::default().fg(Color::Gray)),
    };
    let prefix_span = Span::styled(prefix, style);
    let msg_span = Span::styled(msg, Style::default().fg(Color::White));
    let bar = Span::raw(" | ");

    ListItem::new(Spans(vec![prefix_span, bar, msg_span]))
}
