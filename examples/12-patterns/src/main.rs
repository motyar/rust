// 12-patterns: Common Rust patterns and idioms
// Learn about iterators, closures, smart pointers, and more

use std::rc::Rc;
use std::cell::RefCell;

fn main() {
    println!("=== 12: COMMON PATTERNS ===\n");
    
    // 1. Closures
    closures_demo();
    
    // 2. Iterators
    iterators_demo();
    
    // 3. Smart Pointers - Box
    box_demo();
    
    // 4. Smart Pointers - Rc (Reference Counting)
    rc_demo();
    
    // 5. Smart Pointers - RefCell
    refcell_demo();
    
    // 6. Pattern Matching
    pattern_matching_demo();
    
    // 7. If let and While let
    if_let_demo();
}

// Closures - anonymous functions
fn closures_demo() {
    println!("--- Closures ---");
    
    // Basic closure
    let add_one = |x: i32| -> i32 { x + 1 };
    println!("5 + 1 = {}", add_one(5));
    
    // Closure with type inference
    let multiply = |x, y| x * y;
    println!("3 * 4 = {}", multiply(3, 4));
    
    // Closure capturing environment
    let x = 4;
    let equal_to_x = |z| z == x;
    let y = 4;
    println!("Is {} equal to {}? {}", y, x, equal_to_x(y));
    
    // Using closures with functions
    let numbers = vec![1, 2, 3, 4, 5];
    let squared: Vec<i32> = numbers.iter().map(|x| x * x).collect();
    println!("Squared: {:?}\n", squared);
}

// Iterators
fn iterators_demo() {
    println!("--- Iterators ---");
    
    let v = vec![1, 2, 3];
    
    // Iterating
    print!("Values: ");
    for val in v.iter() {
        print!("{} ", val);
    }
    println!();
    
    // Iterator adaptors
    let v2: Vec<i32> = v.iter().map(|x| x + 1).collect();
    println!("Plus one: {:?}", v2);
    
    // Filter
    let evens: Vec<i32> = v.iter().filter(|x| *x % 2 == 0).copied().collect();
    println!("Even numbers: {:?}", evens);
    
    // Sum
    let sum: i32 = v.iter().sum();
    println!("Sum: {}", sum);
    
    // Chain iterators
    let v3 = vec![4, 5, 6];
    let combined: Vec<i32> = v.iter().chain(v3.iter()).copied().collect();
    println!("Combined: {:?}", combined);
    
    // Zip
    let names = vec!["Alice", "Bob", "Carol"];
    let scores = vec![95, 87, 92];
    let results: Vec<_> = names.iter().zip(scores.iter()).collect();
    println!("Results: {:?}\n", results);
}

// Box - heap allocation
fn box_demo() {
    println!("--- Box<T> ---");
    
    let b = Box::new(5);
    println!("Boxed value: {}", b);
    
    // Recursive type (impossible without Box)
    #[derive(Debug)]
    enum List {
        Cons(i32, Box<List>),
        Nil,
    }
    
    use List::{Cons, Nil};
    
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    println!("Linked list: {:?}\n", list);
}

// Rc - Reference Counting
fn rc_demo() {
    println!("--- Rc<T> ---");
    
    let a = Rc::new(5);
    println!("Count after creating a: {}", Rc::strong_count(&a));
    
    let _b = Rc::clone(&a);
    println!("Count after creating b: {}", Rc::strong_count(&a));
    
    {
        let _c = Rc::clone(&a);
        println!("Count after creating c: {}", Rc::strong_count(&a));
    }
    
    println!("Count after c goes out of scope: {}\n", Rc::strong_count(&a));
}

// RefCell - Interior mutability
fn refcell_demo() {
    println!("--- RefCell<T> ---");
    
    let value = RefCell::new(5);
    
    println!("Value: {:?}", value);
    
    // Borrow mutably
    *value.borrow_mut() += 10;
    
    println!("After mutation: {:?}\n", value);
}

// Pattern Matching
fn pattern_matching_demo() {
    println!("--- Pattern Matching ---");
    
    // Match literals
    let x = 1;
    match x {
        1 => println!("one"),
        2 => println!("two"),
        _ => println!("anything"),
    }
    
    // Match multiple patterns
    match x {
        1 | 2 => println!("one or two"),
        _ => println!("anything"),
    }
    
    // Match ranges
    let y = 5;
    match y {
        1..=5 => println!("one through five"),
        _ => println!("something else"),
    }
    
    // Destructuring structs
    struct Point {
        x: i32,
        y: i32,
    }
    
    let p = Point { x: 0, y: 7 };
    let Point { x, y } = p;
    println!("Point: ({}, {})", x, y);
    
    // Destructuring enums
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
    }
    
    let msg = Message::Move { x: 10, y: 20 };
    
    match msg {
        Message::Quit => println!("Quit"),
        Message::Move { x, y } => println!("Move to ({}, {})", x, y),
        Message::Write(text) => println!("Write: {}", text),
    }
    
    println!();
}

// If let and While let
fn if_let_demo() {
    println!("--- If Let / While Let ---");
    
    // If let - concise match for one pattern
    let some_value = Some(3);
    
    if let Some(3) = some_value {
        println!("Got three!");
    }
    
    // While let - loop while pattern matches
    let mut stack = Vec::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);
    
    print!("Popping: ");
    while let Some(top) = stack.pop() {
        print!("{} ", top);
    }
    println!("\n");
}

// Additional patterns:
// - Turbofish: ::<T> for specifying generic types
// - RAII: Resource Acquisition Is Initialization
// - Newtype pattern: struct Wrapper(Type)
// - Type aliases: type Kilometers = i32
// - Never type: ! for functions that never return
// - Function pointers: fn(i32) -> i32
