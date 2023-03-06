use rusqlite::{Connection, Error, params};
use rocket::http::Status;
use uuid::Uuid;
use rocket::*;
use rocket_contrib::json::Json;

mod structs;
use structs::{User, NewUser, CurrentUser};
#[path = "../utils.rs"]
mod utils;
impl User {
    pub fn new(name: &str, password: &str, admin: bool) -> Result<(), Error> {
        let conn = Connection::open("db_rotify.sqlite3")?;
        let user_id = 1;
    
        conn.execute(
            "INSERT OR IGNORE INTO users (id, name, admin) values (:id, :name, :admin)",
            params![
                &user_id,
                &name.to_string(),
                &admin
            ],
        )?;
        let mut stmt = conn.prepare("SELECT id, name, admin FROM users WHERE id = :id")?;
        let user = stmt.query_map(&[
            (":id", &user_id)
            ], |row| {
            Ok(User {
                id: row.get(0)?,
                name: row.get(1)?,
                admin: row.get(2)?,
            })
        });
        let collection = user.unwrap();
        let row = collection.last();
        match row {
            Some(row_object) => match row_object {
                Ok(_) => {
                    conn.execute(
                        "INSERT OR IGNORE INTO passwords (id, pass) values (:id, :pass)",
                        params![
                            &user_id,
                            &utils::hash_argon(password.to_string()),
                        ],
                    )?;
                    
                },
                Err(_err) => (),
            },
            None => (),
        }
        Ok(())
    }
    fn delete(user_id: &str) -> Result<(), Error> {
        let conn = Connection::open("db_rotify.sqlite3")?;
    
        conn.execute(
            "DELETE FROM users WHERE ID = :id",
            params![
                user_id,
            ],
        )?;
        conn.execute(
            "DELETE FROM passwords WHERE id = :id",
            &[
                (":id", user_id)
            ],
        )?;
        Ok(())
    }
}


/// create a user
#[post("/user", format = "json", data = "<user>")]
pub fn create(user: Json<NewUser>) -> Status {
    match User::new(&user.name, &user.password, user.admin) {
        Ok(_) => Status::Created,
        Err(_) => Status::Forbidden
    }
}
/// delete a user
#[delete("/user/<id>")]
pub fn delete(id: String) -> Status {
    match User::delete(&id) {
        Ok(_) => Status::Ok,
        Err(_) => Status::Forbidden
    }
}
// implement a stub for now until auth is done.
#[get("/current/user")]
pub fn current_user() -> Json<CurrentUser> {
    let dummy_user = CurrentUser {
        admin: true,
        id: 0,
        name: "admin".to_string(),
    };
    return Json(dummy_user);
}

// mirror function for inital setup.
pub fn create_user(name: String, password: String, admin: bool) {
    User::new(&name, &password, admin).unwrap();
}

