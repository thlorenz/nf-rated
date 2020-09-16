const QUERY_HEAD: &str = "SELECT * FROM nf_imdb WHERE";
const QUERY_TAIL: &str = "AND last_sync IS NOT NULL ORDER BY imdb_rating DESC;";

pub const GENRE_COLUMN: &str = "genre";
pub const TITLE_COLUMN: &str = "title";
pub const CAST_COLUMN: &str = "cast";
pub const COUNTRY_COLUMN: &str = "country";
pub const DIRECTOR_COLUMN: &str = "director";
pub const PLOT_COLUMN: &str = "plot";

pub fn build_sorted_query(column: &str, query: &str) -> String {
    let terms = query.split_ascii_whitespace();
    let mut first = true;
    let filters: Vec<String> = terms
        .filter_map(|term| match term.chars().nth(0) {
            Some(c) => {
                let and = if first { "" } else { " AND" };
                first = false;
                let (not, q) = if c == '!' {
                    (" NOT", term.split_at(1).1)
                } else {
                    ("", term)
                };
                Some(format!("\n {}{} {} LIKE %{}%", and, not, column, q))
            }
            None => None,
        })
        .collect();
    let query_filter = filters.join("");

    format!("{}{}\n  {}", QUERY_HEAD, query_filter, QUERY_TAIL)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn query_genre_sci_not_adventure_drama() {
        assert_eq!(
            build_sorted_query(&GENRE_COLUMN, "sci !adventure drama"),
            "SELECT * FROM nf_imdb WHERE
  genre LIKE %sci%
  AND NOT genre LIKE %adventure%
  AND genre LIKE %drama%
  AND last_sync IS NOT NULL ORDER BY imdb_rating DESC;"
        )
    }

    #[test]
    fn query_title_ship() {
        assert_eq!(
            build_sorted_query(&TITLE_COLUMN, "ship"),
            "SELECT * FROM nf_imdb WHERE
  title LIKE %ship%
  AND last_sync IS NOT NULL ORDER BY imdb_rating DESC;"
        )
    }

    #[test]
    fn query_country_not_india() {
        assert_eq!(
            build_sorted_query(&COUNTRY_COLUMN, "!india"),
            "SELECT * FROM nf_imdb WHERE
  NOT country LIKE %india%
  AND last_sync IS NOT NULL ORDER BY imdb_rating DESC;"
        )
    }
}
