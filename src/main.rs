#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_sync_db_pools;
#[macro_use] extern crate diesel;

use std::fmt;
use self::diesel::prelude::*;
use rocket::serde::{Serialize}; //,json::Json

table! {
    tasks (id) {
        id -> Integer,
        description -> Text,
    }
}

#[derive(Queryable, Serialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct Task {
    pub id: i32,
    pub description: String,
}

#[derive(Insertable)]
#[table_name = "tasks"]
pub struct NewTask<'a> {
    pub description: &'a str,
}

impl fmt::Display for Task {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> fmt::Result {
        write!(f, "-- {}", self.description)
    }
}

#[database("sqlite_database")]
struct Db(diesel::SqliteConnection);

fn print_list(v: &Vec<Task>) -> String {
    let mut output = String::new();
    for i in v {
        output.push_str("-- ");
        output.push_str(&i.description);
        output.push_str("\n");
    }
    output
}

#[get("/")]
async fn list(db: Db) -> String {
    let results : Vec<Task> = db.run( |conn| {
        tasks::table
            .load::<Task>(conn)
    }).await.expect("Error loading tasks");
    print_list(&results)
}

fn create_task(conn: &mut diesel::SqliteConnection, description: &str) {
    let new_task = NewTask {description};
    diesel::insert_into(tasks::table)
        .values(&new_task)
        .execute(conn)
        .expect("Error creating task");
}

#[get("/new")]
async fn create(db: Db) {
    let description = "web description";
    db.run(|conn| create_task(conn, description)).await
}

#[get("/delete/<id>")]
async fn delete(db: Db, id: i32) {
    db.run(move |conn| {
        diesel::delete(tasks::table)
        .filter(tasks::id.eq(id))
        .execute(conn)
        .expect("Error deleting")
    }).await;
}

#[get("/<id>")]
async fn read(db: Db, id: i32) -> String {
    let entry : Task = db.run( move |conn| {
        tasks::table
            .filter(tasks::id.eq(id))
            .first(conn)
    }).await.expect("Error retireving id"); 

    format!("{}", entry)
}

#[delete("/")]
async fn clear(dn: Db) {
    dn.run( |conn| {
        diesel::delete(tasks::table)
        .execute(conn)
    }).await.expect("Error deleting table");
}


#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(Db::fairing())
        .mount("/", routes![list, create, delete, read, clear])
}

