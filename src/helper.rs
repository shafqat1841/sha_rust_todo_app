use std::io;

use crate::{
    app_constants::{
        ADD_TODO_TEXT, QUIT_TEXT, REMOVE_TODO_TEXT, UPDATE_TODO_DONE_TEXT, VIEW_ALL_TODOS_TEXT,
        WELCOME_TEXT,
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

pub fn get_user_input() -> Result<String, TodosErrors> {
    let mut input: String = String::new();

    io::stdin().read_line(&mut input)?;

    let trimmed_input = input.trim();

    if trimmed_input.is_empty() {
        return Err(TodosErrors::EmptyCommandError);
    }

    Ok(trimmed_input.to_string())
}
