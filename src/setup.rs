use rusqlite::{Connection, Result};
#[path = "./user/mod.rs"]
mod user;

use crate::constants::{DEFAULT_USER, DEFAULT_PASSWORD};

pub fn db_setup() -> Result<()> {
    let conn = Connection::open("db_rotify.sqlite3")?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS message (
             id INTEGER PRIMARY KEY,
             appid INTEGER NOT NULL,
             date STRING NOT NULL,
             message STRING NOT NULL,
             title STRING NOT NULL,
             priority INTEGER NOT NULL
         )",
        [],
    )?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS app (
             id INTEGER PRIMARY KEY,
             name STRING NOT NULL,
             description STRING NULL,
             image STRING NULL,
             internal BOOL NOT NULL,
             token STRING UNIQUE NOT NULL
         )",
        [],
    )?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS users (
             id INTEGER PRIMARY KEY,
             admin BOOL NOT NULL,
             name STRING UNIQUE NOT NULL
         )",
        [],
    )?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS passwords (
            pass STRING NOT NULL,
            id INTEGER PRIMARY KEY NOT NULL
         )",
        [],
    )?;
    // we've told the DB to close the connection, it's not too important to check if it has.
    conn.close().unwrap();
    // create default user if doesn't already exist
    user::create_user(DEFAULT_USER.to_string(), DEFAULT_PASSWORD.to_string(), true);
    Ok(())
}
