use std::str::FromStr;

use crate::todos_erros::TodosErrors;

pub enum MarkOrUpdateCommands {
    Cancel,
    Mark,
    Update,
}

impl FromStr for MarkOrUpdateCommands {
    type Err = TodosErrors;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "cancel" => Ok(MarkOrUpdateCommands::Cancel),
            "m" => Ok(MarkOrUpdateCommands::Mark),
            "u" => Ok(MarkOrUpdateCommands::Update),
            _ => Err(TodosErrors::WrongCommandError),
        }
    }
}
