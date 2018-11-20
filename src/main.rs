#![feature(proc_macro_hygiene, decl_macro)]

mod db;
mod routes;

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
    let manager = db::manager("tresor", "localhost", "5432", "tresor", "tresor").unwrap();
    db::migration::run_migration(&manager).unwrap();
    rocket::ignite().mount("/", routes![index, routes::users::create_user]).manage(manager)
        .launch();
}
