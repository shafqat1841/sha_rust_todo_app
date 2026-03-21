use std::io;

pub fn get_user_input() -> io::Result<String> {
    let mut input: String = String::new();
    let input_data = io::stdin().read_line(&mut input);

    let input_data_ref = input_data.as_ref();

    if input_data.is_err() {
        let new_error = io::Error::new(
            io::ErrorKind::Other,
            format!("Error reading input: {}", input_data.err().unwrap()),
        );
        let err = Err(new_error);
        return err
    }

    if input_data_ref.is_ok() && *input_data_ref.unwrap() == 2 {
        let err = Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Empty command entered",
        ));
        return err;
    }

    Ok(input.trim().to_string())
}
