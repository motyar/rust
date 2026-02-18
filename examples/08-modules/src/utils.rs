// Utility functions module

pub mod math {
    pub fn add(a: i32, b: i32) -> i32 {
        a + b
    }
    
    pub fn multiply(a: i32, b: i32) -> i32 {
        a * b
    }
    
    pub fn subtract(a: i32, b: i32) -> i32 {
        a - b
    }
}

pub mod string_utils {
    pub fn to_uppercase(s: &str) -> String {
        s.to_uppercase()
    }
    
    pub fn reverse(s: &str) -> String {
        s.chars().rev().collect()
    }
}
