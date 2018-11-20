pub const QUERIES: &'static [&'static str] = &["
    CREATE TABLE \"user\" (
        id          SERIAL PRIMARY KEY,
        email       VARCHAR,
        password    VARCHAR
    )
"];

pub fn get_code_version() -> usize {
    QUERIES.len()
}
