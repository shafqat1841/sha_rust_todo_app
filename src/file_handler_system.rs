use std::{
    fs::{self, File, Metadata, OpenOptions},
    io::{self, BufReader, BufWriter},
    path::Path,
};

use crate::{app_constants::FILE_PATH, helper::get_user_input, todos_system::Todos};

pub struct FileHandler {
    file: File,
    file_path: &'static Path,
}

impl FileHandler {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let file_path = Path::new(FILE_PATH);
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(file_path)?;

        let metadata = file.metadata()?;

        if metadata.len() == 0 {
            let temp_path = file_path.with_extension("temp");
            let temp_file = File::create(&temp_path)?;

            let mut writer = BufWriter::new(temp_file);

            let json_value = Todos::new();

            serde_json::to_writer_pretty(&mut writer, &json_value)?;

            writer.into_inner()?.sync_all()?;

            fs::rename(&temp_path, file_path)?;
        }

        Ok(FileHandler { file, file_path })
    }

    pub fn view_all_todos(&self) -> Result<(), Box<dyn std::error::Error>> {
        let meta_data = self.file.metadata()?;

        if meta_data.len() == 0 {
            println!("No todos found. Please add some todos first.");
            return Ok(());
        }

        let reader = BufReader::new(&self.file);
        let todo_data: Todos = serde_json::from_reader(reader)?;

        println!("All todos are the following:");
        println!("{:#?}", todo_data);
        Ok(())
    }

    pub fn add_todo(&self) {
        let mut initial_text_viewed = false;
        loop {
            if !initial_text_viewed {
                println!("Please enter a description for the new todo:");
                println!("Or type 'cancel' to go back:");
                initial_text_viewed = true
            }
            let description = get_user_input();

            if description.is_err() {
                let description_ref = description.as_ref().err().unwrap();
                match description_ref.kind() {
                    io::ErrorKind::InvalidInput => {
                        println!(
                            "Empty description entered. Please provide a valid description for the todo."
                        );
                        println!("Or type 'cancel' to go back.");
                    }
                    io::ErrorKind::Other => println!("Error reading input: {}", description_ref),
                    _ => println!("An unexpected error occurred: {}", description_ref),
                }
                continue;
            }

            let reader = BufReader::new(&self.file);
            let mut todo_data: serde_json::Result<Todos> = serde_json::from_reader(reader);

            if todo_data.is_err() {
                println!("Something went wront. Please try again");
                continue;
            }

            let mut todo_data_unwrap = todo_data.unwrap();

            todo_data_unwrap.add_new_todo(description.unwrap());

            let temp_path = self.file_path.with_extension("temp");
            let temp_file = File::create(&temp_path);

            if temp_file.is_err() {
                println!("Something went wront. Please try again");
                continue;
            }

            let mut writer = BufWriter::new(temp_file.unwrap());

            let json_value = todo_data_unwrap;

            let serde_json_return = serde_json::to_writer_pretty(&mut writer, &json_value);

            if serde_json_return.is_err() {
                println!("Something went wront. Please try again");
                continue;
            }

            let into_inner_value = writer.into_inner();

            if into_inner_value.is_err() {
                println!("Something went wront. Please try again");
                continue;
            }

            let sync_all_value = into_inner_value.unwrap().sync_all();

            if sync_all_value.is_err() {
                println!("Something went wront. Please try again");
                continue;
            }

            let rename_value = fs::rename(&temp_path, self.file_path);

            if rename_value.is_err() {
                println!("Something went wront. Please try again");
                continue;
            }
        }
        //  println!("Adding a todo...");
        //             let last_id: u32 = match todos.list.last() {
        //                 Some(todo) => {
        //                     let new_id = todo.id + 1;
        //                     new_id
        //                 }
        //                 None => 1,
        //             };

        //             let new_todo = Todo {
        //                 id: last_id,
        //                 description: format!("Todo {}", last_id),
        //                 completed: false,
        //             };

        //             todos.list.push(new_todo);
        println!("Adding a todo...");
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
