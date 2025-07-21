# Hints for Hello, Cargo!

## Level 1: Conceptual Hint

This exercise is about understanding **Cargo**, which is Rust's build system and package manager. Think of it like npm for Node.js, pip for Python, or Maven for Java.

**Key concepts:**
- `Cargo.toml` is the configuration file that defines your project
- Dependencies are external libraries (called "crates" in Rust)
- The `src/` directory contains your source code
- Cargo handles building, running, and testing your code

**What you need to do:**
1. Add a dependency to `Cargo.toml` 
2. Use that dependency in your `main.rs` file
3. Create colorful output using the external crate

**📖 Read:** [Rust Book Chapter 1.3 - Hello, Cargo!](https://doc.rust-lang.org/book/ch01-03-hello-cargo.html)

**💡 Think about:** How do external libraries help you avoid writing everything from scratch?

## Level 2: Strategic Hint

You need to complete two files:

### 1. Fix Cargo.toml
Add the `colored` crate as a dependency. The syntax is:
```toml
[dependencies]
crate_name = "version"
```

For the `colored` crate, you want version "2".

### 2. Use the crate in main.rs
To use an external crate:
1. Import it with `use` statement
2. Use its functionality in your code

The `colored` crate lets you add colors to text:
```rust
use colored::*;  // Import everything from colored

fn main() {
    println!("{}", "This is green".green());
    println!("{}", "This is blue and bold".blue().bold());
}
```

**Strategy:**
- First fix the Cargo.toml
- Then import and use `colored` in main.rs
- Create multiple colorful messages

## Level 3: Implementation Hint

### Complete Cargo.toml:
Replace the TODO line with:
```toml
colored = "2"
```

Your complete Cargo.toml should look like:
```toml
[package]
name = "hello_cargo"
version = "0.1.0"
edition = "2021"

[dependencies]
colored = "2"
```

### Complete main.rs:
```rust
use colored::*;

fn main() {
    println!("{}", "Hello, Cargo!".green().bold());
    println!("{}", "This is a Rust project managed by Cargo!".blue());
    println!("{}", "Cargo handles:".yellow());
    println!("  {} {}", "•".red(), "Building your code");
    println!("  {} {}", "•".red(), "Managing dependencies");
    println!("  {} {}", "•".red(), "Running tests");
    println!("{}", "Welcome to the Rust ecosystem! 🦀".magenta().bold());
}
```

### Test your solution:
```bash
cargo run
```

You should see colorful output demonstrating the `colored` crate in action!