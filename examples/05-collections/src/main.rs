// 05-collections: Common Rust collections
// Learn about Vectors, Strings, and HashMaps

use std::collections::HashMap;

fn main() {
    println!("=== 05: COLLECTIONS ===\n");
    
    // 1. Vectors
    vectors_demo();
    
    // 2. Strings
    strings_demo();
    
    // 3. HashMaps
    hashmaps_demo();
}

// Vectors - growable arrays
fn vectors_demo() {
    println!("--- Vectors ---");
    
    // Create vector
    let mut v: Vec<i32> = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);
    
    println!("Vector: {:?}", v);
    
    // Using vec! macro
    let v2 = vec![1, 2, 3, 4, 5];
    println!("Vector from macro: {:?}", v2);
    
    // Accessing elements
    let third = &v2[2];
    println!("Third element: {}", third);
    
    // Safe access with get
    match v2.get(2) {
        Some(third) => println!("Third element (safe): {}", third),
        None => println!("No third element"),
    }
    
    // Iterating
    print!("Iterating: ");
    for i in &v2 {
        print!("{} ", i);
    }
    println!();
    
    // Mutable iteration
    let mut v3 = vec![1, 2, 3];
    for i in &mut v3 {
        *i += 10;
    }
    println!("After mutation: {:?}", v3);
    
    // Vectors with enums (multiple types)
    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }
    
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Float(10.12),
        SpreadsheetCell::Text(String::from("blue")),
    ];
    
    println!("Spreadsheet row: {:?}\n", row);
}

// Strings
fn strings_demo() {
    println!("--- Strings ---");
    
    // Creating strings
    let mut s = String::new();
    s.push_str("Hello");
    s.push(' ');
    s.push_str("World");
    println!("Built string: {}", s);
    
    let s1 = String::from("Hello");
    let s2 = String::from(" World");
    let s3 = s1 + &s2;  // s1 is moved here
    println!("Concatenated: {}", s3);
    
    // format! macro (doesn't take ownership)
    let s4 = String::from("Hello");
    let s5 = String::from("World");
    let s6 = format!("{} {}", s4, s5);
    println!("Formatted: {} (s4 and s5 still valid)", s6);
    
    // String slices
    let hello = "Здравствуйте";
    let s = &hello[0..4];  // First 4 bytes (not characters!)
    println!("Slice: {}", s);
    
    // Iterating over strings
    println!("By chars:");
    for c in "नमस्ते".chars() {
        print!("{} ", c);
    }
    println!();
    
    println!("By bytes:");
    for b in "नमस्ते".bytes() {
        print!("{} ", b);
    }
    println!("\n");
}

// HashMaps
fn hashmaps_demo() {
    println!("--- HashMaps ---");
    
    // Creating a HashMap
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    
    println!("Scores: {:?}", scores);
    
    // Accessing values
    let team_name = String::from("Blue");
    let score = scores.get(&team_name);
    match score {
        Some(s) => println!("Blue team score: {}", s),
        None => println!("Team not found"),
    }
    
    // Iterating
    println!("All scores:");
    for (key, value) in &scores {
        println!("  {}: {}", key, value);
    }
    
    // Updating values
    scores.insert(String::from("Blue"), 25);  // Overwrite
    println!("After update: {:?}", scores);
    
    // Only insert if key doesn't exist
    scores.entry(String::from("Red")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(100);  // Won't change
    println!("After conditional insert: {:?}", scores);
    
    // Update based on old value
    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    
    println!("Word count: {:?}\n", map);
}
