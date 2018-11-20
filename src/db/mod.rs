extern crate postgres;

use postgres::transaction::Transaction;
use r2d2::Pool;
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
) -> Result<Conn, Error> {
    let connection = format!(
        "postgres://{}:{}@{}:{}/{}",
        user, pass, host, port, database
    );
    let manager = PostgresConnectionManager::new(
        connection, TlsMode::None,
    ).map_err(map_posgres_error)?;
    r2d2::Pool::new(manager).map_err(map_r2d2_error)
}

pub type Conn = Pool<PostgresConnectionManager>;

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

pub fn map_r2d2_error(err: r2d2::Error) -> Error {
    Error { error: io::Error::new(io::ErrorKind::Other, "r2d2 error") }
}

pub fn map_posgres_error(err: postgres::Error) -> Error {
    Error { error: io::Error::from(err) }
}

pub trait Trx {
    fn run_transaction<T>(&self, job: fn(&Transaction) -> Result<T, postgres::Error>) -> Result<T, Error>;
}

impl Trx for Pool<PostgresConnectionManager> {
    fn run_transaction<T>(&self, job: fn(&Transaction) -> Result<T, postgres::Error>) -> Result<T, Error> {
        let conn = self.clone().get().map_err(map_r2d2_error)?;
        let trx = conn.transaction().map_err(map_posgres_error)?;
        let result = job(&trx).map_err(map_posgres_error)?;
        trx.commit();
        Ok(result)
    }
}
