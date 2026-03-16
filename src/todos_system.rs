use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Todo {
    id: u32,
    description: String,
    completed: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Todos {
    list: Vec<Todo>,
}