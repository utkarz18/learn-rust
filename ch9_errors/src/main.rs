use std::fs::File;

fn main() {
    let file_open_result = File::open("Hello.txt");
    match file_open_result {
        Ok(file) => println!("{file:?}"),
        Err(error) => println!("Problem opening the file: {error:?}")
    }
}
