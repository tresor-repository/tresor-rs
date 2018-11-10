use crate::db::Trx;
use postgres::transaction::Transaction;
use postgres::{Connection, Error};
pub fn run_migration(conn: &Connection) -> Result<(), Error> {
    conn.run_transaction(get_current_db_version)
}

fn get_current_db_version<'t>(trx: &'t Transaction) -> Result<(), Error> {
    trx.execute(
        "
        CREATE TABLE IF NOT EXISTS meta (
			key VARCHAR(50) PRIMARY KEY,
			value integer NOT NULL DEFAULT 0
		);",
        &[],
    )?;
    trx.execute(
        "
		INSERT INTO meta (key, value)
		VALUES ('db-version', 0)
		ON CONFLICT DO NOTHING
    ",
        &[],
    )?;
    Ok(())
}
