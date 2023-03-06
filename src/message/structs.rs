use serde::{Deserialize, Serialize};
#[derive(Debug, Deserialize, Serialize)]
pub struct NewMessage {
    // required fields for a new message.
    pub message: String,
    pub title: String,
    pub priority: u8,
    pub token: String
}
#[derive(Debug, Deserialize, Serialize)]
pub struct MessagePublic {
    pub id: i32,
    pub message: String,
    pub title: String,
    pub priority: u8,
    pub date: String
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Message {
    pub id: i32,
    pub date: String,
    pub message: String,
    pub title: String,
    pub priority: u8,
    pub token: String
}
#[derive(Debug, Deserialize, Serialize)]
pub struct Messages {
    pub messages: Vec<MessagePublic>
}
