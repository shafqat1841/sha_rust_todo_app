use serde::{self, Deserialize, Serialize};
use serde_json::{self, Result, json};
use std::{
    fs::File,
    io::{self, Read},
};

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

#[derive(Serialize, Deserialize)]
struct Person {
    name: String,
    age: u8,
    phones: Vec<String>,
}

pub fn serde_serialize() -> Result<()> {
    // Some JSON input data as a &str. Maybe this comes from the user.
    let data = r#"
        {
            "name": "John Doe",
            "age": 43,
            "phones": [
                "+44 1234567",
                "+44 2345678"
            ]
        }"#;

    // Parse the string of data into a Person object. This is exactly the
    // same function as the one that produced serde_json::Value above, but
    // now we are asking it for a Person as output.
    let p: Person = serde_json::from_str(data)?;

    // Do things just like with any other Rust data structure.
    println!("Please call {} at the number {}", p.name, p.phones[0]);

    Ok(())
}

pub fn serde_serialize_file(file: &mut File) -> Result<()> {
    // Some JSON input data as a &str. Maybe this comes from the user.
    let mut buf = String::new();
    let data = file.read_to_string(&mut buf);

    match data {
        Ok(value) => println!("File read successfully!: {}", value),
        Err(e) => {
            println!("Error reading file: {}", e);
            return Err(serde_json::Error::io(e));
        }
    }

    // let data = r#"
    //     {
    //         "name": "John Doe",
    //         "age": 43,
    //         "phones": [
    //             "+44 1234567",
    //             "+44 2345678"
    //         ]
    //     }"#;

    // Parse the string of data into a Person object. This is exactly the
    // same function as the one that produced serde_json::Value above, but
    // now we are asking it for a Person as output.
    println!("{}", buf);
    let p: Result<Person> = serde_json::from_str(&buf);
    
    match p {
        Ok(_) => println!("File deserialized successfully!"),
        Err(e) => {
            println!("Error deserializing file: {}", e);
            match e.classify() {
                serde_json::error::Category::Io => {
                    println!("I/O error occurred.")
                },
                serde_json::error::Category::Syntax => println!("Syntax error in JSON."),
                serde_json::error::Category::Data => println!("Data error during deserialization."),
                serde_json::error::Category::Eof => println!("Unexpected end of JSON input."),
            }
            return Err(e);
        }
    }
    
    // Do things just like with any other Rust data structure.
    println!("Please call {} at the number {}", p.as_ref().unwrap().name, p.as_ref().unwrap().phones[0]);

    Ok(())
}

pub fn serde_deserialize() {
    let full_name = "John2 Doe";
    let age_last_year = 52;

    let john = json!({
        "name": full_name,
        "age": age_last_year + 1,
        "phones": [
            "+44 1234567",
            "+44 2345678"
        ]
    });

    println!("first phone number: {}", john["phones"][0]);

    // Convert to a string of JSON and print it out
    println!("{}", john.to_string());
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
