use std::time::SystemTime;

use csv::StringRecord;
use rusqlite::Row;
use serde::Deserialize;

const N_A: &str = "N/A";

#[derive(Debug)]
pub struct CsvRow {
    id: u32,
    title: String,
    year: u32,
    cast: String,
    country: String,
    director: String,
    typ: String,
    duration: String,
    plot: String,
}

#[derive(Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct OmdbJson {
    Type: String,
    Runtime: String,
    Plot: String,

    Genre: String,
    Language: String,
    Writer: String,

    imdbRating: String,
    imdbVotes: String,
    imdbID: String,
}

fn json_string(s: &str) -> Option<String> {
    match s {
        N_A => None,
        _ => Some(s.to_string()),
    }
}

#[derive(Deserialize, Clone, Debug)]
pub struct JsonRow {
    typ: Option<String>,
    duration: Option<String>,
    plot: Option<String>,

    genre: String,
    language: String,
    writer: String,

    imdb_rating: Option<u32>,
    imdb_votes: Option<u32>,
    imdb_id: Option<String>,
}

impl JsonRow {
    pub fn is_missing_imdb_data(&self) -> bool {
        self.imdb_id.is_none() || self.imdb_rating.is_none()
    }
}

impl From<OmdbJson> for JsonRow {
    fn from(json: OmdbJson) -> Self {
        let imdb_rating = match maybe_float(&json.imdbRating) {
            Some(rating) => Some((rating * 10.0).round() as u32),
            None => None,
        };

        let imdb_votes = maybe_uint(&json.imdbVotes.replace(",", ""));
        let imdb_id = json_string(&json.imdbID);

        Self {
            typ: json_string(&json.Type),
            duration: json_string(&json.Runtime),
            plot: json_string(&json.Plot),

            genre: json.Genre,
            language: json.Language,
            writer: json.Writer,

            imdb_rating,
            imdb_votes,
            imdb_id,
        }
    }
}

#[derive(Debug, Clone)]
pub struct RatedRow {
    // Csv
    pub id: u32,
    pub title: String,
    pub year: u32,
    pub cast: String,
    pub country: String,
    pub director: String,

    // Csv perferring JSON
    pub typ: String,
    pub duration: String,
    pub plot: String,

    // JSON
    pub genre: Option<String>,
    pub language: Option<String>,
    pub writer: Option<String>,

    pub imdb_rating: Option<u32>,
    pub imdb_votes: Option<u32>,
    pub imdb_id: Option<String>,

    // millis since UNIX_EPOCH
    pub last_sync: Option<u32>,
}

const CREATE_SECS: u64 = 1599939357;

pub fn secs_since_creation() -> u32 {
    let since_epoch = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs();

    // This hack will work for about 136 years
    (since_epoch - CREATE_SECS) as u32
}

fn maybe_uint(s: &str) -> Option<u32> {
    match s {
        N_A => None,
        s => s.parse::<u32>().ok(),
    }
}

fn maybe_float(s: &str) -> Option<f32> {
    match s {
        N_A => None,
        s => s.parse::<f32>().ok(),
    }
}

// 0: show_id, 1: type, 2: title, 3: director, 4: cast, 5: country, 6: date_added,
// 7: release_year, 8: rating, 9: duration, 10: listed_in, 11: description
impl From<StringRecord> for CsvRow {
    fn from(x: StringRecord) -> Self {
        Self {
            id: maybe_uint(&x.get(0).unwrap()).unwrap(),
            title: x.get(2).unwrap().to_string(),
            year: maybe_uint(&x.get(7).unwrap()).unwrap(),
            cast: x.get(4).unwrap().to_string(),
            country: x.get(5).unwrap().to_string(),
            director: x.get(3).unwrap().to_string(),
            typ: x.get(1).unwrap().to_string(),
            duration: x.get(9).unwrap().to_string(),
            plot: x.get(11).unwrap().to_string(),
        }
    }
}

impl From<CsvRow> for RatedRow {
    fn from(csv: CsvRow) -> Self {
        RatedRow {
            id: csv.id,
            title: csv.title,
            year: csv.year,
            cast: csv.cast,
            country: csv.country,
            director: csv.director,
            typ: csv.typ,
            duration: csv.duration,
            plot: csv.plot,

            genre: None,
            language: None,
            writer: None,

            imdb_rating: None,
            imdb_votes: None,
            imdb_id: None,

            last_sync: None,
        }
    }
}

pub fn rated_row_from_row(row: &Row) -> RatedRow {
    RatedRow {
        id: row.get(0).unwrap(),
        title: row.get(1).unwrap(),
        year: row.get(2).unwrap(),
        cast: row.get(3).unwrap(),
        country: row.get(4).unwrap(),
        director: row.get(5).unwrap(),
        typ: row.get(6).unwrap(),
        duration: row.get(7).unwrap(),
        plot: row.get(8).unwrap(),
        genre: row.get(9).unwrap(),
        writer: row.get(10).unwrap(),
        language: row.get(11).unwrap(),
        imdb_rating: row.get(12).unwrap(),
        imdb_votes: row.get(13).unwrap(),
        imdb_id: row.get(14).unwrap(),
        last_sync: row.get(15).unwrap(),
    }
}

impl From<(RatedRow, JsonRow, u32)> for RatedRow {
    fn from((rated, json, last_sync): (RatedRow, JsonRow, u32)) -> Self {
        let typ = json.typ.unwrap_or(rated.typ);
        let duration = json.duration.unwrap_or(rated.duration);
        let plot = json.plot.unwrap_or(rated.plot);

        Self {
            id: rated.id,
            title: rated.title,
            year: rated.year,
            cast: rated.cast,
            country: rated.country,
            director: rated.director,
            typ,
            duration,
            plot,
            genre: Some(json.genre),
            writer: Some(json.writer),
            language: Some(json.language),
            imdb_rating: json.imdb_rating,
            imdb_votes: json.imdb_votes,
            imdb_id: json.imdb_id,
            last_sync: Some(last_sync),
        }
    }
}
