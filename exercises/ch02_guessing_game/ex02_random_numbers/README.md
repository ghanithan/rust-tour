# Exercise 2.2: Generating Random Numbers

In this exercise, you'll learn how to use external crates in Rust by adding random number generation capabilities to your programs.

## 🎯 Learning Objectives

By completing this exercise, you will:
- Understand how to add external dependencies to Cargo.toml
- Learn about the Rust ecosystem and crates.io
- Use the `rand` crate to generate random numbers
- Practice working with ranges and number generation
- Understand the difference between standard library and external crates

## 📚 Background

The Rust standard library is intentionally minimal to keep compilation fast and the language focused. Additional functionality comes from external crates (libraries) published on [crates.io](https://crates.io). 

For random number generation, we use the `rand` crate, which provides:
1. **Random Number Generators**: Different algorithms for generating random numbers
2. **Range Support**: Generate numbers within specific ranges
3. **Thread Safety**: Safe random number generation in concurrent programs

Key concepts:
- **Crates**: External libraries that extend Rust's functionality
- **Dependencies**: External crates your project depends on
- **Cargo.toml**: The file where you declare dependencies
- **use statements**: How to import functionality from crates

## 📖 Rust Book References

Read these sections before starting:
- [Chapter 2: Programming a Guessing Game - Generating a Random Number](https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html#generating-a-random-number) - Main content
- [Chapter 1.3: Hello, Cargo!](https://doc.rust-lang.org/book/ch01-03-hello-cargo.html) - Understanding Cargo

## ✅ Your Task

Create a program that generates and displays random numbers:

1. **Add the rand dependency** to your Cargo.toml file
2. **Generate a random number** between 1 and 100 (inclusive)
3. **Display the number** with a descriptive message
4. **Generate multiple numbers** to show randomness

Example output:
```
Generating random numbers...
Random number: 42
Random number: 87
Random number: 23
All numbers are between 1 and 100 (inclusive).
```

**Requirements:**
- Add `rand = "0.8"` to your Cargo.toml dependencies
- Use `rand::thread_rng()` to get a random number generator
- Use the `gen_range()` method to generate numbers in the range 1..=100
- Generate and display at least 3 random numbers

## 🧪 Testing Your Solution

Run the tests to check your solution:
```bash
cargo test
```

Run your program to see random numbers:
```bash
cargo run
```

Note: Each run should produce different numbers!

## 💡 Hints Available

Progressive hints are available if you get stuck:
1. **Conceptual**: Understanding external crates and dependencies
2. **Strategic**: How to add dependencies and use the rand crate
3. **Implementation**: Specific code and configuration

## 🌟 Key Concepts

- **External Dependencies**: Code written by other developers that you can use
- **Cargo.toml**: Configuration file for Rust projects and dependencies
- **crates.io**: The official registry for Rust crates
- **Semantic Versioning**: How crate versions are numbered (e.g., "0.8")
- **Random Number Generation**: Creating unpredictable numbers for games and simulations

## ⏭️ What's Next

After learning about external crates and random numbers, you'll work with string parsing and type conversion to handle user input more robustly.

You're building the core components of a guessing game! 🎲