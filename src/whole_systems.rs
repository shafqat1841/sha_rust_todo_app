use crate::{app_default_texts::DefaultTexts, file_handler_system::FileHandler};

pub struct WholeSystem {
    pub file_handler: FileHandler,
    pub default_texts: DefaultTexts,
}

impl WholeSystem {
    pub fn new() -> Result<WholeSystem, Box<dyn std::error::Error>> {
        let file_handler = FileHandler::new()?;
        let default_texts = DefaultTexts::new();

        Ok(WholeSystem {
            file_handler,
            default_texts,
        })
    }

    pub fn is_initial_text_viewed(&self) -> bool {
        self.default_texts.initial_text_viewed
    }

    pub fn get_initial_text(&mut self) -> String {
        let initial_text = self.default_texts.get_initial_text();
        initial_text
    }

    pub fn show_empty_command_text(&self) -> String {
        let empty_command_text = self.default_texts.show_empty_command_text();
        empty_command_text
    }

    pub fn get_invalid_command_text(&self) -> String {
        let invalid_command_text = self.default_texts.get_invalid_command_text();
        invalid_command_text
    }

    pub fn view_all_todos(&self) -> Result<(), Box<dyn std::error::Error>> {
        let res = self.file_handler.view_all_todos();
        res
    }

    pub fn add_todo(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let res = self.file_handler.add_todo();
        res
    }

    pub fn delete_todo(&self) -> Result<(), Box<dyn std::error::Error>> {
        let res = self.file_handler.delete_todo();
        res
    }

    pub fn update_todo(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let res = self.file_handler.update_todo();
        res
    }
}
