mod file_handler_system;
mod helper;

use crate::file_handler_system::FileHandler;
use crate::helper::aaaaa;


pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let mut file_handler = FileHandler::new();
    file_handler.open_file()?;

    aaaaa();

    Ok(())
}

