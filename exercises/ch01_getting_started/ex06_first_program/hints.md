# Hints for Simple Calculator Program

## Level 1: Conceptual Hint

This calculator exercise brings together several fundamental Rust concepts:

**Program Structure:**
- **Main function**: Entry point that coordinates the program flow
- **Helper functions**: Break down complex tasks into manageable pieces
- **Input/Output**: Reading from user and displaying results
- **Error handling**: Gracefully handle invalid input and edge cases

**Key Concepts You'll Use:**
- **Variables**: Store user input and calculation results
- **String handling**: Read input as text, then convert to numbers
- **Functions**: Organize code into reusable pieces
- **Pattern matching**: Handle different operations
- **Error handling**: Deal with invalid input and division by zero

**Program Flow:**
1. Welcome the user
2. Get first number (with input validation)
3. Get second number (with input validation)  
4. Get operation choice
5. Perform calculation
6. Display result or error
7. Say goodbye

**Error Scenarios to Handle:**
- User enters non-numeric input (like "abc" instead of "123")
- User attempts division by zero
- User enters an invalid operation symbol
- Input/output errors

**Function Design Principles:**
- Each function should have a single, clear purpose
- Use descriptive names (get_number, perform_addition, etc.)
- Handle errors where they occur
- Return meaningful values or error messages

Remember: Start simple and add complexity gradually. Get basic functionality working first, then add error handling and polish!

## Level 2: Strategic Hint

Here's how to approach building your calculator step by step:

**Step 1: Input Handling Strategy**
```rust
// Create a helper function for getting numbers
fn get_number(prompt: &str) -> f64 {
    loop {
        println!("{}", prompt);
        let mut input = String::new();
        
        // Read the line
        io::stdin().read_line(&mut input).expect("Failed to read line");
        
        // Try to parse it
        match input.trim().parse() {
            Ok(num) => return num,
            Err(_) => println!("Please enter a valid number!"),
        }
    }
}
```

**Step 2: Operation Functions**
Create separate functions for each operation:
```rust
fn add(a: f64, b: f64) -> f64 { a + b }
fn subtract(a: f64, b: f64) -> f64 { a - b }
fn multiply(a: f64, b: f64) -> f64 { a * b }

fn divide(a: f64, b: f64) -> Result<f64, &'static str> {
    if b == 0.0 {
        Err("Cannot divide by zero!")
    } else {
        Ok(a / b)
    }
}
```

**Step 3: Main Program Logic**
```rust
fn main() {
    println!("=== Simple Calculator ===");
    
    let num1 = get_number("Enter the first number:");
    let num2 = get_number("Enter the second number:");
    
    let operation = get_operation();
    
    let result = calculate(num1, num2, &operation);
    display_result(num1, num2, &operation, result);
}
```

**Step 4: Operation Selection**
```rust
fn get_operation() -> String {
    loop {
        println!("Enter operation (+, -, *, /):");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        
        let op = input.trim();
        if ["+", "-", "*", "/"].contains(&op) {
            return op.to_string();
        } else {
            println!("Please enter +, -, *, or /");
        }
    }
}
```

**Error Handling Strategy:**
- Use `Result<T, E>` for operations that can fail
- Use loops with validation for user input
- Provide clear error messages
- Allow users to retry on invalid input

**Code Organization:**
- Group related functionality into functions
- Use clear, descriptive variable names
- Add comments explaining non-obvious logic
- Keep main() function clean and high-level

## Level 3: Implementation Hint

Here's a complete implementation structure with all the key components:

**Complete main.rs structure:**

```rust
//! Simple Calculator Program
//! 
//! This program demonstrates basic Rust concepts including functions,
//! variables, user input, string parsing, and error handling.

use std::io;

fn main() {
    println!("=== Simple Calculator ===");
    println!("Enter two numbers and choose an operation!");
    
    // Get user input for numbers and operation
    let num1 = get_number("Enter the first number:");
    let num2 = get_number("Enter the second number:");
    let operation = get_operation();
    
    // Perform calculation and display result
    match calculate(num1, num2, &operation) {
        Ok(result) => {
            println!("\nResult: {} {} {} = {}", num1, operation, num2, result);
        }
        Err(error) => {
            println!("\nError: {}", error);
        }
    }
    
    println!("Thank you for using the calculator!");
}

/// Gets a number from user input with validation and retry logic
fn get_number(prompt: &str) -> f64 {
    loop {
        println!("{}", prompt);
        let mut input = String::new();
        
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                match input.trim().parse::<f64>() {
                    Ok(number) => return number,
                    Err(_) => println!("Invalid input! Please enter a valid number."),
                }
            }
            Err(_) => println!("Failed to read input! Please try again."),
        }
    }
}

/// Gets operation choice from user with validation
fn get_operation() -> String {
    loop {
        println!("Enter operation (+, -, *, /):");
        let mut input = String::new();
        
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                let operation = input.trim();
                if ["+", "-", "*", "/"].contains(&operation) {
                    return operation.to_string();
                } else {
                    println!("Invalid operation! Please enter +, -, *, or /");
                }
            }
            Err(_) => println!("Failed to read input! Please try again."),
        }
    }
}

/// Performs the calculation based on the operation
fn calculate(num1: f64, num2: f64, operation: &str) -> Result<f64, String> {
    match operation {
        "+" => Ok(add(num1, num2)),
        "-" => Ok(subtract(num1, num2)),
        "*" => Ok(multiply(num1, num2)),
        "/" => divide(num1, num2),
        _ => Err(format!("Unknown operation: {}", operation)),
    }
}

/// Addition function
fn add(a: f64, b: f64) -> f64 {
    a + b
}

/// Subtraction function  
fn subtract(a: f64, b: f64) -> f64 {
    a - b
}

/// Multiplication function
fn multiply(a: f64, b: f64) -> f64 {
    a * b
}

/// Division function with zero-check
fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err("Cannot divide by zero!".to_string())
    } else {
        Ok(a / b)
    }
}
```

**Key Implementation Details:**

1. **Input Validation**: Use loops with `match` statements to retry on invalid input
2. **Error Handling**: Return `Result` types for operations that can fail
3. **Function Organization**: Separate concerns into focused functions
4. **User Experience**: Clear prompts and helpful error messages
5. **Type Safety**: Use `f64` for decimal arithmetic operations

**Testing Your Implementation:**
- Test valid operations: 10 + 5, 20 - 8, 6 * 7, 50 / 10
- Test edge cases: division by zero, invalid input
- Test error recovery: ensure program continues after errors
- Test user interface: clear prompts and readable output

**Extension Ideas:**
- Add more operations (modulo, exponentiation)
- Support multiple calculations in one session
- Add operation history
- Improve input parsing (support expressions like "2+3")