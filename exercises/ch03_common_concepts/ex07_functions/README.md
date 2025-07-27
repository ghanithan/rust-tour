# Exercise 7: Basic Functions

## Learning Objectives
By completing this exercise, you will learn:
- How to define functions in Rust
- Function parameters and their types
- Return types and how to return values
- The difference between expressions and statements in function bodies
- How to call functions and use their return values

## Prerequisites
- Understanding of variables and data types
- Basic familiarity with Rust syntax
- Knowledge of primitive types (i32, bool, &str)

## Instructions
In this exercise, you'll complete several function definitions to understand how functions work in Rust.

Your task is to implement the following functions in `src/main.rs`:

1. **`greet()`** - A simple function that prints "Hello, world!" to the console
2. **`say_hello_to(name: &str)`** - A function that takes a name parameter and prints a personalized greeting
3. **`add_numbers(a: i32, b: i32) -> i32`** - A function that takes two integers and returns their sum
4. **`multiply(x: i32, y: i32) -> i32`** - A function that multiplies two integers and returns the result
5. **`check_even(number: i32) -> bool`** - A function that checks if a number is even and returns a boolean

## Key Concepts

### Function Definition Syntax
```rust
fn function_name(parameter: Type) -> ReturnType {
    // function body
    expression_or_return_statement
}
```

### Functions with No Return Value
Functions that don't return a value have an implicit return type of `()` (unit type):
```rust
fn greet() {
    println!("Hello!");
}
```

### Functions with Return Values
Functions can return values using expressions (without semicolon) or explicit return statements:
```rust
fn add(a: i32, b: i32) -> i32 {
    a + b  // expression - returns the value
}

// OR using explicit return
fn add_explicit(a: i32, b: i32) -> i32 {
    return a + b;  // explicit return statement
}
```

### Function Parameters
Parameters are declared with their names and types:
```rust
fn say_hello(name: &str, age: i32) {
    println!("Hello, {}! You are {} years old.", name, age);
}
```

## Expected Output
When you run the completed program, you should see:
```
=== Basic Functions ===
Hello, world!
Hello, Alice!
Sum: 8
Product: 28
Is 10 even? true
```

## Testing
Run `cargo test` to verify your implementation. The tests will check that:
- Your functions produce the correct output
- Functions with return values return the expected results
- The program compiles and runs without errors

## Tips
- Remember that in Rust, the last expression in a function is automatically returned (no semicolon needed)
- Use `println!` macro for printing to the console
- The modulo operator `%` can help check if a number is even (remainder when divided by 2)
- Function names should use snake_case in Rust

## Estimated Time
15 minutes

## Rust Book References
- [Chapter 3.3: Functions](https://doc.rust-lang.org/book/ch03-03-how-functions-work.html)

## Next Steps
After completing this exercise, you'll be ready to learn about:
- Comments and documentation
- Control flow (if expressions, loops)
- More complex function patterns and closures