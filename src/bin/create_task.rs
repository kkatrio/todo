use todo::*;

fn main() {

    let connection = &mut establish_connection();

    let title = String::from("Titlos");
    let body = String::from("Bodys");

    create_task(connection, &title, &body);

    println!("Task created");
}

