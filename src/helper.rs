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