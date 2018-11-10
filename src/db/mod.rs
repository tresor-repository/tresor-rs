extern crate postgres;
use postgres::{Connection, Error, TlsMode};

pub mod migration;

pub fn initiate(
    database: &str,
    host: &str,
    port: &str,
    user: &str,
    pass: &str,
) -> Result<Connection, Error> {
    let connection = format!(
        "postgres://{}:{}@{}:{}/{}",
        user, pass, host, port, database
    );
    println!("{}", connection);
    Connection::connect(connection, TlsMode::None)
}
