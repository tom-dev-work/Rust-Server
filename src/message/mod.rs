use chrono::Utc;
use rocket::http::Status;
use rocket::*;
use rocket_contrib::json::Json;
use rusqlite::{Connection, Error, Result, params};
use uuid::Uuid;

mod structs;
use structs::{Message, MessagePublic, Messages, NewMessage};
#[path = "../app/structs.rs"]
mod app;
#[path = "../websockets/mod.rs"]
mod websockets;
use app::App;

impl Message {
    pub fn new(message: String, title: String, priority: u8, token: String) -> Self {
        Self {
            id: 1,
            date: Utc::now().to_string(),
            message,
            title,
            priority,
            token,
        }
    }
    pub fn get_appid(token: &String) -> Result<i32, Error> {
        let conn = Connection::open("db_rotify.sqlite3")?;
        let mut app_id_stmt =
            conn.prepare("SELECT id, name, token FROM app where token = :token")?;
        let app_id_rows = app_id_stmt.query_map(&[(":token", &token)], |row| {
            Ok(App {
                id: row.get(0)?,
                name: row.get(1)?,
                token: row.get(2)?,
            })
        })?;
        let mut rows = Vec::new();
        for app_id_row in app_id_rows {
            rows.push(app_id_row.unwrap());
        }
        let one_row = rows.clone().into_iter().nth(0);
        match one_row {
            Some(app_id) => Ok(app_id.id),
            None => Err(Error::InvalidQuery),
        }
    }
    fn store(message: Message) -> Result<Message, Error> {
        let app_id_result = Self::get_appid(&message.token);
        let app_id = match app_id_result {
            Ok(app_id) => app_id,
            Err(_e) => 0,
        };
        if app_id > 0 {
            let conn = Connection::open("db_rotify.sqlite3")?;
            conn.execute(
                "INSERT INTO message (date, message, title, priority, appid) values (:date, :message, :title, :priority, :appid)",
                params![
                    &message.date.to_string(),
                    &message.message,
                    &message.title,
                    &message.priority,
                    &app_id
                    ]
            )?;
            match serde_json::to_string(&message) {
                Ok(msg) => websockets::send_message(msg),
                Err(_e) => (),
            }
            Ok(message)
        } else {
            Err(Error::InvalidQuery)
        }
    }
    fn delete(id: String) -> Result<()> {
        let conn = Connection::open("db_rotify.sqlite3")?;
        let mut stmt = conn.prepare(
            "SELECT id, date, message, title, priority, token FROM message WHERE id = :id",
        )?;
        let messages_to_delete = stmt.query_map(&[(":id", &id)], |row| {
            Ok(Message {
                id: row.get(0)?,
                date: row.get(1)?,
                message: row.get(2)?,
                title: row.get(3)?,
                priority: row.get(4)?,
                token: row.get(5)?,
            })
        })?;
        for delete_message in messages_to_delete {
            match delete_message {
                Ok(msg_content) => match conn.execute(
                    "DELETE from message WHERE id = :id",
                    &[(":id", &msg_content.id)],
                ) {
                    Ok(_deleted) => (),
                    Err(delete_failed) => println!("Delete failed: {}", delete_failed),
                },
                Err(_failure) => (),
            }
        }

        Ok(())
    }
    fn delete_all() -> Result<()> {
        let conn = Connection::open("db_rotify.sqlite3")?;
        conn.execute("DELETE from message", [])?;
        Ok(())
    }

    fn get() -> Result<Messages, Error> {
        let mut data: Vec<MessagePublic> = Vec::new();
        let conn = Connection::open("db_rotify.sqlite3")?;
        let mut stmt = conn.prepare("SELECT id, date, message, title, priority FROM message")?;
        let messages = stmt.query_map([], |row| {
            Ok(MessagePublic {
                id: row.get(0)?,
                date: row.get(1)?,
                message: row.get(2)?,
                title: row.get(3)?,
                priority: row.get(4)?,
            })
        });
        if let Ok(message) = messages {
            for row in message {
                match row {
                    Ok(row_result) => data.push(row_result),
                    Err(_e) => (),
                }
            }
        }
        let data_container: Messages = Messages { messages: (data) };
        Ok(data_container)
    }

