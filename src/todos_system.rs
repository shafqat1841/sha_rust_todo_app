use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Todo {
    id: u32,
    description: String,
    completed: bool,
}

impl Todo {
    fn new(id: u32, description: String) -> Self {
        Todo {
            id,
            description,
            completed: false,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Todos {
    todo_number: u32,
    list: Vec<Todo>,
}

impl Todos {
    pub fn new() -> Self {
        let new_todos = Todos {
            todo_number: 0,
            list: Vec::new(),
        };

        return new_todos;
    }

    pub fn add_new_todo(&mut self, description: String) {
        self.todo_number = self.todo_number + 1;
        let new_todo = Todo::new(self.todo_number, description);
        self.list.push(new_todo) 
    }
}
