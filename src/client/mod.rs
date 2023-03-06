use rusqlite::{Connection, Error, params};
use rocket::http::Status;
use uuid::Uuid;
use rocket::*;
use rocket_contrib::json::Json;

mod structs;
use structs::{Client, NewClient};


impl Client {
    pub fn new(name: &str) -> Self {
        Self {
            id: 1,
            name: name.to_string(),
            token: Uuid::new_v4().to_string()
        }
    }
}

/// create a client
#[post("/client", format = "json", data = "<client>")]
pub fn create(client: Json<NewClient>) -> Json<Client> {
    Json(Client::new(&client.name))
}