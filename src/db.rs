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
    // - language       Language
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

const UPSERT_QUERY: &str = "INSERT INTO nf_imdb (
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

const SELECT_UNSYNCED_QUERY: &str = "SELECT * FROM nf_imdb WHERE last_sync IS NULL;";
pub fn get_unsynced_rows(con: &Connection) -> Result<Vec<RatedRow>, Error> {
    let mut stmt = con.prepare(SELECT_UNSYNCED_QUERY)?;
    let iter = stmt.query_map(NO_PARAMS, |row| Ok(rated_row_from_row(&row)))?;
    iter.collect()
}

const SYNC_QUERY: &str = "UPDATE nf_imdb
    SET
        id         = ?1,
        title      = ?2 ,
        year       = ?3 ,
        cast       = ?4 ,
        country    = ?5 ,
        director   = ?6 ,
        type       = ?7 ,
        duration   = ?8 ,
        plot       = ?9 ,

        genre      = ?10 , 
        writer     = ?11 ,
        language   = ?12 ,

        imdb_rating= ?13 ,
        imdb_votes = ?14 ,
        imdb_id    = ?15 ,

        last_sync  = ?16
    WHERE
        id = ?1;
";

pub fn sync_row(con: &Connection, row: &RatedRow) -> Result<usize> {
    con.execute(
        SYNC_QUERY,
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

const DELETE_ROW_QUERY: &str = "DELETE FROM nf_imdb WHERE id = ?1;";
pub fn delete_row(con: &Connection, id: u32) -> Result<usize> {
    con.execute(DELETE_ROW_QUERY, params![id])
}
