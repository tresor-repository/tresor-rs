extern crate postgres;
use postgres::{Connection, Error, TlsMode};

pub type Conn = Connection;
pub type DbError = Error;

pub fn initiate(
    database: &str,
    host: &str,
    port: &str,
    user: &str,
    pass: &str,
) -> Result<Conn, DbError> {
    Connection::connect(
        format!(
            "postgres:://{}:{}@{}:{}/{}",
            user, pass, host, port, database
        ),
        TlsMode::None,
    )
}
