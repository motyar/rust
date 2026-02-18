// 08-modules: Organizing code with modules
// Learn about modules, paths, use keyword, and pub

mod front_of_house;
mod utils;

use crate::front_of_house::hosting;
use crate::utils::math;

fn main() {
    println!("=== 08: MODULES ===\n");
    
    // 1. Using modules
    modules_demo();
    
    // 2. Paths and use
    paths_demo();
}

fn modules_demo() {
    println!("--- Modules ---");
    
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();
    
    // Relative path with use
    hosting::seat_at_table();
    
    println!();
}

fn paths_demo() {
    println!("--- Paths and Use ---");
    
    // Using utilities
    let result = math::add(5, 3);
    println!("5 + 3 = {}", result);
    
    let result = math::multiply(4, 7);
    println!("4 * 7 = {}", result);
    
    println!();
}
