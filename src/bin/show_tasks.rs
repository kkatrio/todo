use diesel::prelude::*;
use todo::*;
use crate::models::Task;

fn main() {
    use todo::schema::tasks::dsl::*; // for tasks::table

    let connection = &mut establish_connection();
    let results = tasks // tasks::table tasks is a table struct
        .limit(5)
        .load::<Task>(connection) 
        .expect("Error loading tasks");

    println!("Displaying {} tasks\n", results.len());
    for post in results {
        println!("{}", post.id);
        println!("{}", post.title);
        println!("----------");
        println!("{}", post.body);
        println!("\n");
    }
}

