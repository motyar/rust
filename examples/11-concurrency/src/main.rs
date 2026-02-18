// 11-concurrency: Concurrent programming in Rust
// Learn about threads, message passing, and shared state

use std::thread;
use std::time::Duration;
use std::sync::{mpsc, Arc, Mutex};

fn main() {
    println!("=== 11: CONCURRENCY ===\n");
    
    // 1. Creating threads
    threads_demo();
    
    // 2. Move closures
    move_closures_demo();
    
    // 3. Message passing
    message_passing_demo();
    
    // 4. Multiple producers
    multiple_producers_demo();
    
    // 5. Shared state with Mutex
    shared_state_demo();
    
    // 6. Multiple threads with shared state
    multiple_threads_demo();
}

// Creating threads
fn threads_demo() {
    println!("--- Creating Threads ---");
    
    let handle = thread::spawn(|| {
        for i in 1..5 {
            println!("  Number {} from spawned thread", i);
            thread::sleep(Duration::from_millis(1));
        }
    });
    
    for i in 1..3 {
        println!("Number {} from main thread", i);
        thread::sleep(Duration::from_millis(1));
    }
    
    // Wait for thread to finish
    handle.join().unwrap();
    
    println!();
}

// Move closures - moving ownership into thread
fn move_closures_demo() {
    println!("--- Move Closures ---");
    
    let v = vec![1, 2, 3];
    
    let handle = thread::spawn(move || {
        println!("  Vector in thread: {:?}", v);
    });
    
    handle.join().unwrap();
    
    // v is no longer accessible here - ownership moved to thread
    println!();
}

// Message passing with channels
fn message_passing_demo() {
    println!("--- Message Passing ---");
    
    let (tx, rx) = mpsc::channel();
    
    thread::spawn(move || {
        let val = String::from("Hello from thread");
        tx.send(val).unwrap();
    });
    
    let received = rx.recv().unwrap();
    println!("Received: {}\n", received);
}

// Multiple messages
fn multiple_producers_demo() {
    println!("--- Multiple Messages ---");
    
    let (tx, rx) = mpsc::channel();
    let tx1 = tx.clone();
    
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("thread"),
            String::from("one"),
        ];
        
        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_millis(100));
        }
    });
    
    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];
        
        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_millis(100));
        }
    });
    
    // Receive all messages
    for received in rx {
        println!("  Received: {}", received);
    }
    
    println!();
}

// Shared state with Mutex
fn shared_state_demo() {
    println!("--- Shared State with Mutex ---");
    
    let m = Mutex::new(5);
    
    {
        let mut num = m.lock().unwrap();
        *num = 6;
    } // lock is released when num goes out of scope
    
    println!("m = {:?}\n", m);
}

// Multiple threads with shared state
fn multiple_threads_demo() {
    println!("--- Multiple Threads with Shared State ---");
    
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    
    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    println!("Result: {}\n", *counter.lock().unwrap());
}

// Key concepts:
// - thread::spawn() - creates a new thread
// - .join() - waits for thread to finish
// - move - moves ownership into closure
// - mpsc::channel() - creates a channel for message passing
// - Mutex<T> - mutual exclusion for shared state
// - Arc<T> - atomic reference counting for shared ownership across threads
//
// Rust's ownership and type system prevent data races at compile time!
