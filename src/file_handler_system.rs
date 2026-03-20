use std::{
    fs::{self, File},
    io::{self, BufReader, BufWriter},
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
        println!("");
        for todo in todo_data.list {
            println!("ID: {:#?}", todo.id);
            println!("Description: {:#?}", todo.description);
            println!("Is complete: {:#?}", todo.completed);
            println!("");
        }
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

    pub fn delete_todo(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.view_all_todos()?;

        println!("Please enter the id of the todo which you want to delete:");
        println!("Or type 'cancel' to go back:");

        loop {
            let id_result = get_user_input();

            println!("id_result: {:?}", id_result);

            let id = match id_result {
                Ok(value) => {
                    if value == "cancel" {
                        println!("Canceling the process of removing a todo...");
                        break;
                    }
                    let number = value.parse::<u32>();

                    match number {
                        Err(err) => {
                            println!("the following error occured: {}", err);
                            continue;
                        }
                        Ok(v) => v,
                    }
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

            todos.remove_todo(id);

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
        Ok(())
    }

    fn get_file_data(&self) -> Result<Todos, Box<dyn std::error::Error>> {
        let file = File::open(self.file_path)?;

        let reader = BufReader::new(file);
        let todos: Todos = serde_json::from_reader(reader)?;

        Ok(todos)
    }

    fn update_file_data(&self, data: &Todos) -> Result<(), Box<dyn std::error::Error>> {
        let temp_path = self.file_path.with_extension("temp");
        let temp_file = File::create(&temp_path)?;

        let writer = BufWriter::new(temp_file);
        serde_json::to_writer_pretty(writer, data)?;

        let rename_data = fs::rename(&temp_path, self.file_path)?;

        Ok(rename_data)
    }

    fn done_undone_todo(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.view_all_todos()?;

        println!("Enter the Id whose completed status you want to change");
        println!("Or type 'cancel' to go back:");

        let done_or_undone_id = get_user_input()?;

        if done_or_undone_id == "cancel" {
            println!("Canceling the process of removing a todo...");
            return Ok(());
        }

        let id_number = done_or_undone_id.parse::<u32>()?;

        let mut todos = self.get_file_data()?;

        todos.done_undone_todo(id_number);

        let res = self.update_file_data(&todos)?;

        Ok(res)
    }

    fn update_todo_description(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.view_all_todos()?;

        println!("Enter the Id whose description you want to change:");
        println!("Or type 'cancel' to go back:");

        let new_description_id = get_user_input()?;

        if new_description_id == "cancel" {
            println!("Canceling the process of removing a todo...");
            return Ok(());
        }

        let id_number = new_description_id.parse::<u32>()?;

        println!("Enter the new description:");
        println!("Or type 'cancel' to go back:");

        let new_description = get_user_input()?;

        if new_description == "cancel" {
            println!("Canceling the process of removing a todo...");
            return Ok(());
        }

        let mut todos = self.get_file_data()?;

        todos.update_todo_description(id_number, new_description.as_str());

        let res = self.update_file_data(&todos)?;

        Ok(res)
    }

    pub fn update_todo(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        println!("If you want a todo to be marked as completed or uncompleted then press m:");
        println!("If you want update a todo description press u:");
        println!("Or type 'cancel' to go back:");

        let marck_or_update = get_user_input()?;
        println!("marck_or_update: {:?}", marck_or_update);

        if marck_or_update == "cancel" {
            println!("Canceling the process of removing a todo...");
            return Ok(());
        }
        if marck_or_update == "m" {
            println!("Canceling the process of removing a todo...");
            let res = self.done_undone_todo()?;
            return Ok(res);
        }
        if marck_or_update == "u" {
            println!("Canceling the process of removing a todo...");
            let res = self.update_todo_description()?;
            return Ok(res);
        }

        let new_error = io::Error::new(io::ErrorKind::InvalidData, "wrong command");

        return Err(Box::new(new_error));
    }

    pub fn quit(&self) {
        println!("Exiting the app. Goodbye!");
    }
}
