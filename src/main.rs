#![feature(proc_macro_hygiene, decl_macro)]

mod db;

#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

fn main() {
    let _conn = db::initiate("tresor", "localhost", "5432", "tresor", "tresor");
    // rocket::ignite().mount("/", routes![index]).launch();
}
