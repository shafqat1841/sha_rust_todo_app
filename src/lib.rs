mod app_constants;
mod app_default_texts;
mod file_handle_test;
mod file_handler_system;
mod helper;

use std::io;

use crate::app_default_texts::DefaultTexts;
use crate::file_handler_system::FileHandler;
// use crate::helper::{ serde_deserialize, serde_serialize, serde_serialize_file};
// use crate::file_handle_test::file_test;


pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let file_handler = FileHandler::new()?;
    let mut default_texts = DefaultTexts::new();


    loop {
        if !default_texts.initial_text_viewed {
            println!("{}", default_texts.get_initial_text());
        }

        let mut input: String = String::new(); // Create a string variable
        match io::stdin().read_line(&mut input) {
            Ok(n) => {
                if n == 2 {
                    println!("{}", default_texts.show_empty_command_text());
                    continue;
                }
            }
            Err(error) => {
                println!("Error reading input: {}", error);
                continue; // Skip the rest of the loop and start over
            }
        };

        match input.trim().to_lowercase().as_str() {
            "v" => {
                file_handler.view_all_todos();
            }
            "a" => {
                file_handler.add_todo();
            }
            "d" => {
                file_handler.delete_todo();
            }
            "u" => {
                file_handler.update_todo();
            }
            "q" => {
                // println!("Exiting the app. Goodbye!");
                file_handler.quit();
                break;
            }
            _ => {
                println!("{}", default_texts.get_invalid_command_text());
                continue;
            }
        }
    }

    // let file = file_handler.get_file()?;

    // println!("File is ready to use: {:?}", file);

    // serde_serialize_file(file)?;

    // serde_serialize()?;

    // serde_deserialize();

    // aaaaa();

    // file_test();

    Ok(())
}
