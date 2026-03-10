use std::process;

use todo_app::run;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    if let Err(e) = run() {
        eprintln!("Error running app: {}", e);
        process::exit(1);
    }
    Ok(())
}
