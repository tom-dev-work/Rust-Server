use serde::{Deserialize, Serialize};
#[derive(Deserialize, Serialize)]
pub struct NewUser {
    // required fields for a new user.
    pub name: String,
    pub admin: bool,
    pub password: String
}
#[derive(Deserialize, Serialize)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub admin: bool
}
#[derive(Deserialize, Serialize)]
pub struct CurrentUser {
    pub admin: bool,
    pub id: i32,
    pub name: String
}