use super::super::db;

#[get("/users")]
pub fn create_user(conn: db::Conn) -> &'static str {
    "heeeloooo"
}