    fn get_app_messages(appid: String) -> Result<Messages, Error> {
        let mut data: Vec<MessagePublic> = Vec::new();
        let conn = Connection::open("db_rotify.sqlite3")?;
        let mut stmt = conn.prepare(
            "SELECT id, date, message, title, priority FROM message WHERE appid = :appid",
        )?;
        let messages = stmt.query_map(&[(":appid", &appid)], |row| {
            Ok(MessagePublic {
                id: row.get(0)?,
                date: row.get(1)?,
                message: row.get(2)?,
                title: row.get(3)?,
                priority: row.get(4)?,
            })
        });
        if let Ok(message) = messages {
            for row in message {
                match row {
                    Ok(row_result) => data.push(row_result),
                    Err(_e) => (),
                }
            }
        }
        let data_container: Messages = Messages { messages: (data) };
        Ok(data_container)
    }
    fn delete_app_messages(appid: String) -> Result<(), Error> {
        let conn = Connection::open("db_rotify.sqlite3")?;
        conn.execute(
            "DELETE FROM message WHERE appid = :appid", 
            &[(":appid", &appid)]
        )?;
        Ok(())
        
    }

    fn get_single(app_id: String, message_id: String) -> Result<MessagePublic, Error> {
        let conn = Connection::open("db_rotify.sqlite3")?;
        let mut stmt =
            conn.prepare("SELECT id, date, message, title, priority FROM message WHERE appid = :appid AND id = :id")?;
        let messages = stmt.query_map(&[(":appid", &app_id), (":id", &message_id)], |row| {
            Ok(MessagePublic {
                id: row.get(0)?,
                date: row.get(1)?,
                message: row.get(2)?,
                title: row.get(3)?,
                priority: row.get(4)?,
            })
        });
        let collection = messages.unwrap();
        let row = collection.last();
        match row {
            Some(row_object) => match row_object {
                Ok(e) => Ok(e),
                Err(err) => Err(err),
            },
            None => Err(Error::QueryReturnedNoRows),
        }
    }
}
/// get all messages
#[get("/message")]
pub fn list() -> Result<Json<Messages>, Status> {
    match Message::get() {
        Ok(data_formatted) => Ok(Json(data_formatted)),
        Err(_) => Err(Status::NoContent),
    }
}
/// get all messages from app
#[get("/message/<app_id>")]
pub fn list_all_app(app_id: String) -> Result<Json<Messages>, Status> {
    match Message::get_app_messages(app_id) {
        Ok(data_formatted) => Ok(Json(data_formatted)),
        Err(_) => Err(Status::NoContent),
    }
}

/// create a message
#[post("/message", data = "<message>")]
pub fn create(message: Json<NewMessage>) -> Result<Json<MessagePublic>, Status> {
    let message_request = Message::store(
        Message::new(
        message.message.to_string(),
        message.title.to_string(),
        message.priority,
        message.token.to_string(),
        )
    );
    match message_request {
        Ok(msg) => {
    let output: Result<MessagePublic, Status> = Ok(MessagePublic {
        id: (msg.id),
        message: (msg.message),
        title: (msg.title),
        priority: (msg.priority),
        date: (msg.date),
    });
    match output {
        Ok(msg) => Ok(Json(msg)),
        Err(_) => Err(Status::Forbidden)
    }
    },
    Err(_) => Err(Status::BadRequest) 
    }
}

/// find a message
#[get("/message/<app_id>/<message_id>")]
pub fn get(app_id: String, message_id: String) -> Result<Json<MessagePublic>, Status> {
    match Message::get_single(app_id, message_id) {
        Ok(found_message) => Ok(Json(found_message)),
        Err(_) => Err(Status::NoContent),
    }
}

/// delete a message
#[delete("/message/<id>")]
pub fn delete(id: String) {
    Message::delete(id).expect("Delete Failed!");
}
/// delete all an applications messages
#[delete("/application/<app_id>/message")]
pub fn delete_app_messages(app_id: String) {
    Message::delete_app_messages(app_id).expect("Delete Failed!");
}
/// delete all messages
#[delete("/message")]
pub fn delete_all() {
    Message::delete_all().expect("Delete Failed!");
}

/// fetch queued websocket messages
/// stubbed for now by fetching all stored messages.
#[get("/stream")]
pub fn stream() -> Result<Json<Messages>, Status> {
    match Message::get() {
        Ok(data_formatted) => Ok(Json(data_formatted)),
        Err(_) => Err(Status::NoContent),
    }
}
