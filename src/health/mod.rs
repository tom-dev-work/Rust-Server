use rocket::*;
use rocket_contrib::json::Json;
use rusqlite::Connection;
use serde::Serialize;

#[derive(Serialize)]
pub struct Health {
    database: String,
    health: String,
}
impl Health {
    fn new() -> Self {
        Self {
            database: db_check(),
            health: String::from("green"),
        }
    }
}
// server health
#[get("/health")]
pub fn healthcheck() -> Json<Health> {
    Json(Health::new())
}

pub fn db_check() -> String {
    match Connection::open("db_rotify.sqlite3") {
        Ok(_e) => String::from("green"),
        Err(_e) => String::from("red"),
    }
}
