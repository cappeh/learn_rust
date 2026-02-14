use std::{fs::File, io::{self, Read}};
use std::fmt;

type FileOpResult<T> = Result<T, FileOpErr>;

#[derive(Debug)]
struct FileOpErr(io::Error);

impl fmt::Display for FileOpErr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Operation Failed With Error: {}", self.0)
    }
}

impl From<io::Error> for FileOpErr {
    fn from(value: io::Error) -> Self {
        FileOpErr(value)
    }
}

impl std::error::Error for FileOpErr {}

fn run() -> FileOpResult<()> {
    let user = read_username_from_file()?;
    println!("{user}");
    Ok(())
}

fn main() {
    if let Err(e) = run() {
        eprintln!("{}", e);
    }
}

fn read_username_from_file() -> FileOpResult<String> {
    let mut username = String::new();
    File::open("hello.txt")?.read_to_string(&mut username)?;
    Ok(username)
}
