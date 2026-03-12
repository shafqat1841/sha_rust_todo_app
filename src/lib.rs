mod file_handler_system;
mod helper;
mod file_handle_test;
mod app_constants;

use crate::file_handler_system::FileHandler;
use crate::helper::{aaaaa, serde_deserialize, serde_serialize, serde_serialize_file};
use crate::file_handle_test::file_test;

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    // let mut file_handler = FileHandler::new();
    // file_handler.open_file()?;

    // let file = file_handler.get_file()?;

    // println!("File is ready to use: {:?}", file);

    // serde_serialize_file(file)?;

    // serde_serialize()?;

    // serde_deserialize();

    // aaaaa();

    file_test();



    Ok(())
}
