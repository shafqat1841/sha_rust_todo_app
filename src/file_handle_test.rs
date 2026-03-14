use serde::{self, Deserialize, Serialize};
use std::fs::{self, File, OpenOptions};
use std::io::{BufReader, BufWriter, SeekFrom, Write, prelude::*};
use std::path::Path;

use crate::app_constants::FILE_PATH;

#[derive(Serialize, Deserialize, Debug)]
struct Person {
    name: String,
    age: u8,
}

pub fn file_test() -> Result<(), Box<dyn std::error::Error>> {

    let path = Path::new(FILE_PATH);

    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(path)?;

    let meta_data = file.metadata()?;

    let mut json_value: Vec<Person> = if meta_data.len() == 0 {
        Vec::new()
    } else {
        let reader = BufReader::new(&file);
        serde_json::from_reader(reader)?
    };

    let new_person = Person {
        name: "Alice".to_string(),
        age: 25,
    };

    json_value.push(new_person);

    let temp_path = path.with_extension("temp");
    let temp_file = File::create(&temp_path)?;
    let mut writer = BufWriter::new(temp_file);

    serde_json::to_writer_pretty(&mut writer, &json_value)?;

    writer.into_inner()?.sync_all()?;

    fs::rename(&temp_path, path)?;

    Ok(())
}

pub fn file_test2() -> Result<(), Box<dyn std::error::Error>> {
    println!("FILE_PATH: {}", FILE_PATH);
    let does_file_exist = fs::exists(FILE_PATH)?;

    if does_file_exist {
        println!("File exists: {}", does_file_exist);

        let contents = fs::read_to_string(FILE_PATH)?;

        println!("File contents: {}", contents);

        let mut json_value = serde_json::from_str::<Vec<Person>>(&contents)?;
        println!("Parsed JSON: {:?}", json_value);

        json_value.push(Person {
            name: "John Doe".to_string(),
            age: 30,
        });

        println!("Parsed JSON with new entry: {:?}", json_value);

        let json_string = serde_json::to_string_pretty(&json_value)?;
        fs::write(FILE_PATH, json_string)?;

        return Ok(());
    } else {
        println!("File exists: {}", does_file_exist);
        let mut file = File::create(FILE_PATH)?;
        file.write_all(b"[]")?;
        return Ok(());
    }
}
