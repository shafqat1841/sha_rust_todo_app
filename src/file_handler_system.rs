use std::{
    fs::{File, OpenOptions},
    io::BufReader,
    path::Path,
};

use crate::{app_constants::FILE_PATH, todos_system::Todos};

pub struct FileHandler {
    file: File,
}

impl FileHandler {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let file_path = Path::new(FILE_PATH);
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(file_path)?;

        Ok(FileHandler { file })
    }

    pub fn view_all_todos(&self) -> Result<(), Box<dyn std::error::Error>> {
        let meta_data = self.file.metadata()?;

        if meta_data.len() == 0 {
            println!("No todos found. Please add some todos first.");
            return Ok(());
        }

        let reader = BufReader::new(&self.file);
        let todo_data: Vec<Todos> = serde_json::from_reader(reader)?;

        println!("All todos are the following:");
        println!("{:#?}", todo_data);
        Ok(())
    }

    pub fn add_todo(&self) {
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
