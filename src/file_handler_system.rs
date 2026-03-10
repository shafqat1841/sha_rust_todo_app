use std::{
    fs::File,
    io::{self, ErrorKind},
    path::Path,
};

pub struct FileHandler {
    file: Option<File>,
    file_path: &'static Path,
}

impl FileHandler {
    pub fn new() -> Self {
        FileHandler { file: None, file_path: Path::new("todo_data.json") }
    }
    pub fn open_file(self: &mut Self) -> Result<(), Box<dyn std::error::Error>> {
        let file_result: Result<File, io::Error> = File::open(self.file_path);

        match file_result {
            Ok(file) => {
                println!("File opened successfully!");
                self.file = Some(file);
                Ok(())
            }
            Err(e) => match e.kind() {
                ErrorKind::NotFound => {
                    self.create_file()?;
                    Ok(())
                }
                _ => Err(Box::new(e)),
            },
        }
    }

    pub fn create_file(self: &mut Self) -> Result<(), Box<dyn std::error::Error>> {
        let file_result: Result<File, io::Error> = File::create(self.file_path);

        match file_result {
            Ok(file) => {
                println!("File created and opened successfully!");
                self.file = Some(file);
                Ok(())
            }
            Err(e) => Err(Box::new(e)),
        }
    }
}
