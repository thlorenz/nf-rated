const QUERY_HEAD: &str = "SELECT * FROM nf_imdb WHERE";
const QUERY_TAIL: &str = "AND last_sync IS NOT NULL ORDER BY imdb_rating DESC;";

pub const GENRE_COLUMN: &str = "genre";
pub const TITLE_COLUMN: &str = "title";
pub const CAST_COLUMN: &str = "cast";
pub const COUNTRY_COLUMN: &str = "country";
pub const DIRECTOR_COLUMN: &str = "director";
pub const PLOT_COLUMN: &str = "plot";

pub enum ItemType {
    Movie,
    Series,
    Both,
}
const MOVIE_ITEM_FILTER: &str = "\n  AND type = 'movie'";
const SHOW_ITEM_FILTER: &str = "\n  AND type = 'series'";
const BOTH_ITEM_FILTER: &str = "";

fn get_item_filter(item_type: &ItemType) -> String {
    match item_type {
        ItemType::Movie => MOVIE_ITEM_FILTER.to_string(),
        ItemType::Series => SHOW_ITEM_FILTER.to_string(),
        ItemType::Both => BOTH_ITEM_FILTER.to_string(),
    }
}

pub fn build_sorted_query(column: &str, query: &str, item_type: &ItemType) -> String {
    let terms = query.split_ascii_whitespace();
    let mut first = true;
    let filters: Vec<String> = terms
        .filter_map(|term| match term.chars().nth(0) {
            Some(c) if c == '!' => {
                let and = if first { "" } else { " AND" };
                first = false;
                let q = term.split_at(1).1;
                if q.is_empty() {
                    None
                } else {
                    Some(format!("\n {} NOT {} LIKE '%{}%'", and, column, q))
                }
            }
            Some(_) => {
                let and = if first { "" } else { " AND" };
                first = false;
                Some(format!("\n {} {} LIKE '%{}%'", and, column, term))
            }
            None => None,
        })
        .collect();
    let query_filter = filters.join("");
    let item_filter = get_item_filter(item_type);

    format!(
        "{}{}{}\n  {}",
        QUERY_HEAD, query_filter, item_filter, QUERY_TAIL
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn query_genre_sci_not_adventure_drama() {
        assert_eq!(
            build_sorted_query(&GENRE_COLUMN, "sci !adventure drama", &ItemType::Both),
            "SELECT * FROM nf_imdb WHERE
  genre LIKE '%sci%'
  AND NOT genre LIKE '%adventure%'
  AND genre LIKE '%drama%'
  AND last_sync IS NOT NULL ORDER BY imdb_rating DESC;"
        )
    }

    #[test]
    fn query_title_ship() {
        assert_eq!(
            build_sorted_query(&TITLE_COLUMN, "ship", &ItemType::Both),
            "SELECT * FROM nf_imdb WHERE
  title LIKE '%ship%'
  AND last_sync IS NOT NULL ORDER BY imdb_rating DESC;"
        )
    }

    #[test]
    fn query_country_not_india() {
        assert_eq!(
            build_sorted_query(&COUNTRY_COLUMN, "!india", &ItemType::Both),
            "SELECT * FROM nf_imdb WHERE
  NOT country LIKE '%india%'
  AND last_sync IS NOT NULL ORDER BY imdb_rating DESC;"
        )
    }

    #[test]
    fn query_title_ship_movies_only() {
        assert_eq!(
            build_sorted_query(&TITLE_COLUMN, "ship", &ItemType::Movie),
            "SELECT * FROM nf_imdb WHERE
  title LIKE '%ship%'
  AND type = 'movie'
  AND last_sync IS NOT NULL ORDER BY imdb_rating DESC;"
        )
    }

    #[test]
    fn query_title_ship_series_only() {
        assert_eq!(
            build_sorted_query(&TITLE_COLUMN, "ship", &ItemType::Series),
            "SELECT * FROM nf_imdb WHERE
  title LIKE '%ship%'
  AND type = 'series'
  AND last_sync IS NOT NULL ORDER BY imdb_rating DESC;"
        )
    }
}
