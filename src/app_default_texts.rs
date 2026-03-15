pub struct DefaultTexts {
    welcome_text: &'static str,
    view_all_todos_text: &'static str,
    add_todo_text: &'static str,
    remove_todo_text: &'static str,
    update_todo_done_text: &'static str,
    quit_text: &'static str,
}

impl DefaultTexts {
    pub fn new() -> Self {
        DefaultTexts {
            welcome_text: "Welcome to the todo app!",
            view_all_todos_text: "To view all todos, press 'v'.",
            add_todo_text: "To add a todo, press 'a'.",
            remove_todo_text: "To delete a todo, press 'd'.",
            update_todo_done_text: "To update a todo, press 'u'.",
            quit_text: "To exit, press 'q'.",
        }
    }

    pub fn get_initial_text(&self) -> String {
        format!(
            "{}\n{}\n{}\n{}\n{}\n{}",
            self.welcome_text,
            self.view_all_todos_text,
            self.add_todo_text,
            self.remove_todo_text,
            self.update_todo_done_text,
            self.quit_text
        )
    }
}
