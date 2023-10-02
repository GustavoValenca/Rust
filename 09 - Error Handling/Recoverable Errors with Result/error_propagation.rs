use std::fs;
use std::fs::File;
use std::io::{self, Read};

fn read_username_from_file() -> Result<String, io::Error> {
    // let username_file_result = File::open("hello.txt");

    // let mut username_file = match username_file_result {
    //     Ok(file) => file,
    //     Err(e) => return Err(e),
    // };

    // let mut username = String::new();

    // match username_file.read_to_string(&mut username){  
    //     // the read_to_string function reads the contents of the file into username
    //     Ok(_) => Ok(username),
    //     Err(e) => Err(e),
    // };

    // Using the ? operator

    // let mut username_file = File::open("hello.txt")?;
    // let mut username = String::new();
    // username_file.read_to_string(&mut username)?;
    // Ok(username);

    // Shortening even more

    // let mut username = String::new();
    // File::open("hello.txt")?.read_to_string(&mut username)?;
    // Ok(username);

    // SHORTEST

    fs::read_to_string("hello.txt")
}