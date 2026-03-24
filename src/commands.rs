use std::str::FromStr;

use crate::todos_erros::TodosErrors;

pub enum AppCommands {
    View,
    Add,
    Delete,
    Update,
    Quit,
}

impl FromStr for AppCommands {
    type Err = TodosErrors;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "v" => Ok(AppCommands::View),
            "a" => Ok(AppCommands::Add),
            "d" => Ok(AppCommands::Delete),
            "u" => Ok(AppCommands::Update),
            "q" => Ok(AppCommands::Quit),
            _ => Err(TodosErrors::WrongCommandError),
        }
    }
}
