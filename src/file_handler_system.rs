use std::{
    fs::{File, OpenOptions},
    io::{self, ErrorKind},
    path::Path,
};

use crate::app_constants::FILE_PATH;

pub struct FileHandler {
    file: File,
    file_path: &'static Path,
}

impl FileHandler {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let file_path = Path::new(FILE_PATH);
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(file_path)?;

            Ok(
                
                FileHandler {
                    file,
                    file_path,
                }
            )
    }
}
