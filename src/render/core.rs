use crate::{data::is_valid_query_filter, data::ItemType, RatedRow};

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

#[derive(Clone, PartialEq)]
pub enum QueryField {
    Genre,
    Title,
    Cast,
    Country,
    Language,
    Plot,
}

impl From<QueryField> for String {
    fn from(field: QueryField) -> Self {
        match field {
            QueryField::Genre => "Genre".to_string(),
            QueryField::Title => "Title".to_string(),
            QueryField::Cast => "Cast".to_string(),
            QueryField::Country => "Country".to_string(),
            QueryField::Language => "Language".to_string(),
            QueryField::Plot => "Plot".to_string(),
        }
    }
}

pub struct App {
    pub items: StatefulList<RatedRow>,

    pub query_field: QueryField,
    pub genre_query: String,
    pub title_query: String,
    pub cast_query: String,
    pub country_query: String,
    pub language_query: String,
    pub plot_query: String,

    pub item_type: ItemType,
    pub logs: Vec<Log>,
    pub input_mode: InputMode,
}

impl App {
    pub fn new(rows: Vec<RatedRow>) -> Self {
        Self {
            items: StatefulList::with_items(rows),

            query_field: QueryField::Genre,
            genre_query: "".to_string(),
            title_query: "".to_string(),
            cast_query: "".to_string(),
            country_query: "".to_string(),
            language_query: "".to_string(),
            plot_query: "".to_string(),

            item_type: ItemType::Both,
            logs: vec![],
            input_mode: InputMode::Querying,
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

    pub fn next_query_field(&mut self) {
        let next_query_field = match self.query_field {
            QueryField::Genre => QueryField::Title,
            QueryField::Title => QueryField::Cast,
            QueryField::Cast => QueryField::Country,
            QueryField::Country => QueryField::Language,
            QueryField::Language => QueryField::Plot,
            QueryField::Plot => QueryField::Genre,
        };
        self.query_field = next_query_field
    }

    pub fn prev_query_field(&mut self) {
        let prev_query_field = match self.query_field {
            QueryField::Plot => QueryField::Language,
            QueryField::Country => QueryField::Cast,
            QueryField::Language => QueryField::Country,
            QueryField::Cast => QueryField::Title,
            QueryField::Title => QueryField::Genre,
            QueryField::Genre => QueryField::Plot,
        };
        self.query_field = prev_query_field
    }

    pub fn get_query(&self) -> &str {
        match self.query_field {
            QueryField::Genre => &self.genre_query,
            QueryField::Title => &self.title_query,
            QueryField::Cast => &self.cast_query,
            QueryField::Country => &self.country_query,
            QueryField::Language => &self.language_query,
            QueryField::Plot => &self.plot_query,
        }
    }

    pub fn push_onto_query(&mut self, c: char) {
        match self.query_field {
            QueryField::Genre => self.genre_query.push(c),
            QueryField::Title => self.title_query.push(c),
            QueryField::Cast => self.cast_query.push(c),
            QueryField::Country => self.country_query.push(c),
            QueryField::Language => self.language_query.push(c),
            QueryField::Plot => self.plot_query.push(c),
        };
    }

    pub fn pop_off_query(&mut self) {
        match self.query_field {
            QueryField::Genre => self.genre_query.pop(),
            QueryField::Title => self.title_query.pop(),
            QueryField::Cast => self.cast_query.pop(),
            QueryField::Country => self.country_query.pop(),
            QueryField::Language => self.language_query.pop(),
            QueryField::Plot => self.plot_query.pop(),
        };
    }

    pub fn has_any_query(&self) -> bool {
        vec![
            &self.genre_query,
            &self.title_query,
            &self.cast_query,
            &self.country_query,
            &self.language_query,
            &self.plot_query,
        ]
        .iter()
        .any(|&q| is_valid_query_filter(q))
    }
}

