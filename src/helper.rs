use std::io::{self};

pub struct Todo {
    id: u32,
    description: String,
    completed: bool,
}

struct Todos {
    list: Vec<Todo>,
}

impl Todos {
    fn new() -> Self {
        Todos { list: Vec::new() }
    }
}

pub fn aaaaa() {
    let mut todos = Todos::new();
    let mut initial_text = "Welcome to the todo app!";
    loop {
        println!("{}", initial_text);
        println!("To view all todos, press 'v'.");
        println!("To add a todo, press 'a'.");
        println!("To delete a todo, press 'd'.");
        println!("To update a todo, press 'u'.");
        println!("To exit, press 'q'.");

        let mut input: String = String::new(); // Create a string variable
        match io::stdin().read_line(&mut input) {
            Ok(n) => {
                if n == 2 {
                    initial_text = "You entered an empty command! Please add a command.";
                    continue;
                }
            }
            Err(error) => {
                println!("Error reading input: {}", error);
                continue; // Skip the rest of the loop and start over
            }
        };

        // println!("You entered: {}", input.trim().to_lowercase());

        match input.trim().to_lowercase().as_str() {
            "v" => {
                if todos.list.len() == 0 {
                    println!("");
                    println!("No todos found! Please add some todos to view them.");
                    println!("");
                    continue;
                }
                println!("Viewing all todos...");
                println!("Total todos: {}", todos.list.len());
                for todo in &todos.list {
                    println!(
                        "ID: {}, Description: {}, Completed: {}",
                        todo.id, todo.description, todo.completed
                    );
                }
            }
            "a" => {
                println!("Adding a todo...");
                let last_id: u32 = match todos.list.last() {
                    Some(todo) => {
                        let new_id = todo.id + 1;
                        new_id
                    }
                    None => 1,
                };

                let new_todo = Todo {
                    id: last_id,
                    description: format!("Todo {}", last_id),
                    completed: false,
                };

                todos.list.push(new_todo);
            }
            "d" => println!("Deleting a todo..."),
            "u" => println!("Updating a todo..."),
            "q" => {
                println!("Exiting the app. Goodbye!");
                break;
            }
            _ => {
                initial_text = "Invalid command! Please try again.";
                continue;
            }
        }
    }
}
