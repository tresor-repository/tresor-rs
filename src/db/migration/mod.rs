use crate::db::Trx;
use postgres::transaction::Transaction;
use postgres::{Connection, Error};
pub fn run_migration(conn: &Connection) -> Result<(), Error> {
    conn.run_transaction(|trx| {
        let version = get_current_db_version(trx)?;
        Ok(())
    })
}

fn get_current_db_version<'t>(trx: &'t Transaction) -> Result<i32, Error> {
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
    let value: i32 = trx
        .query(
            "
            SELECT value FROM meta WHERE key = 'db-version'
            ",
            &[],
        )?
        .get(0)
        .get(0);
    Ok(value)
}
