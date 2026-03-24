use core::fmt;
use std::io;

use crate::app_constants::{EMPTY_COMMAND_TEXT, INVALID_COMMAND};

#[derive(Debug)]
pub enum TodosErrors {
    EmptyCommandError,
    WrongCommandError,
    IOError(io::Error),
}

impl fmt::Display for TodosErrors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TodosErrors::EmptyCommandError => write!(f, "{}", EMPTY_COMMAND_TEXT),
            TodosErrors::WrongCommandError => write!(f, "{}", INVALID_COMMAND),
            TodosErrors::IOError(err) => write!(f, "Error reading input: {}", err),
        }
    }
}

impl std::error::Error for TodosErrors {}
