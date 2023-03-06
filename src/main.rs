#![feature(proc_macro_hygiene, decl_macro)]

use rocket::{routes};
mod app;
mod constants;
mod health;
mod message;
mod response;
mod setup;
mod utils;
mod user;
mod client;
mod version;
mod websockets;

fn main()  {
    setup::db_setup().expect("DB setup failed!");
    rocket::ignite()
        .mount(
            "/",
            routes![
                message::list,
                message::list_all_app,
                message::get,
                message::create,
                message::delete,
                message::delete_all,
                message::delete_app_messages,
                message::stream,
                user::create,
                user::delete,
                user::current_user,
                app::list,
                app::get,
                app::create,
                app::delete,
                app::update,
                client::create,
                health::healthcheck,
                version::version
                ],
        )
        .launch();
}
