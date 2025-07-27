# Hints for Basic Functions

## Level 1: Conceptual Hint

### Understanding Functions in Rust
Functions are reusable blocks of code that perform specific tasks. In Rust, functions are defined using the `fn` keyword followed by the function name, parameters in parentheses, and an optional return type.

**Key Concepts:**
- **Function Signature**: Defines the name, parameters, and return type
- **Function Body**: Contains the code that executes when the function is called
- **Parameters**: Input values that the function can use
- **Return Values**: Output that the function provides back to the caller

**Rust Book Reference:**
Read [Chapter 3.3: Functions](https://doc.rust-lang.org/book/ch03-03-how-functions-work.html) to understand:
- Basic function syntax
- How parameters work
- Return values and the unit type `()`
- Difference between statements and expressions

**Function Types You Need:**
1. Functions that perform actions (like printing)
2. Functions that calculate and return values
3. Functions that take parameters and use them

## Level 2: Strategic Hint

### Implementation Approach

**For functions without return values:**
```rust
fn function_name() {
    // Use println! macro to print to console
    println!("Your message here");
}
```

**For functions with parameters:**
```rust
fn function_name(parameter_name: parameter_type) {
    // Use the parameter in your function body
    println!("Using parameter: {}", parameter_name);
}
```

**For functions with return values:**
```rust
fn function_name(param1: Type1, param2: Type2) -> ReturnType {
    // Calculate result
    // Return expression (no semicolon) or use return statement
    param1 + param2  // This returns the sum
}
```

**Specific Guidance for Each Function:**
1. **`greet()`**: Use `println!` to print "Hello, world!"
2. **`say_hello_to(name)`**: Use string formatting with `{}` placeholder
3. **`add_numbers(a, b)`**: Return the sum using `a + b`
4. **`multiply(x, y)`**: Return the product using `x * y`
5. **`check_even(number)`**: Use modulo operator `%` and comparison `== 0`

**Common Patterns:**
- For printing: `println!("text");`
- For formatted printing: `println!("Hello, {}!", name);`
- For returning values: End with an expression (no semicolon)
- For even checking: `number % 2 == 0`

## Level 3: Implementation Hint

### Complete Implementation Guide

**Function 1: greet()**
```rust
fn greet() {
    println!("Hello, world!");
}
```

**Function 2: say_hello_to(name: &str)**
```rust
fn say_hello_to(name: &str) {
    println!("Hello, {}!", name);
}
```
- The `{}` is a placeholder that gets replaced with the value of `name`
- The `&str` type represents a string slice (borrowed string reference)

**Function 3: add_numbers(a: i32, b: i32) -> i32**
```rust
fn add_numbers(a: i32, b: i32) -> i32 {
    a + b
}
```
- No semicolon after `a + b` because it's an expression that returns a value
- Could also write `return a + b;` but the expression form is more idiomatic

**Function 4: multiply(x: i32, y: i32) -> i32**
```rust
fn multiply(x: i32, y: i32) -> i32 {
    x * y
}
```

**Function 5: check_even(number: i32) -> bool**
```rust
fn check_even(number: i32) -> bool {
    number % 2 == 0
}
```
- `%` is the modulo operator (remainder after division)
- `number % 2` gives 0 for even numbers, 1 for odd numbers
- `== 0` compares the result to 0, returning `true` for even, `false` for odd

### Alternative Approaches

**Using explicit return statements:**
```rust
fn add_numbers(a: i32, b: i32) -> i32 {
    return a + b;  // Explicit return with semicolon
}

fn check_even(number: i32) -> bool {
    return number % 2 == 0;  // Explicit return
}
```

**Multi-line approach for complex logic:**
```rust
fn check_even(number: i32) -> bool {
    let remainder = number % 2;
    remainder == 0
}
```

### Key Learning Points
- **Expressions vs Statements**: `a + b` (expression) vs `let x = a + b;` (statement)
- **Return Values**: Last expression in function is automatically returned
- **Type Safety**: Rust ensures parameter and return types match exactly
- **String Formatting**: Use `{}` placeholders in `println!` for dynamic content