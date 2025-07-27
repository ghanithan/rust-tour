# Exercise 04: Associated Functions - Constructor Patterns

## Learning Objectives
- Understand the difference between methods and associated functions
- Implement constructor patterns using associated functions
- Use the `::` syntax to call associated functions
- Create multiple constructors for different use cases
- Build validation and convenience constructors

## Introduction

Associated functions are functions that are associated with a struct but don't take `self` as a parameter. They're often used as constructors or factory methods to create new instances of the struct. The most common associated function is `new`, which serves as a constructor.

Unlike methods (which are called with `.` syntax), associated functions are called using the `::` syntax, like `String::from()` or `Vec::new()`.

In this exercise, you'll create a `Person` struct with multiple associated functions that serve as different constructors and factory methods.

## Your Task

Complete the implementation in `src/main.rs` by implementing the required associated functions. You'll need to:

1. Create a basic `new` constructor
2. Implement factory methods for common scenarios
3. Add validation constructors that can fail
4. Build convenience constructors with default values

## Key Concepts

### Associated Function Syntax
```rust
impl StructName {
    fn function_name(parameters) -> ReturnType {
        // No self parameter
        // Called with StructName::function_name()
    }
}
```

### Constructor Patterns
- `new()`: Standard constructor
- `default()`: Creates instance with default values  
- `from_*()`: Creates instance from other types
- `with_*()`: Creates instance with specific configuration

### Method vs Associated Function
- **Method**: Takes `self`, called with `instance.method()`
- **Associated Function**: No `self`, called with `Type::function()`

## Expected Output
```
Creating people with different constructors...

Basic constructor:
Person: Alice Johnson (25 years old, alice@email.com)

Child constructor:
Person: Bobby Smith (8 years old, no email)

Adult constructor:
Person: Carol Davis (30 years old, carol@email.com)

Validation constructor (valid):
Person: David Wilson (45 years old, david@email.com)

Validation constructor (invalid age):
Error creating person: Age must be between 0 and 150

Validation constructor (invalid email):
Error creating person: Email must contain @ symbol

From name constructor:
Person: Eve Brown (0 years old, no email)

Testing methods on created instances...
Alice is an adult: true
Bobby is a child: true
Carol can vote: true
David can vote: true
Eve is a child: true
```

## Running the Exercise
```bash
cargo run
```

## Testing Your Solution  
```bash
cargo test
```

## Hints Available
- Level 1: Understanding associated functions and constructor patterns
- Level 2: Implementation strategies and validation techniques
- Level 3: Complete implementations with error handling