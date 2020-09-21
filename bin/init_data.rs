use std::error::Error;

use nf_rated::data::{CsvRow, Db};

fn main() -> Result<(), Box<dyn Error>> {
    let db = Db::new()?;
    db.create_table()?;

    let csv = include_str!("../resources/data/netflix_titles.csv");
    let mut rdr = csv::Reader::from_reader(csv.as_bytes());
    for result in rdr.records() {
        let row: CsvRow = result?.into();
        db.upsert_row(&row.into())?;
    }

    Ok(())
}
