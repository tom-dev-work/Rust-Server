
use serde::{Deserialize, Serialize};
use rocket::*;
use rocket_contrib::json::Json;
#[derive(Debug, Deserialize, Serialize)]
// it's formatted like this for the app.
#[allow(non_snake_case)]
pub struct Version {
    buildDate: String,
    commit: String,
    version: String,
}
impl Version {
    fn new() -> Self {
        Self {
            buildDate: String::from(""),
            commit: String::from(""),
            version: env!("CARGO_PKG_VERSION").to_string(),
        }
    }
}
#[get("/version")]
pub fn version() -> Json<Version> {
    Json(Version::new())
}
