extern crate postgres;

use postgres::transaction::Transaction;
use r2d2_postgres::{PostgresConnectionManager, TlsMode};
use std::io;
use std::fmt::{Display, Formatter, Debug};

pub mod migration;
pub mod queries;

pub fn manager(
    database: &str,
    host: &str,
    port: &str,
    user: &str,
    pass: &str,
) -> Result<Pool, Error> {
    let connection = format!(
        "postgres://{}:{}@{}:{}/{}",
        user, pass, host, port, database
    );
    let manager = PostgresConnectionManager::new(
        connection, TlsMode::None,
    ).map_err(map_postgres_error)?;
    r2d2::Pool::new(manager).map_err(map_r2d2_error)
}

pub type Pool = r2d2::Pool<PostgresConnectionManager>;

pub struct Conn(pub r2d2::PooledConnection<PostgresConnectionManager>);

pub struct Error {
    error: io::Error,
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "{}", &self.error)
    }
}

impl Debug for Error {
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "{}", &self.error)
    }
}

pub fn map_r2d2_error(_err: r2d2::Error) -> Error {
    Error { error: io::Error::new(io::ErrorKind::Other, "r2d2 error") }
}

pub fn map_postgres_error(err: postgres::Error) -> Error {
    Error { error: io::Error::from(err) }
}

pub trait Trx {
    fn run_transaction<T>(&self, job: fn(&Transaction) -> Result<T, postgres::Error>) -> Result<T, Error>;
}

impl Trx for r2d2::PooledConnection<r2d2_postgres::PostgresConnectionManager> {
    fn run_transaction<T>(&self, job: fn(&Transaction) -> Result<T, postgres::Error>) -> Result<T, Error> {
        let trx = self.transaction().map_err(map_postgres_error)?;
        let result = job(&trx).map_err(map_postgres_error)?;
        trx.commit().map_err(map_postgres_error)?;
        Ok(result)
    }
}
