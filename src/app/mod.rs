use rusqlite::{Connection, Error, Result, params};
use rocket::http::Status;
use rocket::*;
use rocket_contrib::json::Json;
use uuid::Uuid;

mod structs;
use structs::{App, Apps, AppUpdate};
impl App {
    pub fn new(name: String) -> Self {
        Self {
            id: 1,
            name,
            token: Uuid::new_v4().to_string(),
        }
    }
    fn store(app: App) -> Result<App, Error> {
        let conn = Connection::open("db_rotify.sqlite3")?;
        conn.execute(
            "INSERT INTO app (name, token, internal) values (:name, :token, :internal)",
            params![
                &app.name,
                &app.token,
                &String::from("TRUE")
                ]
        )?;
        Ok(app)
    }
    fn update(app_id: String, title: String, description: String) -> Result<String, Error> {
        let conn = Connection::open("db_rotify.sqlite3")?;
        conn.execute(
            "UPDATE app SET name = :name, description = :description WHERE id = :id",
            &[
                (":id", &app_id),
                (":name", &title),
                (":description", &description)
                ]
        )?;
        Ok(app_id)
    }
    fn delete(id: String) -> Result<Status, Error> {
        let conn = Connection::open("db_rotify.sqlite3")?;
        let mut stmt =
            conn.prepare("SELECT id, name, token FROM app WHERE id = :id")?;
        let messages_to_delete = stmt.query_map(&[(":id", &id)], |row| {
            Ok(App {
                id: row.get(0)?,
                name: row.get(1)?,
                token: row.get(2)?,
            })
        })?;
        let mut i = 0;
        for delete_message in messages_to_delete {
            let app = delete_message.unwrap();
            conn.execute("DELETE from app WHERE id = :id", &[(":id", &app.id)])?;
            i = i + 1;
        }
        if i > 0 {
            Ok(Status::Ok)
        } else {
            Err(Error::QueryReturnedNoRows)
        }
    }
    fn get() -> Result<Apps, Error> {
        let mut data: Vec<App> = Vec::new();
        let conn = Connection::open("db_rotify.sqlite3")?;
        let mut stmt = conn.prepare("SELECT id, name, token FROM app")?;
        let messages = stmt.query_map([], |row| {
            Ok(App {
                id: row.get(0)?,
                name: row.get(1)?,
                token: row.get(2)?,
            })
        });
        if let Ok(message) = messages {
            for row in message {
                data.push(row.unwrap());
            }
        }
        let data_container: Apps = Apps { apps: (data) };
        Ok(data_container)
    }

    fn get_single(id: String) -> Result<App, Error> {
        let conn = Connection::open("db_rotify.sqlite3")?;
        let mut stmt =
            conn.prepare("SELECT id, name, token FROM app WHERE id = :id")?;
        let apps = stmt.query_map(&[(":id", &id)], |row| {
            Ok(App {
                id: row.get(0)?,
                name: row.get(1)?,
                token: row.get(2)?,
            })
        });
        let collection = apps.unwrap();
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

#[post("/application", data = "<app>")]
pub fn create(app: Json<String>) -> Result<Json<App>, Status> {
    match App::store(App::new(app.to_string())) {
        Ok(app) => Ok(Json(app)),
        Err(_) => Err(Status::Forbidden)
    }
}

/// list all apps
#[get("/application")]
pub fn list() -> Result<Json<Apps>, Status> {
    match App::get() {
        Ok(apps) => Ok(Json(apps)),
        Err(_) => Err(Status::NoContent)
    }
}
#[get("/application/<id>")]
pub fn get(id: String) -> Result<Json<App>, Status> {
    match App::get_single(id) {
        Ok(app) => Ok(Json(app)),
        Err(_) => Err(Status::NotFound)
    }
}
#[put("/application/<id>", data = "<update>")]
pub fn update(id: String, update: Json<AppUpdate>) -> Result<Json<App>, Status> {
    match App::update(id, update.name.to_string(), update.description.to_string()) {
        Ok(app_id) => Ok(Json(App::get_single(app_id).unwrap())),
        Err(_) => Err(Status::NotFound)
    }
}

#[delete("/appplication/<id>")]
pub fn delete(id: String) -> Result<Status, Status> {
    match App::delete(id) {
        Ok(s) => Ok(s),
        Err(_) => Err(Status::NotFound)
    }
}
