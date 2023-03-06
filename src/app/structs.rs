
use serde::{Deserialize, Serialize};
#[derive(Deserialize, Serialize, Clone)]
pub struct App {
    pub id: i32,
    pub name: String,
    pub token: String,
}
#[derive(Deserialize, Serialize, Clone)]
pub struct Apps {
    pub apps: Vec<App>
}
#[derive(Deserialize, Serialize, Clone)]
pub struct AppUpdate {
    pub name: String,
    pub description: String
}