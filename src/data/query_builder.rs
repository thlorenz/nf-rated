use super::ColumnFilter;

const QUERY_HEAD: &str = "SELECT * FROM nf_imdb WHERE";
const QUERY_TAIL: &str = "last_sync IS NOT NULL ORDER BY imdb_rating DESC;";

pub const GENRE_COLUMN: &str = "genre";
pub const TITLE_COLUMN: &str = "title";
pub const CAST_COLUMN: &str = "cast";
pub const LANGUAGE_COLUMN: &str = "language";
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

pub fn build_sorted_query(item_type: &ItemType) -> String {
    let item_filter = get_item_filter(item_type).replace(" AND ", "");
    let and = match item_type {
        ItemType::Movie | ItemType::Series => "AND ",
        ItemType::Both => "",
    };
    format!("{}{}\n {}{}", QUERY_HEAD, item_filter, and, QUERY_TAIL)
}

pub fn build_sorted_filtered_query(filters: Vec<ColumnFilter>, item_type: &ItemType) -> String {
    let mut matched_before = false;
    let resolved_filters: Vec<String> = filters
        .iter()
        .flat_map(|filter| {
            let matches = filter.sql_fragments(matched_before);
            matched_before = matched_before || matches.len() > 0;
            matches
        })
        .collect();

    let query_filter = resolved_filters.join("");
    let item_filter = get_item_filter(item_type);

    format!(
        "{}{}{}\n  AND {}",
        QUERY_HEAD, query_filter, item_filter, QUERY_TAIL
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn query_genre_sci_not_adventure_drama() {
        assert_eq!(
            build_sorted_filtered_query(
                vec![(GENRE_COLUMN, "sci !adventure drama").into()],
                &ItemType::Both
            ),
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
            build_sorted_filtered_query(vec![(TITLE_COLUMN, "ship").into()], &ItemType::Both),
            "SELECT * FROM nf_imdb WHERE
  title LIKE '%ship%'
  AND last_sync IS NOT NULL ORDER BY imdb_rating DESC;"
        )
    }

    #[test]
    fn query_country_not_india() {
        assert_eq!(
            build_sorted_filtered_query(vec![(COUNTRY_COLUMN, "!india").into()], &ItemType::Both),
            "SELECT * FROM nf_imdb WHERE
  NOT country LIKE '%india%'
  AND last_sync IS NOT NULL ORDER BY imdb_rating DESC;"
        )
    }

    #[test]
    fn query_title_ship_movies_only() {
        assert_eq!(
            build_sorted_filtered_query(vec![(TITLE_COLUMN, "ship").into()], &ItemType::Movie),
            "SELECT * FROM nf_imdb WHERE
  title LIKE '%ship%'
  AND type = 'movie'
  AND last_sync IS NOT NULL ORDER BY imdb_rating DESC;"
        )
    }

    #[test]
    fn query_title_ship_series_only() {
        assert_eq!(
            build_sorted_filtered_query(vec![(TITLE_COLUMN, "ship").into()], &ItemType::Series),
            "SELECT * FROM nf_imdb WHERE
  title LIKE '%ship%'
  AND type = 'series'
  AND last_sync IS NOT NULL ORDER BY imdb_rating DESC;"
        )
    }

    #[test]
    fn query_title_ship_genre_sci() {
        assert_eq!(
            build_sorted_filtered_query(
                vec![(TITLE_COLUMN, "ship").into(), (GENRE_COLUMN, "sci").into()],
                &ItemType::Both
            ),
            "SELECT * FROM nf_imdb WHERE
  title LIKE '%ship%'
  AND genre LIKE '%sci%'
  AND last_sync IS NOT NULL ORDER BY imdb_rating DESC;"
        )
    }

    #[test]
    fn query_title_ship_genre_sci_cast_not_badactor() {
        assert_eq!(
            build_sorted_filtered_query(
                vec![
                    (TITLE_COLUMN, "ship").into(),
                    (GENRE_COLUMN, "sci").into(),
                    (CAST_COLUMN, "!badactor").into(),
                ],
                &ItemType::Both
            ),
            "SELECT * FROM nf_imdb WHERE
  title LIKE '%ship%'
  AND genre LIKE '%sci%'
  AND NOT 'cast' LIKE '%badactor%'
  AND last_sync IS NOT NULL ORDER BY imdb_rating DESC;"
        )
    }
}
