// 03-ownership: Rust's unique ownership system
// Learn about ownership, borrowing, references, and lifetimes

fn main() {
    println!("=== 03: OWNERSHIP & BORROWING ===\n");
    
    // 1. Ownership basics
    ownership_basics();
    
    // 2. Move semantics
    move_semantics();
    
    // 3. Clone
    clone_demo();
    
    // 4. References and Borrowing
    references_and_borrowing();
    
    // 5. Mutable references
    mutable_references();
    
    // 6. Slices
    slices_demo();
}

// Ownership Basics
fn ownership_basics() {
    println!("--- Ownership Basics ---");
    println!("Each value has an owner");
    println!("Only one owner at a time");
    println!("Value is dropped when owner goes out of scope\n");
    
    {
        let s = String::from("hello");  // s owns the string
        println!("s in scope: {}", s);
    } // s goes out of scope and is dropped
    
    // println!("{}", s);  // ERROR: s is no longer in scope
    println!();
}

// Move Semantics
fn move_semantics() {
    println!("--- Move Semantics ---");
    
    let s1 = String::from("hello");
    println!("s1: {}", s1);
    
    let s2 = s1;  // s1 is moved to s2
    println!("s2: {}", s2);
    // println!("{}", s1);  // ERROR: s1 is no longer valid
    
    // For simple types (Copy trait), values are copied
    let x = 5;
    let y = x;  // x is copied to y
    println!("x: {}, y: {} (both valid, integers implement Copy)\n", x, y);
}

// Clone - explicit deep copy
fn clone_demo() {
    println!("--- Clone ---");
    
    let s1 = String::from("hello");
    let s2 = s1.clone();  // Explicit deep copy
    
    println!("s1: {}, s2: {} (both valid after clone)\n", s1, s2);
}

// References and Borrowing (immutable)
fn references_and_borrowing() {
    println!("--- References and Borrowing ---");
    
    let s1 = String::from("hello");
    
    let len = calculate_length(&s1);  // Borrow s1
    
    println!("The length of '{}' is {}.", s1, len);
    println!("s1 is still valid after borrowing\n");
}

fn calculate_length(s: &String) -> usize {
    s.len()
}  // s goes out of scope but doesn't drop the data (it doesn't own it)

// Mutable References
fn mutable_references() {
    println!("--- Mutable References ---");
    
    let mut s = String::from("hello");
    println!("Before: {}", s);
    
    change(&mut s);  // Mutable borrow
    
    println!("After: {}", s);
    
    // Rules:
    // 1. Can have ONE mutable reference OR multiple immutable references
    // 2. Cannot have mutable and immutable references at the same time
    
    let r1 = &s;     // OK
    let r2 = &s;     // OK
    println!("r1: {}, r2: {}", r1, r2);
    // let r3 = &mut s;  // ERROR: cannot borrow as mutable
    
    println!();
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

// Slices - reference a portion of a collection
fn slices_demo() {
    println!("--- Slices ---");
    
    let s = String::from("hello world");
    
    let hello = &s[0..5];   // or &s[..5]
    let world = &s[6..11];  // or &s[6..]
    let full = &s[..];      // entire string
    
    println!("Original: {}", s);
    println!("First word: {}", hello);
    println!("Second word: {}", world);
    println!("Full slice: {}", full);
    
    // Array slices
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
    println!("Array slice: {:?}", slice);
    
    // First word function
    let sentence = String::from("Hello Rust World");
    let word = first_word(&sentence);
    println!("First word of '{}': {}\n", sentence, word);
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    
    &s[..]
}
