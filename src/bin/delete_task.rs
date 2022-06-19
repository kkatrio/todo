use diesel::prelude::*;
use todo::*; // including schema mod

fn main() {
    use todo::schema::tasks::dsl::*;


    let connection = &mut establish_connection();
    let num_deleted = diesel::delete(tasks)
        .filter(title.like("Titlos"))
        .execute(connection)
        .expect("Error deleting posts");

    println!("Deleted {} tasks", num_deleted);

}

