use std::fs::File;
use demo::utilities::constant::INVALID_FILE_PATH;


///This program is intended to show the recoverable error with match expression shortcuts(unwrap() and expect()).

fn main()
{
    File::open(INVALID_FILE_PATH).unwrap();

    //File::open(INVALID_FILE_PATH).expect("Unable to open file");
}

