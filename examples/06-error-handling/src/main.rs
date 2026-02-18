// 06-error-handling: Rust error handling patterns
// Learn about panic!, Result, Option, and error propagation

use std::fs::File;
use std::io::{self, Read, ErrorKind};

fn main() {
    println!("=== 06: ERROR HANDLING ===\n");
    
    // 1. Panic - unrecoverable errors
    panic_demo();
    
    // 2. Result - recoverable errors
    result_demo();
    
    // 3. Matching on errors
    match_errors_demo();
    
    // 4. Shortcuts: unwrap and expect
    shortcuts_demo();
    
    // 5. Propagating errors
    propagating_errors_demo();
    
    // 6. The ? operator
    question_mark_demo();
}

// Panic - for unrecoverable errors
fn panic_demo() {
    println!("--- Panic (Unrecoverable Errors) ---");
    println!("Uncomment the line below to see a panic:");
    println!("// panic!(\"crash and burn\");");
    
    // This would crash the program:
    // panic!("crash and burn");
    
    // Accessing invalid index also panics:
    // let v = vec![1, 2, 3];
    // v[99];
    
    println!();
}

// Result enum
fn result_demo() {
    println!("--- Result Enum ---");
    
    let f = File::open("hello.txt");
    
    match f {
        Ok(file) => println!("File opened successfully: {:?}", file),
        Err(error) => println!("Problem opening file: {:?}", error),
    }
    
    println!();
}

// Matching on different errors
fn match_errors_demo() {
    println!("--- Matching on Errors ---");
    
    let f = File::open("hello.txt");
    
    let _f = match f {
        Ok(file) => {
            println!("File opened");
            file
        },
        Err(error) => match error.kind() {
            ErrorKind::NotFound => {
                println!("File not found, would create it here in real app");
                return;  // Exit early for this demo
            }
            other_error => {
                println!("Problem opening file: {:?}", other_error);
                return;
            }
        },
    };
    
    println!();
}

// Shortcuts: unwrap and expect
fn shortcuts_demo() {
    println!("--- Shortcuts: unwrap and expect ---");
    
    // unwrap: returns value or panics
    // let f = File::open("hello.txt").unwrap();  // Would panic if file doesn't exist
    
    // expect: like unwrap but with custom error message
    // let f = File::open("hello.txt")
    //     .expect("Failed to open hello.txt");  // Better panic message
    
    println!("unwrap and expect will panic on error");
    println!("Use them when you're certain the value is Ok\n");
}

// Propagating errors
fn propagating_errors_demo() {
    println!("--- Propagating Errors ---");
    
    match read_username_from_file() {
        Ok(username) => println!("Username: {}", username),
        Err(e) => println!("Error reading username: {}", e),
    }
    
    println!();
}

fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("username.txt");
    
    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),  // Propagate error to caller
    };
    
    let mut s = String::new();
    
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),  // Propagate error to caller
    }
}

// The ? operator - shortcut for propagating errors
fn question_mark_demo() {
    println!("--- The ? Operator ---");
    
    match read_username_with_question_mark() {
        Ok(username) => println!("Username: {}", username),
        Err(e) => println!("Error reading username: {}", e),
    }
    
    // Even shorter version
    match read_username_short() {
        Ok(username) => println!("Username (short): {}", username),
        Err(e) => println!("Error reading username: {}", e),
    }
    
    println!();
}

// Using ? operator
fn read_username_with_question_mark() -> Result<String, io::Error> {
    let mut f = File::open("username.txt")?;  // ? returns error if Err
    let mut s = String::new();
    f.read_to_string(&mut s)?;  // ? returns error if Err
    Ok(s)
}

// Even shorter with chaining
fn read_username_short() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("username.txt")?.read_to_string(&mut s)?;
    Ok(s)
}

// Custom error types
#[derive(Debug)]
enum MyError {
    ParseError,
    IoError(io::Error),
}

impl From<io::Error> for MyError {
    fn from(error: io::Error) -> Self {
        MyError::IoError(error)
    }
}

#[allow(dead_code)]
fn custom_error_example() -> Result<i32, MyError> {
    let mut s = String::new();
    File::open("number.txt")?.read_to_string(&mut s)?;  // ? works with From trait
    
    s.trim().parse()
        .map_err(|_| MyError::ParseError)  // Convert ParseIntError to MyError
}
