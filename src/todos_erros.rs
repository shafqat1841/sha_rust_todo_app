use core::fmt;
use std::{io, num::ParseIntError};

use crate::app_constants::{EMPTY_COMMAND_TEXT, INVALID_COMMAND};

// use crate::app_constants::{EMPTY_COMMAND_TEXT, INVALID_COMMAND};

#[derive(Debug)]
pub enum TodosErrors {
    EmptyCommandError,
    WrongCommandError,
    IOError(io::Error),
    SerdeJsonError(serde_json::Error),
    IntParseError(ParseIntError),
}

impl fmt::Display for TodosErrors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TodosErrors::EmptyCommandError => write!(f, "{}", EMPTY_COMMAND_TEXT),
            TodosErrors::WrongCommandError => write!(f, "{}", INVALID_COMMAND),
            TodosErrors::IOError(err) => write!(f, "Input or Output Error: {}", err),
            TodosErrors::SerdeJsonError(err) => write!(f, "Input or Output Error: {}", err),
            TodosErrors::IntParseError(err) =>  write!(f, "Not a number error: {}", err),
        }
    }
}

impl From<io::Error> for TodosErrors {
    fn from(err: io::Error) -> Self {
        TodosErrors::IOError(err)
    }
}

impl From<serde_json::Error> for TodosErrors {
    fn from(err: serde_json::Error) -> Self {
        TodosErrors::SerdeJsonError(err)
    }
}

impl From<ParseIntError> for TodosErrors {
    fn from(err: ParseIntError) -> Self {
        TodosErrors::IntParseError(err)
    }
}

impl std::error::Error for TodosErrors {}
