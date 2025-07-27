# Exercise 03: Struct Methods - Adding Behavior to Data

## Learning Objectives
- Implement methods on structs using `impl` blocks
- Understand different forms of `self` parameters (`self`, `&self`, `&mut self`)
- Use method call syntax vs function call syntax
- Create methods that read, modify, and consume struct instances
- Build a practical example with multiple related methods

## Introduction

Methods in Rust allow you to define functions that are associated with a particular type. Unlike functions, methods are called with method syntax: `instance.method()`. This makes code more readable and groups related functionality with the data it operates on.

In this exercise, you'll work with a `Rectangle` struct and implement various methods to calculate area, compare sizes, and modify dimensions. You'll learn when to use different forms of `self` and how method calls work with Rust's ownership system.

## Your Task

Complete the implementation in `src/main.rs` by filling in the missing method implementations. You'll need to:

1. Implement basic calculation methods
2. Create comparison methods  
3. Add mutation methods that modify the struct
4. Understand when to use `&self`, `&mut self`, and `self`

## Key Concepts

### Method Definition Syntax
```rust
impl StructName {
    fn method_name(&self, parameters) -> ReturnType {
        // method body
    }
}
```

### Different Forms of Self
- `&self`: Immutable borrow (read-only access)
- `&mut self`: Mutable borrow (can modify fields)
- `self`: Takes ownership (consumes the instance)

### Method Call Syntax
```rust
let result = instance.method(args);
// Equivalent to:
let result = StructName::method(&instance, args);
```

## Expected Output
```
Creating rectangles...
Rectangle: 30x50 (area: 1500)
Rectangle: 10x40 (area: 400)
Rectangle: 60x25 (area: 1500)

Testing area calculations...
rect1 area: 1500
rect2 area: 400

Testing comparisons...
rect1 can hold rect2: true
rect2 can hold rect1: false
rect1 has same area as rect3: true

Testing mutations...
Before scaling: Rectangle: 30x50 (area: 1500)
After scaling by 2: Rectangle: 60x100 (area: 6000)

Making square from rectangle...
Original rectangle was consumed to create square: 25x25 (area: 625)
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
- Level 1: Understanding methods and impl blocks
- Level 2: Method signatures and self parameters
- Level 3: Complete implementations with explanations