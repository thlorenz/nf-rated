use crate::{data::ItemType, data::GENRE_COLUMN, RatedRow};

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
    pub item_type: ItemType,
    pub logs: Vec<Log>,
    pub column: &'static str,
    pub input_mode: InputMode,
}

impl App {
    pub fn new(rows: Vec<RatedRow>) -> Self {
        Self {
            items: StatefulList::with_items(rows),
            query: "".to_string(),
            item_type: ItemType::Both,
            logs: vec![],
            input_mode: InputMode::Querying,
            column: GENRE_COLUMN.clone(),
        }
    }

    pub fn next_item_type(&mut self) {
        let next_type = match self.item_type {
            ItemType::Movie => ItemType::Series,
            ItemType::Series => ItemType::Both,
            ItemType::Both => ItemType::Movie,
        };
        self.item_type = next_type;
    }
}

