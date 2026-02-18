# Rust Learning Repository

A comprehensive, hands-on guide to learning Rust programming language. Perfect for experienced developers from PHP, Node.js, and other backgrounds who want to learn Rust through practical, runnable examples.

## 🎯 For Whom?

This repository is designed for:
- Backend developers with experience in PHP, Node.js, Python, etc.
- Developers with 10+ years of programming experience
- Anyone who wants to learn Rust through practical examples
- Those who prefer learning by running and modifying code

## 🚀 Quick Start

### Prerequisites

Make sure you have Rust installed. If not, install it from [rustup.rs](https://rustup.rs/):

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### Clone and Run

```bash
# Clone the repository
git clone https://github.com/motyar/rust.git
cd rust

# Run any example
cargo run -p basics
cargo run -p control_flow
cargo run -p ownership
# ... and so on
```

## 📚 Learning Path

The examples are organized in a progressive learning path. Follow them in order:

### 1. **Basics** (`01-basics`)
Learn fundamental Rust concepts:
- Variables and mutability
- Data types (integers, floats, booleans, chars)
- Tuples and arrays
- Functions
- Comments and constants

```bash
cargo run -p basics
```

### 2. **Control Flow** (`02-control-flow`)
Master program flow control:
- If/else statements
- Loops (loop, while, for)
- Match expressions (like switch, but better!)
- Pattern matching basics

```bash
cargo run -p control_flow
```

### 3. **Ownership & Borrowing** (`03-ownership`)
Understand Rust's unique ownership system:
- Ownership rules
- Move semantics
- References and borrowing
- Mutable references
- Slices

```bash
cargo run -p ownership
```

⚠️ **Important**: This is Rust's most unique feature. Take time to understand it!

### 4. **Structs & Enums** (`04-structs-enums`)
Create custom data types:
- Defining and using structs
- Tuple structs
- Methods and associated functions
- Enums and pattern matching
- Option and Result types

```bash
cargo run -p structs_enums
```

### 5. **Collections** (`05-collections`)
Work with common collections:
- Vectors (dynamic arrays)
- Strings (UTF-8 encoded)
- HashMaps (key-value pairs)
- Iterating and transforming collections

```bash
cargo run -p collections
```

### 6. **Error Handling** (`06-error-handling`)
Handle errors the Rust way:
- panic! for unrecoverable errors
- Result<T, E> for recoverable errors
- The ? operator
- Custom error types
- Error propagation

```bash
cargo run -p error_handling
```

### 7. **Generics & Traits** (`07-generics-traits`)
Write reusable, flexible code:
- Generic functions and structs
- Trait definitions and implementations
- Trait bounds
- Default implementations
- Common traits (Debug, Clone, Copy, etc.)

```bash
cargo run -p generics_traits
```

### 8. **Modules** (`08-modules`)
Organize your code:
- Module system
- Paths and use keyword
- Public vs private
- Separating code into files

```bash
cargo run -p modules
```

### 9. **Testing** (`09-testing`)
Write reliable code with tests:
- Unit tests
- Integration tests
- Test organization
- Assertions
- Test attributes

```bash
# Run the example
cargo run -p testing

# Run the tests
cargo test -p testing
```

### 10. **File I/O** (`10-file-io`)
Read and write files:
- Reading files
- Writing files
- Appending to files
- Command line arguments
- Standard input

```bash
cargo run -p file_io

# Try with arguments
cargo run -p file_io -- arg1 arg2 arg3
```

### 11. **Concurrency** (`11-concurrency`)
Write concurrent programs safely:
- Creating threads
- Message passing with channels
- Shared state with Mutex
- Arc for shared ownership
- Thread safety guarantees

```bash
cargo run -p concurrency
```

### 12. **Common Patterns** (`12-patterns`)
Learn Rust idioms and patterns:
- Closures (anonymous functions)
- Iterators and adapters
- Smart pointers (Box, Rc, RefCell)
- Advanced pattern matching
- If let and while let

```bash
cargo run -p patterns
```

## 🔧 Running Examples

### Run a Single Example
```bash
cargo run -p <package-name>
```

### Build All Examples
```bash
cargo build --all
```

### Run Tests
```bash
# Run all tests
cargo test --all

# Run tests for specific package
cargo test -p testing
```

### Check Code (Fast)
```bash
cargo check --all
```

## 📖 Additional Resources

### Official Documentation
- [The Rust Book](https://doc.rust-lang.org/book/) - Official comprehensive guide
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/) - Learn by examples
- [Rust Standard Library](https://doc.rust-lang.org/std/) - API documentation

### Practice
- [Rustlings](https://github.com/rust-lang/rustlings) - Small exercises
- [Exercism Rust Track](https://exercism.org/tracks/rust) - Coding exercises

### Community
- [Rust Forum](https://users.rust-lang.org/)
- [Rust Discord](https://discord.gg/rust-lang)
- [r/rust](https://www.reddit.com/r/rust/)

## 💡 Tips for Learning

1. **Run Every Example**: Don't just read—run the code and see the output
2. **Modify the Code**: Change values, add features, break things and fix them
3. **Read Compiler Errors**: Rust's compiler is very helpful—read the error messages carefully
4. **Follow the Order**: The examples build on each other
5. **Take Your Time**: Especially with ownership and borrowing
6. **Practice**: Write your own small programs after each section

## 🎓 What Makes Rust Different?

Coming from PHP, Node.js, or other languages, here are Rust's key differences:

- **No Garbage Collector**: Memory is managed through ownership
- **Compile-Time Safety**: Most bugs are caught before running
- **Zero-Cost Abstractions**: High-level features with no runtime cost
- **Immutable by Default**: Variables are immutable unless marked `mut`
- **Fearless Concurrency**: Data races are impossible at compile time
- **No Null**: Use `Option<T>` instead
- **No Exceptions**: Use `Result<T, E>` for error handling

## 🚀 Next Steps

After completing these examples, you can:

1. Build a CLI tool with [clap](https://github.com/clap-rs/clap)
2. Create a web server with [Actix](https://actix.rs/) or [Rocket](https://rocket.rs/)
3. Build a REST API
4. Learn async programming with [Tokio](https://tokio.rs/)
5. Explore [awesome-rust](https://github.com/rust-unofficial/awesome-rust) for more libraries

## 📝 Contributing

Found an issue or want to add more examples? Feel free to:
- Open an issue
- Submit a pull request
- Suggest improvements

## 📜 License

This repository is for educational purposes. Feel free to use, modify, and share!

---

**Happy Learning! 🦀**

Remember: Rust has a steep learning curve, but it's worth it. Take your time, practice, and don't get discouraged by compiler errors—they're here to help you write better code!