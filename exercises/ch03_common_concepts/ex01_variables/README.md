# Exercise 3.1: Variables and Mutability

Learn about Rust's unique approach to variables, where immutability is the default and mutability must be explicitly declared. This exercise will teach you about variable binding, the `mut` keyword, shadowing, and constants.

## 🎯 Learning Objectives

By completing this exercise, you will understand:
- Why Rust variables are immutable by default
- How to make variables mutable with the `mut` keyword
- The difference between shadowing and mutability
- When and how to use constants
- How Rust's variable system helps prevent bugs

## 📚 Background

Rust takes a unique approach to variables that helps prevent common programming errors:

- **Immutable by default**: Variables can't be changed unless explicitly marked as mutable
- **Explicit mutability**: Use the `mut` keyword when you need to change a variable
- **Shadowing**: You can declare a new variable with the same name, "shadowing" the previous one
- **Constants**: Compile-time constants that are always immutable

This design prevents many bugs while still allowing flexibility when you need it.

## 📖 Rust Book References

Essential reading:
- [Chapter 3.1: Variables and Mutability](https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html) - Core concepts for this exercise

Additional context:
- [Chapter 4.1: What is Ownership?](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html) - How variables relate to ownership

## ✅ Your Task

The `main.rs` file contains code with several compilation errors related to variables and mutability. Your job is to fix these errors while learning about:

1. **Immutable variables** - The default behavior
2. **Mutable variables** - Using the `mut` keyword  
3. **Shadowing** - Reusing variable names
4. **Constants** - Compile-time values

## 🐛 Fix These Issues

The code has intentional errors that you need to fix:
- Attempting to modify immutable variables
- Missing `mut` keywords where needed
- Incorrect constant declarations
- Misunderstanding of shadowing vs mutation

## 🧪 Testing Your Solution

Check that your code compiles:
```bash
cargo check
```

Run your program:
```bash
cargo run
```

Run the tests:
```bash
cargo test
```

## 💡 Hints Available

Three levels of hints are available:
1. **Conceptual**: Understanding Rust's variable system
2. **Strategic**: Identifying what needs to be fixed
3. **Implementation**: Specific code corrections

## 🌟 Key Concepts

### Immutable Variables (Default)
```rust
let x = 5;
// x = 6; // This would cause an error!
```

### Mutable Variables
```rust
let mut x = 5;
x = 6; // This is allowed
```

### Shadowing
```rust
let x = 5;
let x = x + 1; // This creates a new variable, shadowing the old one
let x = "hello"; // Shadowing can even change the type!
```

### Constants
```rust
const MAX_POINTS: u32 = 100_000;
```

## 🤔 Why Does Rust Do This?

Rust's approach to variables helps prevent:
- **Data races** in concurrent code
- **Accidental mutations** that cause bugs
- **Use-after-move** errors
- **Memory corruption** issues

The compiler catches these issues at compile time rather than runtime!

## ⏭️ What's Next

After mastering variables, you'll learn about:
- Data types and type inference
- Functions and parameters
- Control flow with `if` expressions and loops

Understanding variables is crucial for everything else in Rust, so take your time with this exercise! 🦀