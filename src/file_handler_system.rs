use std::{
    fs::{self, File},
    io::{BufReader, BufWriter},
    path::PathBuf,
};

use crate::{
    app_constants::FILE_PATH, helper::get_user_input, todos_erros::TodosErrors, todos_system::Todos,
};

pub struct FileHandler {
    file_path: PathBuf,
}

impl FileHandler {
    pub fn new() -> Result<Self, TodosErrors> {
        let file_path = PathBuf::from(FILE_PATH);

        if !file_path.exists() || fs::metadata::<&PathBuf>(&file_path)?.len() == 0 {
            let json_value = Todos::new();
            let file = File::create(&file_path)?;
            let writer = BufWriter::new(file);
            serde_json::to_writer_pretty(writer, &json_value)?;
        }

        Ok(FileHandler { file_path })
    }

    fn get_file_data(&self) -> Result<Todos, TodosErrors> {
        let file = File::open(&self.file_path)?;

        let reader = BufReader::new(file);
        let todos: Todos = serde_json::from_reader(reader)?;

        Ok(todos)
    }

    fn update_file_data(&self, data: &Todos) -> Result<(), TodosErrors> {
        let temp_path = self.file_path.with_extension("temp");
        let temp_file = File::create(&temp_path)?;

        let writer = BufWriter::new(temp_file);
        serde_json::to_writer_pretty(writer, data)?;

        let rename_data = fs::rename(&temp_path, &self.file_path)?;

        Ok(rename_data)
    }

    pub fn view_all_todos(&self) -> Result<(), TodosErrors> {
        let todos = self.get_file_data()?;

        if todos.list.is_empty() {
            println!("No todos found. Please add some todos first.");
            return Ok(());
        }

        println!("");
        println!("All todos are the following:");
        println!("");
        for todo in todos.list {
            println!(
                "ID: {:#?} | Description: {:#?} | Is complete: {:#?}",
                todo.id, todo.description, todo.completed
            );
            println!("");
        }
        Ok(())
    }

    pub fn add_todo(&mut self) -> Result<(), TodosErrors> {
        println!("Please enter a description for the new todo:");
        println!("Or type 'cancel' to go back:");

        let description = get_user_input()?;

        if description == "cancel" {
            println!("The process is canceled.");

            return Ok(());
        }

        let mut todos = self.get_file_data()?;

        todos.add_new_todo(description);

        let res = self.update_file_data(&todos)?;

        println!("The process is done.");

        Ok(res)
    }

    pub fn delete_todo(&self) -> Result<(), TodosErrors> {
        self.view_all_todos()?;

        println!("Please enter the id of the todo which you want to delete:");
        println!("Or type 'cancel' to go back:");

        let id = get_user_input()?;

        if id == "cancel" {
            println!("The process is canceled.");

            return Ok(());
        }

        let id_number = id.parse::<u32>()?;

        let mut todos = self.get_file_data()?;

        todos.remove_todo(id_number);

        let res = self.update_file_data(&todos)?;

        println!("The process is done.");

        Ok(res)
    }

    fn done_undone_todo(&mut self) -> Result<(), TodosErrors> {
        self.view_all_todos()?;

        println!("Enter the Id whose completed status you want to change");
        println!("Or type 'cancel' to go back:");

        let done_or_undone_id = get_user_input()?;

        if done_or_undone_id == "cancel" {
            println!("The process is canceled.");

            return Ok(());
        }

        let id_number = done_or_undone_id.parse::<u32>()?;

        let mut todos = self.get_file_data()?;

        todos.done_undone_todo(id_number);

        let res = self.update_file_data(&todos)?;

        println!("The process is done.");

        Ok(res)
    }

    fn update_todo_description(&mut self) -> Result<(), TodosErrors> {
        self.view_all_todos()?;

        println!("Enter the Id whose description you want to change:");
        println!("Or type 'cancel' to go back:");

        let new_description_id = get_user_input()?;

        if new_description_id == "cancel" {
            println!("The process is canceled.");

            return Ok(());
        }

        let id_number = new_description_id.parse::<u32>()?;

        println!("Enter the new description:");
        println!("Or type 'cancel' to go back:");

        let new_description = get_user_input()?;

        if new_description == "cancel" {
            println!("The process is canceled.");

            return Ok(());
        }

        let mut todos = self.get_file_data()?;

        todos.update_todo_description(id_number, new_description.as_str());

        let res = self.update_file_data(&todos)?;

        println!("The process is done.");

        Ok(res)
    }

    pub fn update_todo(&mut self) -> Result<(), TodosErrors> {
        println!("If you want a todo to be marked as completed or uncompleted then press m:");
        println!("If you want update a todo description press u:");
        println!("Or type 'cancel' to go back:");

        let marck_or_update = get_user_input()?;
        println!("marck_or_update: {:?}", marck_or_update);

        if marck_or_update == "cancel" {
            println!("The process is canceled.");

            return Ok(());
        }
        if marck_or_update == "m" {
            let res = self.done_undone_todo()?;
            return Ok(res);
        }
        if marck_or_update == "u" {
            let res = self.update_todo_description()?;
            return Ok(res);
        }

        return Err(TodosErrors::WrongCommandError);
    }
}
