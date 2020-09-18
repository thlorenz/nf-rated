use super::{destructure_query_filter, QueryTerm, CAST_COLUMN};

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

// TODO: figure out how to not repeat ourselves here
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
        let terms = destructure_query_filter(&self.query);
        let filters: Vec<String> = terms
            .iter()
            .map(|term| match term {
                QueryTerm::Not(term) => {
                    let and = if first { "" } else { " AND" };
                    first = false;
                    format!("\n {} NOT {} LIKE '%{}%'", and, column, term)
                }
                QueryTerm::And(term) => {
                    let and = if first { "" } else { " AND" };
                    first = false;
                    format!("\n {} {} LIKE '%{}%'", and, column, term)
                }
            })
            .collect();
        filters
    }
}
