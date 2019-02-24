use std::fs::File;
use std::io::Error;


///This program is intended to show the recoverable error with match keyword.

fn main()
{
    let data: Result<File,Error> = File::open("new_file.txt");
    match data
        {
            Ok(file) => file,
            Err(error) => {
                panic!("Unable to open file: {:?}", error)
            },
        };
}

