// 02-control-flow: Conditional statements and loops
// Learn about if/else, loops, while, for, match

fn main() {
    println!("=== 02: CONTROL FLOW ===\n");
    
    // 1. If/Else statements
    if_else_demo();
    
    // 2. Loop (infinite)
    loop_demo();
    
    // 3. While loop
    while_demo();
    
    // 4. For loop
    for_demo();
    
    // 5. Match (like switch, but better!)
    match_demo();
}

// If/Else
fn if_else_demo() {
    println!("--- If/Else Statements ---");
    
    let number = 7;
    
    if number < 5 {
        println!("{} is less than 5", number);
    } else if number == 5 {
        println!("{} equals 5", number);
    } else {
        println!("{} is greater than 5", number);
    }
    
    // if is an expression - can assign to variable
    let condition = true;
    let result = if condition { 5 } else { 6 };
    println!("Result from if expression: {}\n", result);
}

// Loop - infinite loop with break
fn loop_demo() {
    println!("--- Loop (infinite with break) ---");
    
    let mut counter = 0;
    
    let result = loop {
        counter += 1;
        
        if counter == 10 {
            break counter * 2;  // Can return value from loop
        }
    };
    
    println!("Loop result: {}\n", result);
}

// While loop
fn while_demo() {
    println!("--- While Loop ---");
    
    let mut number = 3;
    
    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }
    
    println!("LIFTOFF!!!\n");
}

// For loop
fn for_demo() {
    println!("--- For Loop ---");
    
    // Iterate over array
    let array = [10, 20, 30, 40, 50];
    println!("Iterating over array:");
    for element in array {
        println!("  Value: {}", element);
    }
    
    // Iterate over range
    println!("Countdown using range:");
    for number in (1..4).rev() {
        println!("  {}!", number);
    }
    println!("  GO!\n");
}

// Match - pattern matching
fn match_demo() {
    println!("--- Match Statement ---");
    
    // Match with numbers
    let number = 3;
    match number {
        1 => println!("One!"),
        2 => println!("Two!"),
        3 => println!("Three!"),
        4 | 5 => println!("Four or Five!"),  // Multiple patterns
        _ => println!("Something else!"),    // Default case
    }
    
    // Match with ranges
    let age = 18;
    match age {
        0..=12 => println!("Child"),
        13..=19 => println!("Teenager"),
        20..=59 => println!("Adult"),
        _ => println!("Senior"),
    }
    
    // Match as expression
    let boolean = true;
    let binary = match boolean {
        true => 1,
        false => 0,
    };
    println!("Boolean {} as binary: {}\n", boolean, binary);
}
