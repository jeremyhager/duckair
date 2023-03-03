use std::io::{Error, Read};
use std::fs::File;

pub fn side_lesson() {
    let filename = "/tmp/example_data.txt";
    let file_data = read_file(filename);

    match file_data {
        Ok(data) => {
            println!("{}", data);
        }
        Err(_) => {

        }
    }
}

fn read_file(filename: &str) -> Result<String, Error> {
    let mut file_handle = File::open(filename)?;
    let mut file_data = String::new();

    file_handle.read_to_string(&mut file_data)?;
    Ok(file_data)
}