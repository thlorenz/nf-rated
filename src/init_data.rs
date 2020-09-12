use std::{error::Error, fs::File};

use nf_rated::{core::CsvRow, db::create_table, db::upsert_row};
use rusqlite::Connection;

fn main() -> Result<(), Box<dyn Error>> {
    let con = Connection::open("resources/data/nf_rated.sqlite")?;
    create_table(&con)?;

    let file_path = "resources/data/netflix_titles.csv";
    let file = File::open(file_path)?;
    let mut rdr = csv::Reader::from_reader(file);
    for result in rdr.records() {
        let row: CsvRow = result?.into();
        upsert_row(&con, &row.into())?;
    }

    Ok(())
}
