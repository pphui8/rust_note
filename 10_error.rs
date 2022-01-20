use core::panic;
use std::{fs::{File, self}, io::{ErrorKind, self, Read}};
/*
Rust groups errors into two major categories: 
recoverable and unrecoverable errors

add to Cargo.toml
[profile.release]
panic = 'abort'
to abort executing without unwinding
*/

#[allow(unreachable_code, dead_code)]
fn simple_panic() { // unrecoverable errors
    panic!("crash and burn");
    // back_trace 
    {
        //  set the RUST_BACKTRACE environment variable to get a backtrace of
        //  exactly what happened to cause the error
    }
}

#[allow(unused, dead_code)]
fn recoveravle_result() {
    let f = File::open("hello.txt");
    // match defferent errors
    let f = match f {
        // take the inner file value out of result
        Ok(file) => file,
        Err(error) => match error.kind() {
            // create a new file and retuen the handler
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error)
            }
        },
    };
    // syntactic sugar
    {
        let f = File::open("hello.txt").unwrap_or_else(|error| {
            if error.kind() == ErrorKind::NotFound {
                File::create("hello.txt").unwrap_or_else(|error| {
                    panic!("Problem creating the file: {:?}", error);
                })
            } else {
                panic!("Problem opening the file: {:?}", error);
            }
        });
    }

    // shortcuts: unwrap and expect
    {
        // if value is Ok, return value inside Ok
        // else call panic!()
        let f = File::open("hello.txt").unwrap();
        // pass the message to panic!()
        let f = File::open("hello.txt").expect("Failed to open file");
    }
}

// propagating the error
// get value or err propagat to the calling code
#[allow(dead_code)]
fn propagating_error() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

// propagating shortcut(syntactic sugar)
// The ? operator can only be used in functions that have 
// a return type compatible with the value the ? is used on
#[allow(dead_code)]
fn propagating_shortcut() -> Result<String, io::Error> {
    // ? will call the function "From"  to convert errors
    // from one type into another (to return type)
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}
#[allow(dead_code)]
fn propagating_more() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}
#[allow(dead_code)]
fn propagating_much_more() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}
#[allow(dead_code)]
// last char of first line
fn another_way_to_use(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}


use std::error::Error;
#[allow(unused)]
// Err in main function
fn main() -> Result<(), Box<dyn Error>> {
    let f = File::open("hello.txt")?;
    // Ok() means execute successfully
    // Error could contain any type of err
    Ok(())
}