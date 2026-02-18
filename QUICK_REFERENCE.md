# Quick Reference Guide

This is a quick reference for common Rust commands and patterns you'll use while learning.

## Cargo Commands

```bash
# Run a specific example
cargo run -p basics
cargo run -p ownership

# Build all examples
cargo build --all

# Build specific example
cargo build -p basics

# Check code (faster than build)
cargo check --all

# Run tests
cargo test --all
cargo test -p testing

# Clean build artifacts
cargo clean

# Update dependencies
cargo update

# Format code
cargo fmt

# Lint code
cargo clippy
```

## Running Examples

```bash
# Basic examples
cargo run -p basics
cargo run -p control_flow
cargo run -p ownership
cargo run -p structs_enums
cargo run -p collections

# Error handling
cargo run -p error_handling

# Advanced topics
cargo run -p generics_traits
cargo run -p modules
cargo run -p testing
cargo run -p file_io
cargo run -p concurrency
cargo run -p patterns
```

## Common Rust Patterns

### Variable Declaration
```rust
let x = 5;                    // Immutable
let mut y = 10;               // Mutable
const MAX: u32 = 100;         // Constant
```

### Data Types
```rust
let a: i32 = 42;              // 32-bit integer
let b: f64 = 3.14;            // 64-bit float
let c: bool = true;           // Boolean
let d: char = '🦀';           // Unicode character
let e = (1, 2.5, 'x');        // Tuple
let f = [1, 2, 3, 4, 5];      // Array
```

### Functions
```rust
fn add(a: i32, b: i32) -> i32 {
    a + b  // Returns without semicolon
}
```

### Control Flow
```rust
// If/else
if x > 5 {
    println!("big");
} else {
    println!("small");
}

// Loop
loop {
    break;
}

// While
while n != 0 {
    n -= 1;
}

// For
for i in 0..10 {
    println!("{}", i);
}

// Match
match number {
    1 => println!("One"),
    2 => println!("Two"),
    _ => println!("Other"),
}
```

### Ownership
```rust
let s1 = String::from("hello");
let s2 = s1;          // s1 moved to s2
let s3 = s2.clone();  // Deep copy

// References
let len = calculate_length(&s2);  // Borrow

// Mutable reference
let mut s = String::from("hello");
change(&mut s);
```

### Structs
```rust
struct User {
    username: String,
    email: String,
}

let user = User {
    username: String::from("john"),
    email: String::from("john@example.com"),
};

// Methods
impl User {
    fn new(username: String, email: String) -> User {
        User { username, email }
    }
    
    fn display(&self) {
        println!("{}: {}", self.username, self.email);
    }
}
```

### Enums
```rust
enum Status {
    Active,
    Inactive,
}

// With data
enum Message {
    Text(String),
    Number(i32),
}

// Option
let some: Option<i32> = Some(5);
let none: Option<i32> = None;

// Result
let ok: Result<i32, String> = Ok(42);
let err: Result<i32, String> = Err(String::from("error"));
```

### Collections
```rust
// Vector
let mut v = Vec::new();
v.push(1);
let v2 = vec![1, 2, 3];

// HashMap
use std::collections::HashMap;
let mut map = HashMap::new();
map.insert("key", "value");

// String
let mut s = String::from("hello");
s.push_str(", world");
```

### Error Handling
```rust
// Result
match File::open("file.txt") {
    Ok(file) => { /* use file */ },
    Err(error) => { /* handle error */ },
}

// ? operator
fn read_file() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("file.txt")?.read_to_string(&mut s)?;
    Ok(s)
}

// unwrap (panics on error)
let f = File::open("file.txt").unwrap();

// expect (panics with message)
let f = File::open("file.txt").expect("Failed to open file");
```

### Iterators
```rust
let v = vec![1, 2, 3];

// Map
let doubled: Vec<i32> = v.iter().map(|x| x * 2).collect();

// Filter
let evens: Vec<i32> = v.iter().filter(|x| *x % 2 == 0).copied().collect();

// Sum
let sum: i32 = v.iter().sum();

// For each
v.iter().for_each(|x| println!("{}", x));
```

### Closures
```rust
let add_one = |x| x + 1;
let result = add_one(5);

// With explicit types
let multiply = |x: i32, y: i32| -> i32 { x * y };
```

### Traits
```rust
trait Summary {
    fn summarize(&self) -> String;
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}", self.headline)
    }
}
```

## Common Cargo.toml Patterns

```toml
[package]
name = "my_app"
version = "0.1.0"
edition = "2021"

[dependencies]
serde = "1.0"
tokio = { version = "1", features = ["full"] }
```

## Useful Attributes

```rust
#[derive(Debug)]              // Auto-implement Debug trait
#[derive(Clone, Copy)]        // Auto-implement Clone and Copy
#[allow(dead_code)]           // Suppress unused code warning
#[cfg(test)]                  // Only compile for tests
#[test]                       // Mark function as test
```

## Getting Help

```bash
# Get help on a command
cargo help run

# Check documentation for a function
rustup doc

# Open std library docs
rustup doc --std

# Format: https://doc.rust-lang.org/std/
```

## Common Errors and Solutions

### "value borrowed after move"
**Solution**: Either clone the value or use references

### "cannot borrow as mutable more than once"
**Solution**: Ensure only one mutable reference exists at a time

### "lifetime may not live long enough"
**Solution**: Add lifetime annotations or restructure code

### "trait X is not implemented"
**Solution**: Implement the trait or use a type that has it

## Tips

1. **Read compiler errors carefully** - Rust's error messages are very helpful
2. **Use `cargo clippy`** - Get additional linting suggestions
3. **Use `cargo fmt`** - Keep code formatted consistently
4. **Start small** - Run examples, modify them, break them, fix them
5. **Practice ownership** - It's the hardest concept but most important
6. **Use the playground** - https://play.rust-lang.org/ for quick experiments
