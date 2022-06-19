#[macro_use]
extern crate diesel;

pub mod schema;
pub mod models;

use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;

use crate::models::NewTask;

pub fn create_task<'a>(conn: &mut SqliteConnection, title: &'a str, body: &'a str) {

    use schema::tasks;

    let new_task = NewTask {
        title,
        body
    };

    diesel::insert_into(tasks::table)
        .values(&new_task)
        .execute(conn)
        .expect("Error inserting task");
}

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}
