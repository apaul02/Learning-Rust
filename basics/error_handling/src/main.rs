use std::fs::{self, File};
use std::io::{self, ErrorKind, Read};

fn main() {
    let gretting_file_result = File::open("h.txt").expect("The file isnt here");
}

fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("username.txt");
    
    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}

fn read_username_from_file2() -> Result<String, io::Error> {
    let mut username = String::new();

    File::open("hello.txt")?.read_to_string(&mut username)?;
    Ok(username)
}

fn read_username_from_file3() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}

