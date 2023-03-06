use serde::{Deserialize, Serialize};
#[derive(Deserialize, Serialize)]
pub struct NewClient {
    // required fields for a new Client.
    pub name: String,
}
#[derive(Deserialize, Serialize)]
pub struct Client {
    pub id: i32,
    pub token: String,
    pub name: String
}