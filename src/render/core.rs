use crate::RatedRow;

use super::StatefulList;

#[derive(Clone)]
pub enum InputMode {
    Querying,
    Configuring,
}

pub struct App {
    pub items: StatefulList<RatedRow>,
    pub query: String,
    pub input_mode: InputMode,
}

impl App {
    pub fn new(rows: Vec<RatedRow>) -> Self {
        Self {
            items: StatefulList::with_items(rows),
            query: "".to_string(),
            input_mode: InputMode::Querying,
        }
    }
}

