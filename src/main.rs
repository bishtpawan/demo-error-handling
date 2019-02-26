///This program is intended to show the unrecoverable error.
fn main() {
    display_unrecoverable_error();
}
fn display_unrecoverable_error(){
    let count: Vec<i32> = vec![1,2,3];
    print!("element of a vector is {}:",count[3]);
}


#[cfg(test)]
mod test {
    use crate::display_unrecoverable_error;

    # [test]
    # [should_panic]
    fn test_display_unrecoverable_error(){
        display_unrecoverable_error();
    }
}