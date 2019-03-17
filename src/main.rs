use std::fs::File;
use demo::utilities::constant::{INVALID_FILE_PATH};

///This program is intended to show the recoverable error with match expression.


fn main()
{
    get_file_from_path();
}
fn get_file_from_path() {
    match  File::open(INVALID_FILE_PATH)
        {
            Ok(file) => println!("{:?}", file),
            Err(error) => {
                panic!("Unable to open file: {:?}", error)
            }
        };
}

#[cfg(test)]
mod test {
    use crate::get_file_from_path;

    #[test]
    #[should_panic]
    fn test_get_file_from_path() {
        get_file_from_path();
    }
}
