use nf_rated::{core::JsonRow, core::OmdbJson, db::get_unsynced_rows};
use reqwest::blocking::get;
use rusqlite::Connection;
use std::{collections::HashMap, env, error::Error};

fn get_api_key() -> String {
    env::var("OMDB_KEY").expect(
        "Please add an OMDB (http://www.omdbapi.com/) API key as 'OMDB_KEY' to your environment",
    )
}

fn request_imdb_data(api_key: &str, title: &str) -> Result<(), Box<dyn Error>> {
    let uri = format!("http://www.omdbapi.com/?apikey={}&t={}", api_key, title);
    let res: OmdbJson = get(&uri)?.json()?;
    println!("{:#?}", res);
    let json_row: JsonRow = res.into();
    println!("{:#?}", json_row);
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    // let con = Connection::open("resources/data/nf_rated.sqlite")?;
    // let unsynceds = get_unsynced_rows(&con)?;

    let api_key = get_api_key();
    request_imdb_data(&api_key, "batman")?;
    // TODO: request omdb info given movie title (figure out crate to use)
    // TODO: if no value was returned remove movie from db
    // TODO: if value was returned update row
    // TODO: when we reach rate limit of 1000 stop

    // TODO: if after all rows synced at least once we still have requests remaining
    // 1. sort rows by sync date
    // 2. sync rows until no more requests remaining

    Ok(())
}
