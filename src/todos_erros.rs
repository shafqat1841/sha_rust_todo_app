use core::fmt;
use std::io;

#[derive(Debug)]
pub enum TodosErrors {
    EmptyCommandError,
    IOError(io::Error),
}

impl fmt::Display for TodosErrors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TodosErrors::EmptyCommandError => write!(f, "Empty command entered"),
            TodosErrors::IOError(err) => write!(f, "Error reading input: {}", err)
        }
    }
}

impl std::error::Error for TodosErrors {}
