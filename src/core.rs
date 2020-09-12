const N_A: &str = "N/A";

struct CsvRow {
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

struct JsonRow {
    typ: String,
    duration: String,
    plot: String,

    genre: String,
    language: String,
    writer: String,

    imdb_rating: u32,
    imdb_votes: u32,
    imdb_id: String,
}

struct Row {
    // Csv
    id: u32,
    title: String,
    year: u32,
    cast: String,
    country: String,
    director: String,

    // Csv perferring JSON
    typ: String,
    duration: String,
    plot: String,

    // JSON
    genre: String,
    language: String,
    writer: String,

    imdb_rating: u32,
    imdb_votes: u32,
    imdb_id: String,
}

fn json_string(s: &str) -> Option<&str> {
    match s {
        N_A => None,
        _ => Some(s),
    }
}

fn json_uint(s: &str) -> Option<u32> {
    match s {
        N_A => None,
        s => s.parse::<u32>().ok(),
    }
}

fn json_float(s: &str) -> Option<f32> {
    match s {
        N_A => None,
        s => s.parse::<f32>().ok(),
    }
}

impl From<(CsvRow, JsonRow)> for Row {
    fn from((csv, json): (CsvRow, JsonRow)) -> Self {

        // TODO: use https://crates.io/crates/csv to parse CSV
        // TODO: JSON Row needs Option types for everything
        // TODO: Row needs Option types for nullable values
        // Row {}
    }
}
