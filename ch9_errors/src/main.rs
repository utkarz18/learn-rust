use std::fs::File;
use std::io::{Error, ErrorKind, Read};

fn main() {
    let file_open_result = File::open("Hello_world.txt");

    match &file_open_result {
        Ok(file) => println!("{file:?}"),
        Err(error) => match error.kind() {
            ErrorKind::NotFound => println!("File not found!"),
            other_error => println!("Problem opening the file: {other_error:?}"),
        },
    }

    File::open("hello.txt").expect("File Not Found!");
    read_file();
}

fn read_file() -> Result<String, Error> {
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}