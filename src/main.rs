use std::io::{Read, Error};
use std::fs::File;
use demo::utilities::constant::FILE_PATH;



///This program is intended to show the recoverable error with propagation.

fn main()
{
    println!("{:?}",read_content_from_file());
}
fn read_content_from_file() -> Result<String, Error>
{
    let mut read_file: File = match  File::open(FILE_PATH) {
        Ok(file) => file,
        Err(error) => return Err(error),
    };
    let mut content: String = String::new();
    match read_file.read_to_string(&mut content) {
        Ok(_) => Ok(content),
        Err(error) => Err(error),
    }
}

