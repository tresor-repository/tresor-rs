use r2d2_postgres::{TlsMode, PostgresConnectionManager};
use r2d2_postgres::postgres::Error;

pub fn manager(
    database: &str,
    host: &str,
    port: &str,
    user: &str,
    pass: &str,
) -> Result<PostgresConnectionManager, Error> {
    let connection = format!(
        "postgres://{}:{}@{}:{}/{}",
        user, pass, host, port, database
    );
    PostgresConnectionManager::new(connection, TlsMode::None)
}