pub struct DefaultTexts {
    // default texts
    view_all_todos_text: &'static str,
    add_todo_text: &'static str,
    remove_todo_text: &'static str,
    update_todo_done_text: &'static str,
    quit_text: &'static str,
    // conditional texts
    welcome_text: &'static str,
    empty_command_text: &'static str,
    invalid_command: &'static str,
    // states
    pub initial_text_viewed: bool,
}

impl DefaultTexts {
    pub fn new() -> Self {
        DefaultTexts {
            view_all_todos_text: "To view all todos, press 'v'.",
            add_todo_text: "To add a todo, press 'a'.",
            remove_todo_text: "To delete a todo, press 'd'.",
            update_todo_done_text: "To update a todo, press 'u'.",
            quit_text: "To exit, press 'q'.",
            welcome_text: "Welcome to the todo app!",
            empty_command_text: "You entered an empty command! Please add a command.",
            invalid_command: "Invalid command! Please try again.",
            initial_text_viewed: false,
        }
    }

    pub fn get_default_texts(&self) -> String {
        format!(
            "{}\n{}\n{}\n{}\n{}",
            self.view_all_todos_text,
            self.add_todo_text,
            self.remove_todo_text,
            self.update_todo_done_text,
            self.quit_text
        )
    }

    pub fn get_initial_text(&mut self) -> String {
        self.initial_text_viewed = true;
        format!("{}\n{}\n", self.welcome_text, self.get_default_texts(),)
    }

    pub fn show_empty_command_text(&self) -> String {
        format!(
            "{}\n{}\n",
            self.empty_command_text,
            self.get_default_texts(),
        )
    }

    pub fn get_invalid_command_text(&self) -> String {
        format!("{}\n{}\n", self.invalid_command, self.get_default_texts())
    }
}
