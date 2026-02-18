// 04-structs-enums: Custom data types
// Learn about structs, enums, methods, and associated functions

fn main() {
    println!("=== 04: STRUCTS & ENUMS ===\n");
    
    // 1. Structs
    structs_demo();
    
    // 2. Tuple Structs
    tuple_structs_demo();
    
    // 3. Methods
    methods_demo();
    
    // 4. Enums
    enums_demo();
    
    // 5. Option enum
    option_demo();
    
    // 6. Match with enums
    match_enum_demo();
}

// Regular Structs
#[derive(Debug)]  // Allows printing with {:?}
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn structs_demo() {
    println!("--- Structs ---");
    
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someuser"),
        active: true,
        sign_in_count: 1,
    };
    
    println!("User: {:?}", user1);
    println!("Username: {}", user1.username);
    
    // Mutable struct
    let mut user2 = User {
        email: String::from("another@example.com"),
        username: String::from("anotheruser"),
        active: true,
        sign_in_count: 1,
    };
    
    user2.email = String::from("newemail@example.com");
    println!("Updated email: {}", user2.email);
    
    // Struct update syntax
    let user3 = User {
        email: String::from("third@example.com"),
        ..user2  // Use remaining fields from user2
    };
    
    println!("User3: {:?}\n", user3);
}

// Tuple Structs
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn tuple_structs_demo() {
    println!("--- Tuple Structs ---");
    
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    
    println!("Black color: ({}, {}, {})", black.0, black.1, black.2);
    println!("Origin point: ({}, {}, {})\n", origin.0, origin.1, origin.2);
}

// Methods and Associated Functions
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // Associated function (like static method)
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
    
    // Method (takes &self)
    fn area(&self) -> u32 {
        self.width * self.height
    }
    
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn methods_demo() {
    println!("--- Methods ---");
    
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    
    let square = Rectangle::square(25);  // Associated function
    
    println!("rect1: {:?}", rect1);
    println!("rect1 area: {}", rect1.area());  // Method
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Square: {:?}\n", square);
}

// Enums
#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

#[derive(Debug)]
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

fn enums_demo() {
    println!("--- Enums ---");
    
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
    
    println!("IP kinds: {:?}, {:?}", four, six);
    
    // Enums with data
    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));
    
    println!("Home: {:?}", home);
    println!("Loopback: {:?}\n", loopback);
}

// Option enum (built into Rust)
fn option_demo() {
    println!("--- Option Enum ---");
    
    let some_number = Some(5);
    let some_string = Some("a string");
    let absent_number: Option<i32> = None;
    
    println!("Some number: {:?}", some_number);
    println!("Some string: {:?}", some_string);
    println!("Absent number: {:?}\n", absent_number);
}

// Match with Enums
#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    California,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

fn match_enum_demo() {
    println!("--- Match with Enums ---");
    
    let penny = Coin::Penny;
    let quarter = Coin::Quarter(UsState::California);
    
    println!("Penny value: {} cents", value_in_cents(penny));
    println!("Quarter value: {} cents", value_in_cents(quarter));
    
    // Match with Option
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    
    println!("Five plus one: {:?}", six);
    println!("None plus one: {:?}\n", none);
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}
