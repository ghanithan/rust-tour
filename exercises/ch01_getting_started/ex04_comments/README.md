# Exercise 1.4: Documentation and Code Comments

Learn about Rust's different comment styles and how to write clear, helpful documentation for your code.

## 🎯 Learning Objectives

By completing this exercise, you will:
- Understand different types of comments in Rust
- Learn when and how to use regular comments vs documentation comments  
- Practice writing clear, helpful documentation
- Understand how `cargo doc` generates documentation
- Learn about comment best practices

## 📚 Background

Good documentation is crucial for maintainable code. Rust has excellent built-in support for documentation through comments, and the community values well-documented code highly.

Rust supports several types of comments:
- **Line comments** (`//`) - For regular code comments
- **Block comments** (`/* */`) - For multi-line comments  
- **Doc comments** (`///` and `//!`) - For generating documentation
- **Inner doc comments** (`//!`) - For documenting modules and crates

## 📖 Rust Book References

Read these sections:
- [Chapter 3.4: Comments](https://doc.rust-lang.org/book/ch03-04-comments.html) - Essential reading
- [Chapter 14.2: Publishing a Crate](https://doc.rust-lang.org/book/ch14-02-publishing-to-crates-io.html) - Documentation standards

## ✅ Your Task

Complete the `src/main.rs` file by:

1. **Adding appropriate comments** where TODO markers indicate
2. **Using different comment styles** correctly
3. **Writing documentation comments** for the functions
4. **Following Rust documentation conventions**

The program calculates the area of basic shapes and should demonstrate:
- Regular line comments for code explanation
- Documentation comments for functions
- Multi-line comments for complex logic
- Module-level documentation

## 🧪 Testing Your Solution

Run your program:
```bash
cargo run
```

Generate and view documentation:
```bash
cargo doc --open
```

Run the tests:
```bash
cargo test
```

Check your documentation:
```bash
cargo doc --no-deps
```

## 💡 Hints Available

1. **Conceptual**: Understanding comment types and when to use them
2. **Strategic**: How to write effective documentation
3. **Implementation**: Specific comment examples and syntax

## 🌟 Key Concepts

### Comment Types

**Line Comments (`//`)**
```rust
// This is a regular comment
let x = 5; // Comments can go at the end of lines too
```

**Block Comments (`/* */`)**  
```rust
/*
This is a multi-line comment
that spans several lines
*/
```

**Documentation Comments (`///`)**
```rust
/// This function adds two numbers together.
/// 
/// # Examples
/// ```
/// let result = add(2, 3);
/// assert_eq!(result, 5);
/// ```
fn add(a: i32, b: i32) -> i32 {
    a + b
}
```

**Inner Doc Comments (`//!`)**
```rust
//! This module contains math utilities.
//! 
//! It provides basic arithmetic operations.
```

### Documentation Best Practices

- **Write for your future self** - You'll forget how code works
- **Explain the "why", not just the "what"** - Code shows what, comments explain why
- **Use examples** - Show how to use functions
- **Keep comments up to date** - Outdated comments are worse than no comments
- **Use proper grammar** - Documentation is user-facing

### Cargo Doc Generation

Rust can automatically generate beautiful HTML documentation from your doc comments:
- `cargo doc` - Generate documentation
- `cargo doc --open` - Generate and open in browser
- `cargo doc --no-deps` - Generate only for your crate

## ⏭️ What's Next

After mastering comments and documentation, you'll move on to learning about Rust editions and then create your first substantial Rust program!