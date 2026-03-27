use std::str::FromStr;

use crate::{helper::get_user_input, marck_or_update_commands::MarkOrUpdateCommands, todos_erros::TodosErrors};

pub enum AppCommands {
    View,
    Add,
    Delete,
    Update(MarkOrUpdateCommands),
    Quit,
}

impl FromStr for AppCommands {
    type Err = TodosErrors;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "v" => Ok(AppCommands::View),
            "a" => Ok(AppCommands::Add),
            "d" => Ok(AppCommands::Delete),
            "u" => {
                println!(
                    "If you want a todo to be marked as completed or uncompleted then press m:"
                );
                println!("If you want update a todo description press u:");
                println!("Or type 'cancel' to go back:");

                let sub_cmd: MarkOrUpdateCommands = get_user_input()?.parse()?;
                Ok(AppCommands::Update(sub_cmd))
            }
            "q" => Ok(AppCommands::Quit),
            _ => Err(TodosErrors::WrongCommandError),
        }
    }
}
