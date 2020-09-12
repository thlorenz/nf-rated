use rusqlite::{params, Connection, Result, NO_PARAMS};

fn main() -> Result<()> {
    let con = Connection::open("resources/data/nf_rated.sqlite")?;

    // CSV data
    // - id             show_id
    // - title          title
    // - year           release_year
    // - genre
    // - cast           cast
    // - country        country
    // - director       director

    // Combination (JSON if available otherwise CSV)
    // - type           (Type, type)
    // - duration       (Runtime, duration)
    // - plot           (Plot, description)

    // JSON
    // - genre          Genre
    // - languate       Language
    // - writer         Writer
    // - imdb_rating    imdbRating (multiplied by 10 -> 0..100)
    // - imdb_votes     imdbVotes
    // - imdb_id        imdbID -> URL https://www.imdb.com/title/<imdb_id>
    con.execute(
        // CSV data
        "CREATE TABLE IF NOT EXISTS nf_imdb (
           id               INTEGER PRIMARY KEY,
           type             TEXT NOT NULL,
           title            TEXT NOT NULL,
           year             INTEGER,
           genre            TEXT,
           director         TEXT,
           writer           TEXT,
           cast             TEXT,
           country          TEXT,
           duration         TEXT,
           language         TEXT
           plot             TEXT,

           imdb_rating      INTEGER NOT NULL,
           imdb_votes       INTEGER NOT NULL,
           imdb_id          TEXT NOT NULL 
        )",
        NO_PARAMS,
    )?;

    let x: Option<u32> = None;
    con.execute(
        "INSERT INTO nf_imdb (id, type) VALUES (?1, ?2)",
        params![6, x],
    )?;

    Ok(())
}
