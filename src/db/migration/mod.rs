use postgres::{Connection, Error};
pub fn run_migration(conn: &Connection) -> Result<(), Error> {
    println!("hello here");
    Ok(())
}
