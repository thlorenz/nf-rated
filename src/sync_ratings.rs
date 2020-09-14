use nf_rated::{
    core::secs_since_creation, core::JsonRow, core::OmdbErrorResponseJson,
    core::OmdbSuccessResponseJson, core::RatedRow, db::delete_row, db::get_unsynced_rows,
    db::sync_row,
};
use reqwest::blocking::get;
use rusqlite::Connection;
use std::{env, error::Error};

const RATE_LIMIT: usize = 2000;

fn get_api_key() -> String {
    env::var("OMDB_KEY").expect(
        "Please add an OMDB (http://www.omdbapi.com/) API key as 'OMDB_KEY' to your environment",
    )
}

fn reached_rate_limit(json: &OmdbErrorResponseJson) -> bool {
    json.Error.contains("limit reached")
}

fn not_found(json: &OmdbErrorResponseJson) -> bool {
    json.Error.contains("not found")
}

fn request_imdb_data_for_title(api_key: &str, title: &str) -> Result<String, Box<dyn Error>> {
    let uri = format!("http://www.omdbapi.com/?apikey={}&t={}", api_key, title);
    let res = get(&uri).expect("could not get uri");
    let txt = res.text()?;
    Ok(txt)
}

enum SyncImdbResultType {
    Success,
    RateLimitExceeded,
    MissingImdbData,
    NotFound,
    NoResponse,
    UnknownError,
}
struct SyncImdbResult {
    typ: SyncImdbResultType,
    row: Option<JsonRow>,
}

fn sync_imdb_title(api_key: &str, title: &str) -> SyncImdbResult {
    match request_imdb_data_for_title(api_key, title) {
        Ok(text) => {
            let omdb_json: Option<OmdbSuccessResponseJson> =
                serde_json::from_str(&text).unwrap_or(None);
            match omdb_json {
                Some(json) => {
                    let row: JsonRow = json.into();
                    if row.is_missing_imdb_data() {
                        SyncImdbResult {
                            typ: SyncImdbResultType::MissingImdbData,
                            row: None,
                        }
                    } else {
                        SyncImdbResult {
                            typ: SyncImdbResultType::Success,
                            row: Some(row),
                        }
                    }
                }
                None => {
                    let error_response_json: Option<OmdbErrorResponseJson> =
                        serde_json::from_str(&text).unwrap_or(None);
                    match error_response_json {
                        Some(json) => {
                            if reached_rate_limit(&json) {
                                SyncImdbResult {
                                    typ: SyncImdbResultType::RateLimitExceeded,
                                    row: None,
                                }
                            } else if not_found(&json) {
                                SyncImdbResult {
                                    typ: SyncImdbResultType::NotFound,
                                    row: None,
                                }
                            } else {
                                eprintln!("Error: {}", json.Error);
                                SyncImdbResult {
                                    typ: SyncImdbResultType::UnknownError,
                                    row: None,
                                }
                            }
                        }
                        None => {
                            eprintln!("Response: {}", text);
                            SyncImdbResult {
                                typ: SyncImdbResultType::UnknownError,
                                row: None,
                            }
                        }
                    }
                }
            }
        }
        // Didn't get a response at all
        Err(err) => {
            eprintln!("No response {}", err);
            SyncImdbResult {
                typ: SyncImdbResultType::NoResponse,
                row: None,
            }
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let con = Connection::open("resources/data/nf_rated.sqlite")?;
    let unsynceds = get_unsynced_rows(&con)?;
    let api_key = get_api_key();
    let nunsynced = unsynceds.len();
    let amount_to_sync = RATE_LIMIT.min(nunsynced);
    eprintln!(
        "Found {} unsynced records, syncing {}",
        nunsynced, amount_to_sync
    );
    let first_sync = true;
    let mut exceeded_limit = false;
    for i in 0..amount_to_sync {
        let rated_row = unsynceds.get(i).unwrap();
        eprint!("Syncing '{}'", rated_row.title);
        match sync_imdb_title(&api_key, &rated_row.title) {
            SyncImdbResult {
                typ: SyncImdbResultType::Success,
                row,
            } => {
                assert!(row.is_some(), "row should be set for successful sync");
                let json_row = row.unwrap();
                let synced_rated_row: RatedRow =
                    (rated_row.clone(), json_row, secs_since_creation()).into();
                sync_row(&con, &synced_rated_row)?;
                eprintln!(" ✓");
            }
            SyncImdbResult {
                typ: SyncImdbResultType::RateLimitExceeded,
                ..
            } => {
                eprintln!("\nExceeded rate limit!!");
                exceeded_limit = true;
                break;
            }
            SyncImdbResult {
                typ: SyncImdbResultType::UnknownError,
                ..
            } => {
                eprintln!(
                    "\nEncountered unknown error when syncing title '{}'",
                    rated_row.title
                );
            }
            SyncImdbResult {
                typ: SyncImdbResultType::NotFound,
                ..
            } => {
                eprintln!("\nCould not find title '{}'", rated_row.title);
                if first_sync {
                    delete_row(&con, rated_row.id)?;
                }
            }
            SyncImdbResult {
                typ: SyncImdbResultType::NoResponse,
                ..
            } => {
                eprintln!(
                    "\nFailed to get response when syncing title '{}'",
                    rated_row.title
                );
            }
            SyncImdbResult {
                typ: SyncImdbResultType::MissingImdbData,
                ..
            } => {
                eprintln!("\nResponse for '{}' is missing IMDB data", rated_row.title);
                if first_sync {
                    delete_row(&con, rated_row.id)?;
                }
            }
        }
    }

    // TODO: if after all rows synced at least once we still have requests remaining
    // 1. sort rows by sync date
    // 2. sync rows until no more requests remaining

    Ok(())
}
