use std::{
    fs::{self, File},
    io::{BufReader, BufWriter},
    path::Path,
};

use crate::{app_constants::FILE_PATH, helper::get_user_input, todos_system::Todos};

pub struct FileHandler {
    // file: File,
    file_path: &'static Path,
}

impl FileHandler {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let file_path = Path::new(FILE_PATH);

        if !file_path.exists() || fs::metadata(file_path)?.len() == 0 {
            let json_value = Todos::new();
            let file = File::create(file_path)?;
            let writer = BufWriter::new(file);
            serde_json::to_writer_pretty(writer, &json_value)?;
        }

        Ok(FileHandler { file_path })
    }
    pub fn view_all_todos(&self) -> Result<(), Box<dyn std::error::Error>> {
        let file = File::open(self.file_path)?;
        let reader = BufReader::new(file);
        let todo_data: Todos = serde_json::from_reader(reader)?;

        if todo_data.list.is_empty() {
            println!("No todos found. Please add some todos first.");
            return Ok(());
        }

        println!("All todos are the following:");
        println!("{:#?}", todo_data);
        Ok(())
    }

    pub fn add_todo(&mut self) {
        let mut initial_text_viewed = false;
        loop {
            if !initial_text_viewed {
                println!("Please enter a description for the new todo:");
                println!("Or type 'cancel' to go back:");
                initial_text_viewed = true
            }

            let description_data = get_user_input();

            println!("description_data: {:?}", description_data);

            let description = match description_data {
                Ok(value) => {
                    if value == "cancel" {
                        println!("Canceling the process of add a todo...");
                        break;
                    }
                    value
                }
                Err(err) => {
                    println!("the following error occured: {}", err);
                    continue;
                }
            };

            let file = match File::open(self.file_path) {
                Ok(f) => f,
                Err(e) => {
                    println!("Error: {}", e);
                    continue;
                }
            };
            let reader = BufReader::new(file);
            let todos_data: serde_json::Result<Todos> = serde_json::from_reader(reader);

            let mut todos = match todos_data {
                Ok(value) => value,
                Err(e) => {
                    println!("Error: {}", e);
                    continue;
                }
            };

            todos.add_new_todo(description);

            let temp_path = self.file_path.with_extension("temp");
            let temp_file_data = File::create(&temp_path);
            let temp_file = match temp_file_data {
                Ok(v) => v,
                Err(e) => {
                    println!("Error: {}", e);
                    continue;
                }
            };
            let writer = BufWriter::new(temp_file);
            let serde_write_data = serde_json::to_writer_pretty(writer, &todos);

            match serde_write_data {
                Err(e) => {
                    println!("Error: {}", e);
                    continue;
                }
                Ok(_) => {}
            };

            let rename_data = fs::rename(&temp_path, self.file_path);

            match rename_data {
                Err(e) => {
                    println!("Error: {}", e);
                    continue;
                }
                Ok(_) => {}
            };

            break;
        }
    }

    pub fn delete_todo(&self) {
        println!("Deleting a todo...");
    }

    pub fn update_todo(&self) {
        println!("Updating a todo...");
    }

    pub fn quit(&self) {
        println!("Exiting the app. Goodbye!");
    }
}
