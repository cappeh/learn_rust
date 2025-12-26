use std::{fs::File, io::ErrorKind};

fn main() {
    let greeting_file_result = File::open("hello.txt");
    
    let _greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("problem creating the file: {e:?}"),
            },
            _ => {
                panic!("problem opening the file: {error:?}");
            }
        }
    };

}
