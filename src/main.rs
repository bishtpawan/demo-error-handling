use std::fs::File;


///This program is intended to show the recoverable error with match expression shortcuts(unwrap() and expect()).

fn main()
{
    File::open("new_file.txt").unwrap();

    //File::open("new_file.txt").expect("Unable to open file");
}

