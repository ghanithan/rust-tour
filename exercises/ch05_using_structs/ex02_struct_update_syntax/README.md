# Exercise 02: Struct Update Syntax - Efficient Struct Creation

## Learning Objectives
- Use struct update syntax to create instances from existing ones
- Apply field init shorthand for cleaner code
- Understand ownership implications of struct updates
- Fix common errors with struct creation patterns

## Introduction

When working with structs, you'll often want to create new instances that are similar to existing ones, or create instances where variable names match field names. Rust provides convenient syntax for both scenarios:

1. **Struct Update Syntax**: Create new instances by specifying some fields and copying the rest from another instance
2. **Field Init Shorthand**: When variable names match field names, you can use shorthand syntax

This exercise contains several bugs related to these concepts. Your task is to fix them while learning these important Rust patterns.

## Your Task

The code in `src/main.rs` contains compilation errors and logic bugs related to:
- Improper use of struct update syntax
- Missing field init shorthand opportunities
- Ownership violations when updating structs
- Incorrect syntax for creating struct instances

Fix all the bugs to make the program compile and run correctly.

## Key Concepts

### Struct Update Syntax
```rust
let instance2 = SomeStruct {
    field1: new_value,
    ..instance1  // Copy remaining fields from instance1
};
```

### Field Init Shorthand
```rust
let name = String::from("Alice");
let age = 30;

// Instead of:
let person = Person { name: name, age: age };

// You can write:
let person = Person { name, age };
```

### Ownership with Updates
When using struct update syntax, fields that don't implement `Copy` will be moved from the source struct.

## Expected Output
```
Creating original user...
Original user: alice@example.com (Alice Smith) - Active: true, Sign-in count: 1

Creating updated user with struct update syntax...
Updated user: alice@example.com (Alice Johnson) - Active: true, Sign-in count: 1

Creating user with field init shorthand...
Shorthand user: bob@example.com (Bob Wilson) - Active: false, Sign-in count: 0

Creating admin from user template...
Admin user: admin@company.com (Alice Johnson) - Active: true, Sign-in count: 100

Demonstrating ownership after update...
Template user email: alice@example.com
Admin was created using update syntax
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
- Level 1: Understanding struct update syntax and field shorthand
- Level 2: Syntax patterns and ownership considerations  
- Level 3: Complete bug fixes with detailed explanations