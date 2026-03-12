use serde::{self, Deserialize, Serialize};
use std::fs::{self, File};
use std::io::{BufReader,BufWriter, Write, prelude::*};

use crate::app_constants::FILE_PATH;

#[derive(Serialize, Deserialize, Debug)]
struct Person {
    name: String,
    age: u8,
}

pub fn file_test() -> Result<(), Box<dyn std::error::Error>> {
    println!("FILE_PATH: {}", FILE_PATH);
    let does_file_exist = fs::exists(FILE_PATH)?;

    if does_file_exist {
        println!("File exists: {}", does_file_exist);

        let file = File::open(FILE_PATH)?;
        let mut buf_reader = BufReader::new(&file);
        let mut contents = String::new();
        buf_reader.read_to_string(&mut contents)?;
        println!("File contents: {}", contents);

        // let json_value = serde_json::to_vec_pretty(&contents)?;
        let mut json_value = serde_json::from_str::<Vec<Person>>(&contents)?;
        println!("Parsed JSON: {:?}", json_value);
        
        json_value.push(Person {
            name: "John Doe".to_string(),
            age: 30,
        });
        println!("Parsed JSON: {:?}", json_value);

        let mut buf_writer = BufWriter::new(file);


        serde_json::to_writer(&mut buf_writer, &json_value)?;

        buf_writer.flush()?;

        return Ok(());
    } else {
        println!("File exists: {}", does_file_exist);
        let mut file = File::create(FILE_PATH)?;
        file.write_all(b"[]")?;
        return Ok(());
    }
}
