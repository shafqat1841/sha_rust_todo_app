use std::{thread::sleep, time::Duration};

pub fn run() {
    loop {
        println!("Hello, world2222!");

        sleep(Duration::from_secs(5));
        break;
    }
}
