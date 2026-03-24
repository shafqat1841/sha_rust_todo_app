use std::io;

use crate::{
    app_constants::{
        ADD_TODO_TEXT, EMPTY_COMMAND_TEXT, INVALID_COMMAND, QUIT_TEXT, REMOVE_TODO_TEXT,
        UPDATE_TODO_DONE_TEXT, VIEW_ALL_TODOS_TEXT, WELCOME_TEXT,
    },
    todos_erros::TodosErrors,
};

pub fn get_default_texts() -> String {
    format!(
        "{}\n{}\n{}\n{}\n{}",
        VIEW_ALL_TODOS_TEXT, ADD_TODO_TEXT, REMOVE_TODO_TEXT, UPDATE_TODO_DONE_TEXT, QUIT_TEXT
    )
}

pub fn get_initial_text() -> String {
    format!("{}\n{}\n", WELCOME_TEXT, get_default_texts(),)
}

pub fn get_invalid_command_text() -> String {
    format!("{}\n{}\n", INVALID_COMMAND, get_default_texts())
}

pub fn get_user_input() -> Result<String, TodosErrors> {
    let mut input: String = String::new();
    let input_data = io::stdin().read_line(&mut input);

    let input_data_ref = input_data.as_ref();

    if input_data.is_err() {
        return Err(TodosErrors::IOError(input_data.err().unwrap()));
    }

    if input_data_ref.is_ok() && *input_data_ref.unwrap() == 2 {
        return Err(TodosErrors::EmptyCommandError);
    }

    Ok(input.trim().to_string())
}
