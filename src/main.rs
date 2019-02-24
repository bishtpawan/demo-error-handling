use std::io::{Read, Error};
use std::fs::File;
use demo::utilities::constant::FILE_PATH;



///This program is intended to show the recoverable error with shortcut of propagating errors.

fn main()
{
    println!("{:?}",read_content_from_file());
}
fn read_content_from_file() -> Result<String, Error>
{
    let mut read_file: File =File::open(FILE_PATH)?;
    let mut content: String = String::new();
    read_file.read_to_string(&mut content)?;
    Ok(content)
}

