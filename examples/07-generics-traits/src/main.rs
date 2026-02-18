// 07-generics-traits: Generic programming and traits
// Learn about generic types, traits, and trait bounds

fn main() {
    println!("=== 07: GENERICS & TRAITS ===\n");
    
    // 1. Generic functions
    generic_functions_demo();
    
    // 2. Generic structs
    generic_structs_demo();
    
    // 3. Generic enums
    generic_enums_demo();
    
    // 4. Traits
    traits_demo();
    
    // 5. Trait bounds
    trait_bounds_demo();
    
    // 6. Default implementations
    default_impl_demo();
}

// Generic Functions
fn generic_functions_demo() {
    println!("--- Generic Functions ---");
    
    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest(&number_list);
    println!("The largest number is {}", result);
    
    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest(&char_list);
    println!("The largest char is {}\n", result);
}

fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    
    largest
}

// Generic Structs
#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

fn generic_structs_demo() {
    println!("--- Generic Structs ---");
    
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };
    
    println!("Integer point: {:?}", integer);
    println!("Float point: {:?}", float);
    println!("x coordinate: {}", integer.x());
    println!("Distance from origin: {}\n", float.distance_from_origin());
}

// Generic Enums (Option and Result are generic!)
fn generic_enums_demo() {
    println!("--- Generic Enums ---");
    
    let some_number: Option<i32> = Some(5);
    let some_string: Option<&str> = Some("hello");
    
    println!("Option<i32>: {:?}", some_number);
    println!("Option<&str>: {:?}", some_string);
    
    let ok_result: Result<i32, &str> = Ok(200);
    let err_result: Result<i32, &str> = Err("error");
    
    println!("Result Ok: {:?}", ok_result);
    println!("Result Err: {:?}\n", err_result);
}

// Traits - defining shared behavior
trait Summary {
    fn summarize(&self) -> String;
}

struct NewsArticle {
    headline: String,
    location: String,
    author: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

struct Tweet {
    username: String,
    content: String,
    reply: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

fn traits_demo() {
    println!("--- Traits ---");
    
    let article = NewsArticle {
        headline: String::from("Rust 2.0 Released!"),
        location: String::from("Internet"),
        author: String::from("Rustacean"),
    };
    
    let tweet = Tweet {
        username: String::from("rustlang"),
        content: String::from("Rust is awesome!"),
        reply: false,
    };
    
    println!("Article: {}", article.summarize());
    println!("Tweet: {}\n", tweet.summarize());
}

// Trait bounds
fn trait_bounds_demo() {
    println!("--- Trait Bounds ---");
    
    let article = NewsArticle {
        headline: String::from("Breaking News"),
        location: String::from("Tokyo"),
        author: String::from("Reporter"),
    };
    
    notify(&article);
    println!();
}

fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

// Can also be written as:
// fn notify<T: Summary>(item: &T) {
//     println!("Breaking news! {}", item.summarize());
// }

// Multiple trait bounds
fn _notify_multiple<T: Summary + std::fmt::Display>(item: &T) {
    println!("{}", item);
}

// Where clause for complex bounds
fn _some_function<T, U>(_t: &T, _u: &U) -> i32
where
    T: std::fmt::Display + Clone,
    U: Clone + std::fmt::Debug,
{
    42
}

// Default Implementations
trait DefaultSummary {
    fn summarize_author(&self) -> String;
    
    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

struct Blog {
    author: String,
}

impl DefaultSummary for Blog {
    fn summarize_author(&self) -> String {
        format!("@{}", self.author)
    }
}

fn default_impl_demo() {
    println!("--- Default Implementations ---");
    
    let blog = Blog {
        author: String::from("rustacean"),
    };
    
    println!("Blog summary: {}\n", blog.summarize());
}

// Common traits to implement:
// - Debug: for printing with {:?}
// - Clone: for explicit copying
// - Copy: for implicit copying
// - PartialEq: for == comparison
// - Eq: for full equality
// - PartialOrd: for < > comparison
// - Ord: for full ordering
// - Display: for printing with {}
