mod app_constants;
mod commands;
mod file_handler_system;
mod helper;
mod todos_erros;
mod todos_system;
mod marck_or_update_commands;

use crate::commands::AppCommands;
use crate::file_handler_system::FileHandler;
use crate::helper::{get_default_texts, get_initial_text, get_user_input};
use crate::todos_erros::TodosErrors;

pub fn run() -> Result<(), TodosErrors> {
    let mut file_handler = FileHandler::new()?;

    println!("{}", get_initial_text());
    loop {
        let input = get_user_input();

        let input_string = match input {
            Ok(val) => val,
            Err(err) => {
                println!("{}", err);

                match err {
                    todos_erros::TodosErrors::EmptyCommandError => {
                        println!("{}", get_default_texts());
                    }
                    _ => {}
                }
                continue;
            }
        };

        let command: AppCommands = match input_string.parse() {
            Err(err) => {
                println!("{}", err);
                println!("{}", get_default_texts());

                continue;
            }
            Ok(cmd) => cmd,
        };

        match command {
            AppCommands::View => {
                let res = file_handler.view_all_todos();
                if res.is_err() {
                    println!("following error occured: {:?}", res.err())
                }
            }
            AppCommands::Add => {
                let res = file_handler.add_todo();
                if res.is_err() {
                    println!("following error occured: {:?}", res.err())
                }
            }
            AppCommands::Delete => {
                let res = file_handler.delete_todo();
                if res.is_err() {
                    println!("following error occured: {:?}", res.err())
                }
            }
            AppCommands::Update => {
                let res = file_handler.update_todo();

                if res.is_err() {
                    println!("following error occured: {:?}", res.err())
                }
            }
            AppCommands::Quit => {
                println!("Exiting the app. Goodbye!");
                break;
            }
        }
        println!("{}", get_default_texts());
    }

    Ok(())
}
