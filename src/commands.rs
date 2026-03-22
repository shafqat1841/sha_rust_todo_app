use core::fmt;
use std::str::FromStr;

use crate::helper::get_invalid_command_text;

pub enum AppCommands {
    View,
    Add,
    Delete,
    Update,
    Quit,
}

#[derive(Debug)]
pub struct AppCommandsError;

impl fmt::Display for AppCommandsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", get_invalid_command_text())
    }
}

impl FromStr for AppCommands {
    
    type Err = AppCommandsError;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "v" => Ok(AppCommands::View),
            "a" => Ok(AppCommands::Add),
            "d" => Ok(AppCommands::Delete),
            "u" => Ok(AppCommands::Update),
            "q" => Ok(AppCommands::Quit),
            _ => Err(AppCommandsError),
        }
    }
}
