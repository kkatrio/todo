#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_sync_db_pools;
#[macro_use] extern crate diesel;

use std::fmt;
use self::diesel::prelude::*;
use rocket::serde::Serialize;
use rocket_dyn_templates::Template;
use rocket::form::{Form, Contextual};
use rocket::http::Status;

table! {
    tasks (id) {
        id -> Integer,
        description -> Text,
    }
}

#[derive(Queryable, Serialize, Debug)]
#[serde(crate = "rocket::serde")]
struct Task {
    id: i32,
    description: String,
}

#[derive(Insertable, Debug, FromForm, Clone)]
#[table_name = "tasks"]
struct NewTask {
    #[field(default = "default description")]
    description: String,
}

impl fmt::Display for Task {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> fmt::Result {
        write!(f, "-- {}", self.description)
    }
}

#[database("sqlite_database")]
struct Db(diesel::SqliteConnection);

#[derive(Debug, Serialize)]
#[serde(crate = "rocket::serde")]
struct TaskContext {
    tasks: Vec<Task>
}

impl TaskContext {
    async fn raw(db: &Db) -> TaskContext {
        let results : Vec<Task> = db.run( |conn| {
            tasks::table
                .load::<Task>(conn)
        }).await.expect("Error loading tasks");

        TaskContext { tasks: results }
    }
}

fn create_task(conn: &mut diesel::SqliteConnection, description: &str) {
    let new_task = NewTask {description: description.to_string()};
    diesel::insert_into(tasks::table)
        .values(&new_task)
        .execute(conn)
        .expect("Error creating task");
}

async fn insert_task(db: &Db, new_task: NewTask) {
    db.run(|conn| {
           diesel::insert_into(tasks::table)
           .values(new_task)
           .execute(conn)
           .expect("Error creating task");
    }).await
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

#[get("/")]
async fn index(db: Db) -> Template {
    Template::render("index", TaskContext::raw(&db).await)
}

#[post("/", data = "<form>")]
async fn new<'r>(form: Form<Contextual<'r, NewTask>>, db: Db) -> (Status, Template) {

    let new_task = form.value.clone().unwrap();
    insert_task(&db, new_task).await;

    let template = match form.value {
        Some(ref submission) => {
            println!("submission: {:#?}", submission);
            Template::render("success", &form.context)
        }
        None => Template::render("index", &form.context),
    };
    (form.context.status(), template)
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(Db::fairing())
        .mount("/", routes![index, create, delete, read, clear, new])
        .attach(Template::fairing())
}

