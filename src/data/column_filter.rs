use super::CAST_COLUMN;

pub struct ColumnFilter {
    column: String,
    query: String,
}

impl From<(&str, &str)> for ColumnFilter {
    fn from(tp: (&str, &str)) -> Self {
        Self {
            column: tp.0.to_string(),
            query: tp.1.to_string(),
        }
    }
}

impl From<(&str, &String)> for ColumnFilter {
    fn from(tp: (&str, &String)) -> Self {
        Self {
            column: tp.0.to_string(),
            query: tp.1.to_string(),
        }
    }
}

impl ColumnFilter {
    pub fn sql_fragments(&self, matched_before: bool) -> Vec<String> {
        let column = if self.column == CAST_COLUMN {
            "'cast'"
        } else {
            &self.column
        };
        let mut first = !matched_before;
        let terms = self.query.split_ascii_whitespace();
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
        filters
    }
}

