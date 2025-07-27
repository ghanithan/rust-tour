# Exercise 01: Struct Definition - Building Your First Data Structure

## Learning Objectives
- Define custom structs with named fields
- Instantiate structs with field values
- Access and modify struct fields
- Understand struct ownership and moves

## Introduction

Structs are Rust's way of creating custom data types that group related data together. Unlike tuples, structs let you name each piece of data so it's clear what the values mean. Structs are fundamental to writing idiomatic Rust code and organizing your data effectively.

In this exercise, you'll work with a `User` struct to manage user account information. You'll learn the basic syntax for defining structs, creating instances, and working with their fields.

## Your Task

Complete the implementation in `src/main.rs` by filling in the missing parts marked with `// TODO:`. The program should:

1. Define a `User` struct with appropriate fields
2. Create instances of the User struct
3. Access and display user information
4. Demonstrate struct ownership behavior

## Key Concepts

### Struct Definition Syntax
```rust
struct StructName {
    field1: Type1,
    field2: Type2,
    // more fields...
}
```

### Struct Instantiation
```rust
let instance = StructName {
    field1: value1,
    field2: value2,
};
```

### Field Access
```rust
let value = instance.field1;
instance.field2 = new_value; // if instance is mutable
```

## Expected Output
```
Creating users...
User: alice@example.com (Alice Smith) - Active: true, Sign-in count: 1
User: bob@example.com (Bob Jones) - Active: false, Sign-in count: 0
After transferring alice...
Bob's email: bob@example.com
Alice was moved and is no longer accessible
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
Use the hint system if you get stuck:
- Level 1: Conceptual explanation of structs
- Level 2: Syntax guidance and patterns
- Level 3: Complete implementation with explanations