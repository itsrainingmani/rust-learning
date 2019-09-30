use std::fs::File;
use std::io::ErrorKind;
use std::io::{self, Read};

fn main() {
    println!("Hello, world!");

    // Unrecoverable errors with panic!
    // panic!("crash and burn");

    // let v = vec![1, 2, 3];
    // v[99];

    // Recoverable Errors with Result
    // let f = File::open("hello.txt");

    // let f = match f {
    //     Ok(file) => file,
    //     Err(error) => match error.kind() {
    //         ErrorKind::NotFound => match File::create("hello.txt") {
    //             Ok(fc) => fc,
    //             Err(e) => panic!("Problem creating the file: {:?}", e),
    //         },
    //         other_error => panic!("Problem opening the file: {:?}", other_error),
    //     },
    // };

    // Using Closures - helps avoid multiple nested match expressions
    let f = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problemt creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:#?}", error);
        }
    });
}

fn read_username_from_file() -> Result<String, io::Error> {
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

fn read_username_from_file_simplified() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

fn read_username_from_file_more_simplified() -> Result<String, io::Error> {
    // Using the ? operator to simplify error propagation
    // ? placed after a Result value, returns the value inside an Ok and continues program execution,
    // or Err is returned from the whole function so the error value gets propagated to the calling code

    // Error values that ? called on them go through the from function defined in the From trait
    // This is used to convert errors from one type to another
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}
