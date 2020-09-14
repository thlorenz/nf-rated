use std::{error::Error, fs::File};

use nf_rated::data::{CsvRow, Db};

fn main() -> Result<(), Box<dyn Error>> {
    let db = Db::new()?;
    db.create_table()?;

    let file_path = "resources/data/netflix_titles.csv";
    let file = File::open(file_path)?;
    let mut rdr = csv::Reader::from_reader(file);
    for result in rdr.records() {
        let row: CsvRow = result?.into();
        db.upsert_row(&row.into())?;
    }

    Ok(())
}
