use std::fs::File;
use demo::utilities::constant::INVALID_FILE_PATH;


///This program is intended to show the recoverable error with match expression shortcuts(unwrap() and expect()).

fn main()
{
    get_file_from_path();
}

fn get_file_from_path(){
    File::open(INVALID_FILE_PATH).unwrap();

    //File::open(INVALID_FILE_PATH).expect("Unable to open file");
}
#[cfg(test)]
mod test {
    use std::fs::File;
    use demo::utilities::constant::INVALID_FILE_PATH;
    use crate::get_file_from_path;

    #[test]
    #[should_panic]
    fn test_shortcut_unwrap(){
       get_file_from_path();
    }

    #[test]
    #[should_panic]
    fn test_shortcut_expect(){
        File::open(INVALID_FILE_PATH).expect("Unable to open file");
    }

}
