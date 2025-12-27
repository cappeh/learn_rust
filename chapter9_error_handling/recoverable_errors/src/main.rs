use std::{fs::{self, File}, io::{self, ErrorKind, Read}};

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

    let _closure_greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("problem creating the file: {error:?}");
            })
        } else {
            panic!("problem opening the file: {error:?}");
        }
    });

    // if successful the value contained in Ok will be returned
    // if an error occurs, the panic! macro is called with panic message provided by the Err value
    let _greeting_file_unwrap = File::open("hello.txt").unwrap();

    // if successful the value contained in Ok will be returned
    // if an error occurs panic! is called with the provided error message and message contained
    // in the Err value
    let _greeting_file_expect = File::open("hello.txt").expect("hello.txt not in project");

}

#[allow(dead_code)]
// function will return Result<String, io::Error> T is String, E is io::Error
// if the function succeeds without issues, the code that calls this function gets Ok(String)
// containing the username
fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e), // return early out of the function returning the error "e"
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e), // dont need to explicitly use "return" as this is the last expression
    }
}

#[allow(dead_code)]
fn read_username_from_file2() -> Result<String, io::Error> {
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}

#[allow(dead_code)]
fn read_username_from_file3() -> Result<String, io::Error> {
    let mut username = String::new();
    File::open("hello.txt")?.read_to_string(&mut username)?;
    Ok(username)
}

#[allow(dead_code)]
fn read_username_from_file4() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}
