// 01-basics: Fundamental Rust concepts
// Learn about variables, data types, functions, and basic syntax

fn main() {
    println!("=== 01: RUST BASICS ===\n");
    
    // 1. Variables and Mutability
    variables_and_mutability();
    
    // 2. Data Types
    data_types();
    
    // 3. Functions
    functions_demo();
    
    // 4. Comments
    comments_demo();
    
    // 5. Constants
    constants_demo();
}

// Variables and Mutability
fn variables_and_mutability() {
    println!("--- Variables and Mutability ---");
    
    // Immutable variable (default)
    let x = 5;
    println!("Immutable x: {}", x);
    
    // Mutable variable
    let mut y = 10;
    println!("Mutable y before: {}", y);
    y = 15;
    println!("Mutable y after: {}", y);
    
    // Shadowing - can change type
    let spaces = "   ";
    println!("Spaces as string: '{}'", spaces);
    let spaces = spaces.len();
    println!("Spaces as number: {}\n", spaces);
}

// Data Types
fn data_types() {
    println!("--- Data Types ---");
    
    // Integer types
    let a: i32 = 42;  // 32-bit signed integer
    let b: u32 = 42;  // 32-bit unsigned integer
    println!("Signed i32: {}, Unsigned u32: {}", a, b);
    
    // Floating point
    let f1: f64 = 3.14159;  // 64-bit float
    let f2: f32 = 2.718;    // 32-bit float
    println!("Float f64: {}, Float f32: {}", f1, f2);
    
    // Boolean
    let is_rust_cool: bool = true;
    println!("Is Rust cool? {}", is_rust_cool);
    
    // Character (Unicode!)
    let letter: char = 'A';
    let emoji: char = '🦀';  // Rust mascot!
    println!("Char: {}, Emoji: {}", letter, emoji);
    
    // Tuple - different types
    let tuple: (i32, f64, char) = (500, 6.4, 'x');
    println!("Tuple: ({}, {}, {})", tuple.0, tuple.1, tuple.2);
    
    // Array - same type, fixed size
    let array: [i32; 5] = [1, 2, 3, 4, 5];
    println!("Array first element: {}", array[0]);
    println!("Array length: {}\n", array.len());
}

// Functions
fn functions_demo() {
    println!("--- Functions ---");
    
    let result = add(5, 3);
    println!("5 + 3 = {}", result);
    
    let result = multiply(4, 7);
    println!("4 * 7 = {}", result);
    
    // Expression vs Statement
    let y = {
        let x = 3;
        x + 1  // No semicolon - this is an expression that returns a value
    };
    println!("Expression result: {}\n", y);
}

fn add(a: i32, b: i32) -> i32 {
    a + b  // Last expression is returned (no semicolon)
}

fn multiply(x: i32, y: i32) -> i32 {
    return x * y;  // Can also use explicit return
}

// Comments
fn comments_demo() {
    println!("--- Comments ---");
    
    // This is a single-line comment
    
    /*
     * This is a multi-line comment
     * It can span multiple lines
     */
    
    /// This is a documentation comment (for functions, structs, etc.)
    /// It supports Markdown!
    
    println!("Comments don't affect program execution\n");
}

// Constants
const MAX_POINTS: u32 = 100_000;  // Constants are always immutable

fn constants_demo() {
    println!("--- Constants ---");
    println!("MAX_POINTS constant: {}", MAX_POINTS);
    println!("Constants are evaluated at compile time\n");
}
