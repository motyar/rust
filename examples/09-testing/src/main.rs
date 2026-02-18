// 09-testing: Writing tests in Rust
// Learn about unit tests, integration tests, and test organization

fn main() {
    println!("=== 09: TESTING ===\n");
    println!("Run tests with: cargo test -p testing");
    println!("Run specific test: cargo test -p testing test_name");
    println!("Run with output: cargo test -p testing -- --show-output\n");
    
    // Example functions to test
    println!("add(2, 3) = {}", add(2, 3));
    println!("multiply(4, 5) = {}", multiply(4, 5));
    
    let rect = Rectangle { width: 30, height: 50 };
    println!("Rectangle area: {}", rect.area());
    println!("Can hold smaller? {}", rect.can_hold(&Rectangle { width: 20, height: 40 }));
}

// Functions to test
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

pub fn multiply(a: i32, b: i32) -> i32 {
    a * b
}

#[derive(Debug, PartialEq)]
pub struct Rectangle {
    pub width: u32,
    pub height: u32,
}

impl Rectangle {
    pub fn area(&self) -> u32 {
        self.width * self.height
    }
    
    pub fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

pub fn greeting(name: &str) -> String {
    format!("Hello, {}!", name)
}

pub fn divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err(String::from("Cannot divide by zero"))
    } else {
        Ok(a / b)
    }
}

// Unit tests - test private and public functions
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_add() {
        assert_eq!(add(2, 3), 5);
    }
    
    #[test]
    fn test_multiply() {
        assert_eq!(multiply(4, 5), 20);
    }
    
    #[test]
    fn test_rectangle_area() {
        let rect = Rectangle {
            width: 10,
            height: 20,
        };
        assert_eq!(rect.area(), 200);
    }
    
    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };
        
        assert!(larger.can_hold(&smaller));
    }
    
    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };
        
        assert!(!smaller.can_hold(&larger));
    }
    
    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        assert!(
            result.contains("Carol"),
            "Greeting did not contain name, value was `{}`",
            result
        );
    }
    
    #[test]
    fn test_divide_success() {
        assert_eq!(divide(10, 2), Ok(5));
    }
    
    #[test]
    fn test_divide_by_zero() {
        assert!(divide(10, 0).is_err());
    }
    
    #[test]
    #[should_panic(expected = "index out of bounds")]
    fn test_panic() {
        let v = vec![1, 2, 3];
        v[99];  // This should panic
    }
    
    #[test]
    fn test_with_result() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }
    
    #[test]
    #[ignore]
    fn expensive_test() {
        // This test is ignored by default
        // Run with: cargo test -- --ignored
        assert_eq!(2 + 2, 4);
    }
}

// Common test attributes:
// #[test] - marks a function as a test
// #[should_panic] - test passes if function panics
// #[ignore] - test is skipped unless --ignored flag is used
//
// Common assertions:
// assert!(boolean) - panics if false
// assert_eq!(a, b) - panics if not equal
// assert_ne!(a, b) - panics if equal
