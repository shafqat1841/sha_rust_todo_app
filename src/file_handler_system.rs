use std::{
    fs::{File, OpenOptions},
    io::{self, ErrorKind},
    path::Path,
};

use crate::app_constants::FILE_PATH;

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

        Ok(FileHandler { file, file_path })
    }

    pub fn view_all_todos(&self) {
        // if todos.list.len() == 0 {
        //     println!("");
        //     println!("No todos found! Please add some todos to view them.");
        //     println!("");
        //     continue;
        // }
        // println!("Viewing all todos...");
        // println!("Total todos: {}", todos.list.len());
        // for todo in &todos.list {
        //     println!(
        //         "ID: {}, Description: {}, Completed: {}",
        //         todo.id, todo.description, todo.completed
        //     );
        // }
        println!("Viewing all todos...");
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
