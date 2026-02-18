// 10-file-io: File and I/O operations
// Learn about reading/writing files, command line args, and stdin

use std::env;
use std::fs;
use std::io::{self, Write, BufRead, BufReader};

fn main() {
    println!("=== 10: FILE I/O ===\n");
    
    // 1. Command line arguments
    command_line_args_demo();
    
    // 2. Writing to files
    writing_files_demo();
    
    // 3. Reading from files
    reading_files_demo();
    
    // 4. Appending to files
    appending_files_demo();
    
    // 5. Reading line by line
    reading_lines_demo();
    
    // 6. Standard input
    stdin_demo();
}

fn command_line_args_demo() {
    println!("--- Command Line Arguments ---");
    
    let args: Vec<String> = env::args().collect();
    println!("Program name: {}", args[0]);
    
    if args.len() > 1 {
        println!("Arguments:");
        for (i, arg) in args.iter().enumerate().skip(1) {
            println!("  {}: {}", i, arg);
        }
    } else {
        println!("No arguments provided");
        println!("Try: cargo run -p file_io -- arg1 arg2 arg3");
    }
    
    println!();
}

fn writing_files_demo() {
    println!("--- Writing to Files ---");
    
    let content = "Hello, Rust!\nThis is a test file.\nWritten from Rust.";
    
    match fs::write("/tmp/rust-test.txt", content) {
        Ok(_) => println!("Successfully wrote to /tmp/rust-test.txt"),
        Err(e) => println!("Error writing file: {}", e),
    }
    
    println!();
}

fn reading_files_demo() {
    println!("--- Reading from Files ---");
    
    match fs::read_to_string("/tmp/rust-test.txt") {
        Ok(contents) => {
            println!("File contents:");
            println!("{}", contents);
        }
        Err(e) => println!("Error reading file: {}", e),
    }
    
    println!();
}

fn appending_files_demo() {
    println!("--- Appending to Files ---");
    
    let mut file = match fs::OpenOptions::new()
        .append(true)
        .create(true)
        .open("/tmp/rust-test.txt")
    {
        Ok(file) => file,
        Err(e) => {
            println!("Error opening file: {}", e);
            return;
        }
    };
    
    match writeln!(file, "Appended line!") {
        Ok(_) => println!("Successfully appended to file"),
        Err(e) => println!("Error appending: {}", e),
    }
    
    println!();
}

fn reading_lines_demo() {
    println!("--- Reading Lines ---");
    
    let file = match fs::File::open("/tmp/rust-test.txt") {
        Ok(file) => file,
        Err(e) => {
            println!("Error opening file: {}", e);
            return;
        }
    };
    
    let reader = BufReader::new(file);
    
    println!("Reading file line by line:");
    for (i, line) in reader.lines().enumerate() {
        match line {
            Ok(content) => println!("  Line {}: {}", i + 1, content),
            Err(e) => println!("  Error reading line: {}", e),
        }
    }
    
    println!();
}

fn stdin_demo() {
    println!("--- Standard Input ---");
    println!("Type something and press Enter (or Ctrl+D to skip):");
    
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    let mut line = String::new();
    
    match handle.read_line(&mut line) {
        Ok(n) => {
            if n > 0 {
                println!("You typed: {}", line.trim());
            } else {
                println!("No input provided (EOF)");
            }
        }
        Err(e) => println!("Error reading stdin: {}", e),
    }
    
    println!();
}

// Additional file operations:
// - fs::remove_file() - delete a file
// - fs::rename() - rename/move a file
// - fs::metadata() - get file metadata
// - fs::create_dir() - create a directory
// - fs::read_dir() - list directory contents
