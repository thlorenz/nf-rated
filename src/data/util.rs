use std::{error::Error, path::PathBuf, time::SystemTime};

use app_dirs::{get_app_root, AppDataType, AppInfo};
const CREATE_SECS: u64 = 1599939357;

pub fn secs_since_creation() -> u32 {
    let since_epoch = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs();

    // This hack will work for about 136 years
    (since_epoch - CREATE_SECS) as u32
}

pub enum QueryTerm {
    And(String),
    Not(String),
}
pub fn destructure_query_filter(q: &str) -> Vec<QueryTerm> {
    let terms = q.split_ascii_whitespace();
    terms
        .filter_map(|term| match term.chars().nth(0) {
            Some(c) if c == '!' => {
                let q = term.split_at(1).1;
                if q.is_empty() {
                    None
                } else {
                    Some(QueryTerm::Not(q.to_string()))
                }
            }
            Some(_) => Some(QueryTerm::And(term.to_string())),
            None => None,
        })
        .collect()
}

pub fn is_valid_query_filter(q: &str) -> bool {
    !destructure_query_filter(q).is_empty()
}

const APP_INFO: AppInfo = AppInfo {
    name: "nf-rated",
    author: "thlorenz",
};

#[derive(Debug)]
pub struct DatabaseInfo {
    pub db_exists: bool,
    pub folder_exists: bool,
    pub folder: PathBuf,
    pub db_path: PathBuf,
}

pub fn get_database_info() -> Result<DatabaseInfo, Box<dyn Error>> {
    let data_root = get_app_root(AppDataType::UserData, &APP_INFO)?;
    let folder_exists = data_root.exists();
    let db_path = data_root.join("nf_rated.sqlite");
    let db_exists = db_path.exists();
    Ok(DatabaseInfo {
        folder_exists,
        db_exists,
        folder: data_root,
        db_path,
    })
}
