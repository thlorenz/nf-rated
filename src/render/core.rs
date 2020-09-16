use crate::{data::GENRE_COLUMN, RatedRow};

use super::StatefulList;

#[derive(Clone)]
pub enum InputMode {
    Querying,
    Configuring,
}

#[derive(Clone)]
pub enum Log {
    Error(String),
    Warn(String),
    Info(String),
    Debug(String),
}

pub struct App {
    pub items: StatefulList<RatedRow>,
    pub query: String,
    pub logs: Vec<Log>,
    pub column: &'static str,
    pub input_mode: InputMode,
}

impl App {
    pub fn new(rows: Vec<RatedRow>) -> Self {
        Self {
            items: StatefulList::with_items(rows),
            query: "".to_string(),
            logs: vec![],
            input_mode: InputMode::Querying,
            column: GENRE_COLUMN.clone(),
        }
    }
}

