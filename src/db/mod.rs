extern crate postgres;
use postgres::transaction::Transaction;
use postgres::{Connection, Error, TlsMode};

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

impl Trx for Connection {
    fn run_transaction<T>(&self, job: fn(&Transaction) -> Result<T, Error>) -> Result<T, Error> {
        let trx = self.transaction()?;
        let result = job(&trx)?;
        trx.commit()?;
        Ok(result)
    }
}
