use core::fmt;
use std::io;

use crate::helper::get_invalid_command_text;

#[derive(Debug)]
pub enum TodosErrors {
    EmptyCommandError,
    WrongCommandError,
    IOError(io::Error),
}

impl fmt::Display for TodosErrors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TodosErrors::EmptyCommandError => write!(f, "Empty command entered"),
            TodosErrors::IOError(err) => write!(f, "Error reading input: {}", err),
            TodosErrors::WrongCommandError => write!(f, "{}", get_invalid_command_text())
        }
    }
}

impl std::error::Error for TodosErrors {}
