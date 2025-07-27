# Exercise 1.6: Simple Calculator Program

Create your first complete Rust program - a simple calculator that demonstrates variables, functions, user input, string parsing, and basic operations.

## 🎯 Learning Objectives

By completing this exercise, you will:
- Create a complete, interactive Rust program from scratch
- Practice reading and parsing user input
- Implement helper functions and proper program structure
- Handle errors gracefully (division by zero, invalid input)
- Use variables, string manipulation, and basic operations
- Apply everything learned in Chapter 1

## 📚 Background

This exercise brings together all the concepts from Chapter 1 into one cohesive program. You'll build a simple calculator that:
- Takes user input for two numbers
- Asks for an operation (+, -, *, /)
- Performs the calculation
- Displays the result
- Handles common error cases

This mirrors the structure of the guessing game in Chapter 2 of the Rust Book, giving you experience with the fundamental patterns of Rust programs.

## 📖 Rust Book References

Read these sections:
- [Chapter 2: Programming a Guessing Game](https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html) - Essential reading
- [Chapter 1.2: Hello, World!](https://doc.rust-lang.org/book/ch01-02-hello-world.html) - Basic program structure
- [Chapter 1.3: Hello, Cargo!](https://doc.rust-lang.org/book/ch01-03-hello-cargo.html) - Using Cargo

## ✅ Your Task

Complete the `src/main.rs` file by implementing:

1. **User Input Functions**
   - Get numbers from user input with error handling
   - Parse strings to numeric types
   - Handle invalid input gracefully

2. **Calculation Functions**
   - Addition, subtraction, multiplication, division
   - Handle division by zero
   - Return meaningful error messages

3. **Main Program Logic**
   - Interactive user interface
   - Clear prompts and instructions
   - Proper program flow and structure

4. **Error Handling**
   - Invalid number input
   - Division by zero
   - Invalid operations

The calculator should:
- Prompt for two numbers
- Ask for an operation (+, -, *, /)
- Display the result or error message
- Exit gracefully

## 🧪 Testing Your Solution

Run your program:
```bash
cargo run
```

Test with different inputs:
```bash
# Try valid operations
10
5
+

# Try division by zero
10
0
/

# Try invalid input
abc
5
+
```

Run the automated tests:
```bash
cargo test
```

Check your code quality:
```bash
cargo clippy
cargo fmt
```

## 💡 Hints Available

1. **Conceptual**: Understanding program structure, user input, and error handling
2. **Strategic**: How to organize functions and handle different input scenarios
3. **Implementation**: Specific code examples and error handling patterns

## 🌟 Key Concepts

### User Input Handling
```rust
use std::io;

let mut input = String::new();
io::stdin().read_line(&mut input)
    .expect("Failed to read line");

let number: f64 = input.trim().parse()
    .expect("Invalid number");
```

### Error Handling with Result
```rust
fn get_number() -> Result<f64, String> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)
        .map_err(|_| "Failed to read input")?;
    
    input.trim().parse()
        .map_err(|_| "Invalid number format")
}
```

### Function Organization
```rust
fn main() {
    // Main program logic
}

fn get_number() -> f64 {
    // Input handling
}

fn add(a: f64, b: f64) -> f64 {
    // Calculation
}

fn divide(a: f64, b: f64) -> Result<f64, String> {
    // Division with error handling
}
```

### Match Statements for Operations
```rust
match operation.as_str() {
    "+" => add(num1, num2),
    "-" => subtract(num1, num2),
    "*" => multiply(num1, num2),
    "/" => divide(num1, num2),
    _ => {
        println!("Invalid operation!");
        return;
    }
}
```

## 🔧 Program Requirements

**Must Have:**
- Interactive prompts for user input
- Functions for each mathematical operation
- Error handling for division by zero
- Input validation for numbers
- Clear output formatting
- Proper documentation and comments

**Nice to Have:**
- Loop for multiple calculations
- More operations (%, ^)
- Better error messages
- Input history

## 🎮 Example Session

```
=== Simple Calculator ===
Enter two numbers and choose an operation!

Enter the first number: 15
Enter the second number: 3
Enter operation (+, -, *, /): *

Result: 15 * 3 = 45

Thank you for using the calculator!
```

## ⚠️ Common Pitfalls

- **Forgetting to trim input** - `read_line` includes the newline
- **Not handling parse errors** - Invalid input will panic without error handling
- **Division by zero** - Check for zero before dividing
- **String ownership** - Be careful with String vs &str
- **Input loops** - Consider what happens with invalid input

## ⏭️ What's Next

Congratulations! You've completed Chapter 1 and built your first real Rust program. In Chapter 2, you'll dive deeper into:
- Variables and mutability
- Data types
- Control flow
- Ownership concepts

This calculator demonstrates the foundation you'll build upon throughout your Rust journey!