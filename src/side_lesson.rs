use std::fs::File;
use std::io::ErrorKind;

pub fn side_lesson() {
    let filename = "/tmp/customer.json";

    match File::open(filename) {
        Ok(file) => {
            println!("{:#?}", file);
        },
        Err(error) => {
            match error.kind() {
                ErrorKind::NotFound => {
                    match File::create(filename) {
                        Ok(file) => {
                            println!("File created");
                        }
                        Err(error) => {
                            println!("{:#?}", error);
                        }
                    }
                }
                _ => {
                    println!("{:#?}", error);
                }
            }
        }
    }
}