extern crate postgres;

use postgres::transaction::Transaction;
use postgres::{Connection, Error, TlsMode};
use r2d2::Pool;
use r2d2_postgres::PostgresConnectionManager;

pub mod migration;
pub mod manager;

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
    Connection::connect(connection, TlsMode::None)
}

pub trait Trx {
    fn run_transaction<T>(&self, job: fn(&Transaction) -> Result<T, Error>) -> Result<T, Error>;
}

impl Trx for Pool<PostgresConnectionManager> {
    fn run_transaction<T>(&self, job: fn(&Transaction) -> Result<T, Error>) -> Result<T, Error> {
        let conn = self.clone().get().unwrap();
        let trx = conn.transaction().unwrap();
        let result = job(&trx)?;
        trx.commit()?;
        Ok(result)
    }
}
