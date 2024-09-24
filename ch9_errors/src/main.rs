use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let file_open_result = File::open("Hello_world.txt");
 
    match &file_open_result {
        Ok(file) => println!("{file:?}"),
        Err(error) => match error.kind() {
            ErrorKind::NotFound => println!("File not found!"),
            other_error => println!("Problem opening the file: {other_error:?}")
        }
    }
}
