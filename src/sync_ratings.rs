use std::{env, error::Error};

use nf_rated::db::get_unsynced_rows;
use rusqlite::Connection;

fn get_api_key() -> String {
    env::var("OMDB_KEY").expect(
        "Please add an OMDB (http://www.omdbapi.com/) API key as 'OMDB_KEY' to your environment",
    )
}

fn main() -> Result<(), Box<dyn Error>> {
    let con = Connection::open("resources/data/nf_rated.sqlite")?;
    let _api_key = get_api_key();

    let unsynceds = get_unsynced_rows(&con)?;
    eprintln!("{:?}", unsynceds);

    // TODO: read rows from db + filter rows that don't have imdb rating yet
    // TODO: request omdb info given movie title (figure out crate to use)
    // TODO: if no value was returned remove movie from db
    // TODO: if value was returned update row
    // TODO: when we reach rate limit of 1000 stop

    // TODO: if after all rows synced at least once we still have requests remaining
    // 1. sort rows by sync date
    // 2. sync rows until no more requests remaining

    Ok(())
}
