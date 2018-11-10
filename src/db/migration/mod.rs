use crate::db::Trx;
use postgres::transaction::Transaction;
use postgres::{Connection, Error};

mod queries;

pub fn run_migration(conn: &Connection) -> Result<(), Error> {
    conn.run_transaction(|trx| {
        let version = get_current_db_version(trx)?;
        let code_version = queries::get_code_version();
        if code_version as i32 > version {
            for query in queries::QUERIES {
                trx.execute(query, &[])?;
            }
            update_version_to_lastest(trx, code_version as i32)?;
        }
        Ok(())
    })
}

fn update_version_to_lastest(trx: &Transaction, version: i32) -> Result<u64, Error> {
    trx.execute(
        "
        UPDATE meta SET value = $1 WHERE key = 'db-version'
    ",
        &[&version],
    )
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
