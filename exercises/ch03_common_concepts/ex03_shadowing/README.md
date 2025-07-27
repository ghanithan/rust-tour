# Exercise 3: Variable Shadowing

## Learning Objectives

Master variable shadowing in Rust:
- Understanding how shadowing creates new variables
- Type conversion through shadowing
- Scope-based shadowing behavior
- The difference between shadowing and mutation

## Background

Variable shadowing is a powerful feature in Rust that allows you to:

1. **Reuse variable names** by creating new variables that "shadow" previous ones
2. **Change types** while keeping the same variable name
3. **Transform data** step by step while maintaining readable code
4. **Work within scopes** where inner scopes can shadow outer variables

**Key Point**: Shadowing creates entirely new variables - it's not mutation!

## Rust Book References

Essential reading:
- [Chapter 3.1: Variables and Mutability](https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html) - Shadowing section
- [Chapter 3.2: Data Types](https://doc.rust-lang.org/book/ch03-02-data-types.html) - Type conversions

## Your Task

Complete the code in `src/main.rs` by filling in the missing parts:

1. **Numeric calculations** - Shadow variables with mathematical operations
2. **Type conversions** - Transform strings to numbers and back
3. **String processing** - Use methods like `.trim()` and `.parse()`
4. **Scope behavior** - Understand how shadowing works in different scopes

## Key Concepts

### Shadowing vs Mutation
```rust
// Shadowing - creates new variable
let x = 5;
let x = x + 1;    // New variable shadows old one

// Mutation - changes existing variable
let mut y = 5;
y = y + 1;        // Modifies the same variable
```

### Type Changes
```rust
let value = "42";        // &str
let value = value.parse::<i32>().unwrap();  // i32 - same name, different type!
```

## Testing

```bash
cargo run        # See your output
cargo test       # Run automated tests
```

## Expected Output

Your completed program should show transformations like:
- Circle area calculations
- String trimming and parsing
- Type conversions from string → number → float → string
- Scope-based shadowing behavior

## Success Criteria

- All code compiles without errors
- Variables are properly shadowed (not mutated)
- Type conversions work correctly
- Scope behavior is demonstrated