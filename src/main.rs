#![feature(proc_macro_hygiene, decl_macro)]

mod db;

#[macro_use]
extern crate rocket;
extern crate r2d2;
extern crate r2d2_postgres;
extern crate postgres;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

fn main() {
    let conn = db::initiate("tresor", "localhost", "5432", "tresor", "tresor").unwrap();
    db::migration::run_migration(&conn).unwrap();

    let manager = db::manager::manager("tresor", "localhost", "5432", "tresor", "tresor").unwrap();
    let pool = r2d2::Pool::new(manager).expect("db pool");

    rocket::ignite().mount("/", routes![index]).manage(pool)
        .launch();
}
