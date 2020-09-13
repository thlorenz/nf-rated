use nf_rated::{
    core::secs_since_creation, core::JsonRow, core::OmdbJson, core::RatedRow, db::delete_row,
    db::get_unsynced_rows, db::sync_row,
};
use reqwest::blocking::get;
use rusqlite::Connection;
use std::{env, error::Error};

const RATE_LIMIT: usize = 5;

fn get_api_key() -> String {
    env::var("OMDB_KEY").expect(
        "Please add an OMDB (http://www.omdbapi.com/) API key as 'OMDB_KEY' to your environment",
    )
}

fn request_imdb_data_for_title(api_key: &str, title: &str) -> Result<JsonRow, Box<dyn Error>> {
    let uri = format!("http://www.omdbapi.com/?apikey={}&t={}", api_key, title);
    let res: OmdbJson = get(&uri)?.json()?;
    Ok(res.into())
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
    for i in 0..amount_to_sync {
        let rated_row = unsynceds.get(i).unwrap();
        eprint!("Syncing '{}'", rated_row.title);

        match request_imdb_data_for_title(&api_key, &rated_row.title) {
            Ok(json_row) => {
                if json_row.is_missing_imdb_data() {
                    eprintln!(
                        "\nMissing imdb data for '{}', purging row.",
                        rated_row.title
                    );
                    delete_row(&con, rated_row.id)?;
                } else {
                    let now = secs_since_creation();
                    let synced_row: RatedRow = (rated_row.clone(), json_row, now).into();
                    sync_row(&con, &synced_row)?;
                    eprintln!(" ✓");
                }
            }
            Err(_) => {
                eprintln!("\nFailed to sync '{}', purging row.", rated_row.title);
                delete_row(&con, rated_row.id)?;
            }
        }
    }

    // TODO: if after all rows synced at least once we still have requests remaining
    // 1. sort rows by sync date
    // 2. sync rows until no more requests remaining

    Ok(())
}
