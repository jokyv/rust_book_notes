use std::{
    fs::{self, File},
    io::{self, ErrorKind, Read},
};

fn main() {
    // PANIC
    // when code panics there no way to recover
    // you want to panic on examples, prototype code and tests
    // two examples where our code will panic
    // panic!("crash and burn")
    // let v = vec![1, 2, 3];
    // v[99];

    // RESULT
    // give an option of what to do, error can be retriavable
    // ok_error();
    // error_with_unwrap();
    // error_with_expect();
    // error_with_questionmark();
    error_with_questionmark_shortest();
}

fn error_with_questionmark_shortest() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}

// function returns a string or an io Error
fn error_with_questionmark() -> Result<String, io::Error> {
    // create the mutable variable s as string
    let mut s = String::new();
    // if it is successul it will open hello.txt
    // if not will return an io Error
    // then read_to_string if it is successul will store it into s
    // if not will return an io Error
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}

fn _error_with_expect() {
    let f = File::open("hello.txt").expect("Failed to open hello.txt - MY MESSAGE");
}

fn _error_with_unwrap() {
    // unwrap will print the same error message so is difficult to find
    // from where the error message is coming from
    let f = File::open("hello.txt").unwrap();
}

fn _ok_error() {
    let f = File::open("hello.txt");

    // shadowing f
    let f = match f {
        // can be named file can be named file_file or anything you want
        Ok(file) => file,
        Err(error) => match error.kind() {
            // if you get file not found error create the file
            ErrorKind::NotFound => match File::create("hello1.txt") {
                Ok(file_file) => file_file,
                // handling the case of file could not be created for whatever the reason and panic
                Err(e) => panic!("Problem creating the file {:?}", e),
            },
            // here you are handling all other error types by just print a message and panic
            other_error => panic!("Problem opening the file {:?}", other_error),
        },
    };
}
