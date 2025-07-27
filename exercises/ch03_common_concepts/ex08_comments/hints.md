# Hints for Comments in Rust

## Level 1: Conceptual Hint

**Understanding Comment Types in Rust**

Rust has three main types of comments:

1. **Line Comments (`//`)** - Single line explanations
   - Used for brief explanations
   - Everything after `//` is ignored by the compiler
   - Can be used inline or on separate lines

2. **Block Comments (`/* */`)** - Multi-line comments
   - Span multiple lines
   - Useful for temporarily disabling code
   - Can be nested in Rust (unlike some languages)

3. **Documentation Comments (`///`)** - Special comments for documentation
   - Generate documentation with `cargo doc`
   - Support Markdown formatting
   - Used above functions, structs, and modules

**Comment Best Practices:**
- Explain WHY, not WHAT (the code already shows what)
- Be concise but clear
- Update comments when code changes
- Use TODO for incomplete functionality

**Rust Book References:**
- [Comments](https://doc.rust-lang.org/book/ch03-04-comments.html)
- [Documentation Comments](https://doc.rust-lang.org/rustdoc/how-to-write-documentation.html)

## Level 2: Strategic Hint

**Comment Implementation Strategies**

**For Line Comments:**
```rust
// Explanatory comment above the line
let variable = value;

let variable = value; // Inline comment explaining the value
```

**For Block Comments:**
```rust
/* This is a multi-line comment
   that can explain complex logic
   or temporarily disable code blocks */
```

**For Documentation Comments:**
```rust
/// Brief description of what the function does.
/// 
/// # Arguments
/// * `param1` - Description of first parameter
/// * `param2` - Description of second parameter
/// 
/// # Returns
/// Description of what the function returns
fn my_function(param1: Type1, param2: Type2) -> ReturnType {
    // Implementation
}
```

**Commenting Out Code:**
```rust
// let unused = 42;  // Single line commented out

/* 
let block_of_code = "disabled";
println!("This won't run");
*/
```

**Good Comment Examples:**
- `let max_retries = 3; // Prevent infinite retry loops`
- `// Convert Fahrenheit to Celsius using standard formula`
- `/// Calculates compound interest over time`

## Level 3: Implementation Hint

**Complete Comment Solutions**

**Explanatory Comments:**
```rust
// Check if temperature is within comfortable range (68-78°F)
let is_comfortable = temperature >= 68 && temperature <= 78;

// Convert Fahrenheit to Celsius using the standard formula
let celsius = (fahrenheit - 32.0) * 5.0 / 9.0;

// Calculate statistics for a sample array of numbers
let numbers = [1, 2, 3, 4, 5]; // Array of consecutive integers for testing
let sum: i32 = numbers.iter().sum(); // Sum all elements in the array
let average = sum as f64 / numbers.len() as f64; // Convert to f64 for decimal precision
```

**Commenting Out Code:**
```rust
// println!("This line should be commented out!");

/* This entire block is commented out for demonstration purposes
let unused_variable = 42;
println!("This should not print: {}", unused_variable);
let another_unused = "test";
println!("This should also not print: {}", another_unused);
*/
```

**Documentation Comments:**
```rust
/// Adds two integers and returns the result.
/// 
/// # Arguments
/// * `a` - The first integer
/// * `b` - The second integer
/// 
/// # Examples
/// ```
/// let result = add_numbers(2, 3);
/// assert_eq!(result, 5);
/// ```
fn add_numbers(a: i32, b: i32) -> i32 {
    // Perform simple addition and return the result
    a + b
}

/// Represents a person with basic contact information.
/// 
/// This struct stores essential data about a person including
/// their name, age, and email address for contact purposes.
struct Person {
    name: String,    // Person's full name
    age: u8,         // Person's age in years (0-255 range is sufficient)
    email: String,   // Person's email address for contact
}
```

**Inline Comments:**
```rust
let max_attempts = 3; // Limit retries to prevent infinite loops
let result = max_attempts * 2; // Double the max attempts for special cases
```

**Key Techniques:**
- Use `//` for single-line explanations
- Use `/* */` for multi-line explanations or disabling code
- Use `///` for function and struct documentation
- Keep comments concise but informative
- Explain the reasoning behind complex logic