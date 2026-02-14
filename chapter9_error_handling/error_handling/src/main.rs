use std::{error, fs::{self}, io};

type FileOpResult<T> = Result<T, Box<dyn error::Error>>;

fn main() -> FileOpResult<()> {
    let user = read_username_from_file()?;
    println!("{user}");
    Ok(())
}

fn read_username_from_file() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}
