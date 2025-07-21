# Exercise 1.2: Hello, Cargo!

In this exercise, you'll learn about Cargo, Rust's build system and package manager. You'll understand project structure and how to use external dependencies.

## 🎯 Learning Objectives

By completing this exercise, you will:
- Understand the role of Cargo in Rust development
- Learn about `Cargo.toml` configuration files
- Experience adding external dependencies
- Understand Rust project structure conventions
- Use Cargo commands to build and run projects

## 📚 Background

**Cargo** is Rust's built-in build system and package manager. It handles:
- Building your code
- Managing dependencies (external libraries called "crates")
- Running tests
- Publishing packages
- Much more!

Every Rust project should use Cargo, even simple ones. The `Cargo.toml` file is the heart of your project configuration.

## 📖 Rust Book References

Read these sections before starting:
- [Chapter 1.3: Hello, Cargo!](https://doc.rust-lang.org/book/ch01-03-hello-cargo.html) - Essential reading
- [Chapter 14: More About Cargo](https://doc.rust-lang.org/book/ch14-00-more-about-cargo.html) - For deeper understanding

## ✅ Your Tasks

### Task 1: Complete the Cargo.toml
The `Cargo.toml` file has a TODO comment. Add the `colored` crate as a dependency to enable colorful terminal output.

### Task 2: Implement the Main Function
Create a program in `src/main.rs` that:
1. Prints a colorful greeting using the `colored` crate
2. Demonstrates basic Cargo project structure

Your program should output something like:
```
Hello, Cargo! (in green)
This is a Rust project managed by Cargo! (in blue)
```

## 🧪 Testing Your Solution

Test that your Cargo.toml is valid:
```bash
cargo check
```

Build your project:
```bash
cargo build
```

Run your project:
```bash
cargo run
```

Run the tests:
```bash
cargo test
```

## 💡 Hints Available

If you need help:
1. **Conceptual**: Understanding Cargo and project structure
2. **Strategic**: How to add dependencies and structure your code
3. **Implementation**: Specific code examples

## 🌟 Key Concepts

### Cargo.toml Structure
```toml
[package]
name = "project_name"      # Your project's name
version = "0.1.0"         # Semantic versioning
edition = "2021"          # Rust edition

[dependencies]
some_crate = "1.0"        # External dependencies
```

### Project Structure
```
project/
├── Cargo.toml           # Project configuration
├── src/                 # Source code directory
│   └── main.rs         # Main source file
└── target/             # Build output (created by Cargo)
```

### Using External Crates
```rust
use crate_name::SomeType;  // Import from external crate

fn main() {
    // Use the imported functionality
}
```

## 📦 About the `colored` Crate

The `colored` crate allows you to add colors to terminal output:

```rust
use colored::*;

println!("{}", "Hello".green());
println!("{}", "World".blue().bold());
```

Find more crates at [crates.io](https://crates.io) - Rust's package registry.

## ⏭️ What's Next

After this exercise, you'll understand how Rust projects are structured and managed. Next, we'll dive into variables, data types, and the fundamentals of Rust programming!

## 🔧 Useful Cargo Commands

- `cargo new project_name` - Create a new project
- `cargo build` - Build the project
- `cargo run` - Build and run the project
- `cargo test` - Run tests
- `cargo check` - Check if code compiles (faster than build)
- `cargo clean` - Clean build artifacts