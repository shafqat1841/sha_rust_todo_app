type StaticStr = &'static str;
pub const FILE_PATH: StaticStr = "todo_data.json";

// default texts
pub const VIEW_ALL_TODOS_TEXT: StaticStr = "To view all todos, press 'v'.";
pub const ADD_TODO_TEXT: StaticStr = "To add a todo, press 'a'.";
pub const REMOVE_TODO_TEXT: StaticStr = "To delete a todo, press 'd'.";
pub const UPDATE_TODO_DONE_TEXT: StaticStr = "To update a todo, press 'u'.";
pub const QUIT_TEXT: StaticStr = "To exit, press 'q'.";

// conditional texts
pub const WELCOME_TEXT: StaticStr = "Welcome to the todo app!";
pub const EMPTY_COMMAND_TEXT: StaticStr = "You entered an empty command! Please add a command.";
pub const INVALID_COMMAND: StaticStr = "Invalid command! Please try again.";
