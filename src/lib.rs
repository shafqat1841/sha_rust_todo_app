use std::io;

pub fn run() {
    let mut initial_text = "Welcome to the todo app!";
    loop {
        println!("{}", initial_text);
        println!("To view all todos, press 'v'.");
        println!("To add a todo, press 'a'.");
        println!("To delete a todo, press 'd'.");
        println!("To update a todo, press 'u'.");
        println!("To exit, press 'q'.");

        let mut input: String = String::new(); // Create a string variable
        match io::stdin().read_line(&mut input) {
            Ok(n) => {
                if n == 2 {
                    initial_text = "You entered an empty command! Please add a command.";
                    continue;
                }
            }
            Err(error) => {
                println!("Error reading input: {}", error);
                continue; // Skip the rest of the loop and start over
            }
        };

        println!("You entered: {}", input.trim());

        // sleep(Duration::from_secs(5));
        break;
    }
}
