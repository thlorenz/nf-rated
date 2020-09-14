#[derive(Debug, Clone)]
pub struct RatedRow {
    // Csv
    pub id: u32,
    pub title: String,
    pub year: u32,
    pub cast: String,
    pub country: String,
    pub director: String,

    // Csv preferring JSON
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
