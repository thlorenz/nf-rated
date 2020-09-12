use rusqlite::{params, Connection, Error, Result, NO_PARAMS};

use crate::core::{rated_row_from_row, RatedRow};

pub fn create_table(con: &Connection) -> Result<usize> {
    // CSV data
    // - id             show_id
    // - title          title
    // - year           release_year
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
           title            TEXT NOT NULL,
           year             INTEGER NOT NULL,
           cast             TEXT NOT NULL,
           country          TEXT NOT NULL,
           director         TEXT NOT NULL,
           type             TEXT NOT NULL,
           duration         TEXT NOT NULL,
           plot             TEXT NOT NULL,

           genre            TEXT,
           writer           TEXT,
           language         TEXT,

           imdb_rating      INTEGER,
           imdb_votes       INTEGER,
           imdb_id          TEXT,

           last_sync        INTEGER
        )",
        NO_PARAMS,
    )
}

static UPSERT_QUERY: &str = "INSERT INTO nf_imdb (
    id          ,
    title       ,
    year        ,
    cast        ,
    country     ,
    director    ,
    type        ,
    duration    ,
    plot        ,

    genre       , 
    writer      ,
    language    ,

    imdb_rating ,
    imdb_votes  ,
    imdb_id     ,

    last_sync)
VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16)
ON CONFLICT (id) DO NOTHING;
";

pub fn upsert_row(con: &Connection, row: &RatedRow) -> Result<usize> {
    con.execute(
        UPSERT_QUERY,
        params![
            row.id,
            row.title,
            row.year,
            row.cast,
            row.country,
            row.director,
            row.typ,
            row.duration,
            row.plot,
            row.genre,
            row.writer,
            row.language,
            row.imdb_rating,
            row.imdb_votes,
            row.imdb_id,
            row.last_sync
        ],
    )
}

static SELECT_UNSYNCED_QUERY: &str = "SELECT * FROM nf_imdb WHERE last_sync IS NULL;";
pub fn get_unsynced_rows(con: &Connection) -> Result<Vec<RatedRow>, Error> {
    let mut stmt = con.prepare(SELECT_UNSYNCED_QUERY)?;
    let iter = stmt.query_map(NO_PARAMS, |row| Ok(rated_row_from_row(&row)))?;
    iter.collect()
}
