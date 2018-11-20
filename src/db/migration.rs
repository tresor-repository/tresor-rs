extern crate r2d2_postgres;
extern crate r2d2;

use super::{Conn, Trx, Error, queries};

use postgres::transaction::Transaction;


pub fn run_migration(conn: &Conn) -> Result<(), Error> {
    let conn = conn.clone();
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

fn update_version_to_lastest(trx: &Transaction, version: i32) -> Result<u64, postgres::Error> {
    trx.execute(
        "
        UPDATE meta SET value = $1 WHERE key = 'db-version'
    ",
        &[&version],
    )
}

fn get_current_db_version(trx: &Transaction) -> Result<i32, postgres::Error> {
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
