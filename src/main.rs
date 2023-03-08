#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;

use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;

mod models;
mod responses;
mod routes;
mod schema;
mod structs;

#[launch]
fn rocket() -> _ {
    println!("Hello, world!");

    rocket::build().mount(
        "/api/v1",
        routes![
            routes::take_a_sip_of_tea,
            routes::get_user,
            routes::get_leaderboard
        ],
    )
}

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}
