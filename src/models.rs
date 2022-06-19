use crate::schema::tasks;

#[derive(Queryable)]
pub struct Task {
    pub id: i32,
    pub title: String,
    pub body: String,
}

#[derive(Insertable)]
#[table_name = "tasks"]
pub struct NewTask<'a> {
    pub title: &'a str,
    pub body: &'a str,
}


