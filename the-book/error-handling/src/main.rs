fn _unrecoverable_errors() {
    // We could panic with a macro
    panic!("I am panicking here");
    // We can see a backtrace by running with the
    // RUST_BACKTRACE=1 environment variable set
}

use std::fs::File;
use std::io::{Error, ErrorKind};
use std::io::Read;

// Example of many matches
fn _long_read_username_from_file() -> Result<String, Error> {
    // Let's open a file
    let mut file = match File::open("hi.txt") {
        Ok(file) => file,
        Err(error) => if error.kind() == ErrorKind::NotFound {
            match File::create("hi.txt") {
                Ok(fc) => fc,
                Err(e) => return Err(e),
            }
        }
        else {
            return Err(error);
        },
    };

    let mut username = String::new();
    match file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(error) => Err(error), // propagate the error
    }
}

// We can do better
fn read_username_from_file() -> Result<String, Error> {
    // The question mark operator will propagate the error for us
    let mut file = File::open("hi.txt")?;
    let mut username = String::new();
    file.read_to_string(&mut username)?;
    Ok(username)
}

fn recoverable_errors() {
 

    // Unwrap returns the value in Ok, otherwise panics with a defualt error message
    // let another_f = File::open("another.txt").unwrap();
    
    // This panics with a nice error message
    // let another_f = File::open("another.txt").expect("Cannot open another.txt");

    if let Ok(username) = read_username_from_file() {
        println!("Username {}", username);
    }
    else {
        println!("Could not read from file");
    }
}

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess should be between 1 and 100, got {}", value);
        }
        Guess { value }
    }
}

fn constraints() {
    // A type which should be between 1 and 100
    let g = Guess::new(1);
    println!("Guess is {}", g.value);
    //let g = Guess::new(0); // panics
}

fn main() {
    // _unrecoverable_error();
    recoverable_errors();
    constraints();
    println!("Hello, world!");
}
