# Exercise 2.1: Reading User Input

Welcome to Chapter 2! In this exercise, you'll learn how to make your Rust programs interactive by reading input from the user.

## 🎯 Learning Objectives

By completing this exercise, you will:
- Understand how to use `std::io` for input/output operations
- Learn about mutable variables with `mut`
- Practice reading from standard input (stdin)
- Handle string input and basic input validation
- Build the foundation for interactive programs

## 📚 Background

Interactive programs need to accept input from users. In Rust, we use the `std::io` module to handle input and output operations. The key concepts you'll learn:

1. **Standard Input (stdin)**: Reading from the console where users type
2. **Mutable Variables**: Variables that can be changed after creation (using `mut`)
3. **String Storage**: Using `String::new()` to create an empty string for input
4. **Error Handling**: Basic handling of input operations that might fail

## 📖 Rust Book References

Read these sections before starting:
- [Chapter 2: Programming a Guessing Game](https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html) - Main content
- [Chapter 3.1: Variables and Mutability](https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html) - Understanding `mut`

## ✅ Your Task

Create a program that:
1. Prompts the user to enter their name
2. Reads the name from standard input
3. Prints a personalized greeting

Example interaction:
```
Please enter your name: 
Alice
Hello, Alice! Welcome to Rust programming.
```

**Requirements:**
- Use `std::io::stdin()` to read input
- Use a mutable String variable to store the input
- Handle the input properly (hint: you'll need to handle the Result)
- Print a personalized greeting using the entered name

## 🧪 Testing Your Solution

Run the tests to check your solution:
```bash
cargo test
```

Run your program interactively:
```bash
cargo run
```

## 💡 Hints Available

If you get stuck, progressive hints are available:
1. **Conceptual**: Understanding input/output and mutable variables
2. **Strategic**: How to structure the input reading process
3. **Implementation**: Specific code patterns and syntax

## 🌟 Key Concepts

- **std::io**: The standard library module for input/output operations
- **stdin()**: Function to get a handle to standard input
- **mut**: Keyword to make variables mutable (changeable)
- **String::new()**: Creates a new, empty String
- **Result handling**: Input operations return Results that need handling

## ⏭️ What's Next

After mastering user input, you'll learn how to generate random numbers and work with external crates, building toward a complete guessing game.

This is your first step into interactive Rust programming! 🦀