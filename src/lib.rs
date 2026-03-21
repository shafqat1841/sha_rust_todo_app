mod app_constants;
mod app_default_texts;
mod file_handler_system;
mod helper;
mod todos_system;
mod whole_systems;

use std::io;

use crate::whole_systems::WholeSystem;
use crate::helper::get_user_input;

pub fn run() -> Result<(), Box<dyn std::error::Error>> {

    let mut whole_systems = WholeSystem::new()?;


    loop {
        if !whole_systems.is_initial_text_viewed() {
            println!("{}", whole_systems.get_initial_text());
        }

        let input = get_user_input();

        if input.is_err() {
            let input_ref = input.as_ref().err().unwrap();
            match input_ref.kind() {
                io::ErrorKind::InvalidInput => {
                    println!("{}", whole_systems.show_empty_command_text())
                }
                io::ErrorKind::Other => println!("Error reading input: {}", input_ref),
                _ => println!("An unexpected error occurred: {}", input_ref),
            }
            continue;
        }

        match input.unwrap().trim().to_lowercase().as_str() {
            "v" => {
                let res = whole_systems.view_all_todos();
                if res.is_err() {
                    println!("following error occured: {:?}", res.err())
                }
            }
            "a" => {
                let res = whole_systems.add_todo();
                if res.is_err() {
                    println!("following error occured: {:?}", res.err())
                }
            }
            "d" => {
                let res = whole_systems.delete_todo();
                if res.is_err() {
                    println!("following error occured: {:?}", res.err())
                }
            }
            "u" => {
                let res = whole_systems.update_todo();

                if res.is_err() {
                    println!("following error occured: {:?}", res.err())
                }
            }
            "q" => {
                println!("Exiting the app. Goodbye!");
                break;
            }
            _ => {
                println!("{}", whole_systems.get_invalid_command_text());
                continue;
            }
        }
    }

    Ok(())
}
