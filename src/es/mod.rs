extern crate rs_es;
use rs_es::Client;

pub fn getSpendings(userId: &str, limit: i32, offset: i32) -> str {
    let mut client = Client::new("http://localhost:9200");
    client.get("spending", userId).send()
}