use std::time::SystemTime;
const CREATE_SECS: u64 = 1599939357;

pub fn secs_since_creation() -> u32 {
    let since_epoch = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs();

    // This hack will work for about 136 years
    (since_epoch - CREATE_SECS) as u32
}

pub enum QueryTerm {
    And(String),
    Not(String),
}
pub fn destructure_query_filter(q: &str) -> Vec<QueryTerm> {
    let terms = q.split_ascii_whitespace();
    terms
        .filter_map(|term| match term.chars().nth(0) {
            Some(c) if c == '!' => {
                let q = term.split_at(1).1;
                if q.is_empty() {
                    None
                } else {
                    Some(QueryTerm::Not(q.to_string()))
                }
            }
            Some(_) => Some(QueryTerm::And(term.to_string())),
            None => None,
        })
        .collect()
}
