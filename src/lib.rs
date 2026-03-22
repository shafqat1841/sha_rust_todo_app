mod app_constants;
mod file_handler_system;
mod helper;
mod todos_system;

use std::io;

use crate::file_handler_system::FileHandler;
use crate::helper::{
    get_default_texts, get_initial_text, get_invalid_command_text, get_user_input,
    show_empty_command_text,
};

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let mut file_handler = FileHandler::new()?;

    println!("{}", get_initial_text());
    loop {
        let input = get_user_input();

        if input.is_err() {
            let input_ref = input.as_ref().err().unwrap();
            match input_ref.kind() {
                io::ErrorKind::InvalidInput => {
                    println!("{}", show_empty_command_text())
                }
                io::ErrorKind::Other => println!("Error reading input: {}", input_ref),
                _ => println!("An unexpected error occurred: {}", input_ref),
            }
            continue;
        }

        match input.unwrap().trim().to_lowercase().as_str() {
            "v" => {
                let res = file_handler.view_all_todos();
                if res.is_err() {
                    println!("following error occured: {:?}", res.err())
                }
                get_default_texts();
            }
            "a" => {
                let res = file_handler.add_todo();
                if res.is_err() {
                    println!("following error occured: {:?}", res.err())
                }
                get_default_texts();
            }
            "d" => {
                let res = file_handler.delete_todo();
                if res.is_err() {
                    println!("following error occured: {:?}", res.err())
                }
                get_default_texts();
            }
            "u" => {
                let res = file_handler.update_todo();

                if res.is_err() {
                    println!("following error occured: {:?}", res.err())
                }
                get_default_texts();
            }
            "q" => {
                println!("Exiting the app. Goodbye!");
                break;
            }
            _ => {
                println!("{}", get_invalid_command_text());
                continue;
            }
        }
    }

    Ok(())
}
