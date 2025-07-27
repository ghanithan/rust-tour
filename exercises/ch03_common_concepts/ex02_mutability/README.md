# Exercise 2: Advanced Mutability Concepts

## Learning Objectives

This exercise deepens your understanding of mutability in Rust:
- Working with mutable references
- Understanding borrowing rules
- Passing mutable parameters to functions
- The relationship between ownership and mutability

## Background

Mutability in Rust is more nuanced than just adding `mut` to variables. You also need to understand:

**Mutable References**: `&mut T` allows you to modify data through a reference
**Borrowing Rules**: Only one mutable reference OR multiple immutable references at a time
**Function Parameters**: Functions can take ownership, immutable references, or mutable references

## Rust Book References

Essential reading:
- [Chapter 4.2: References and Borrowing](https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html)
- [Chapter 3.1: Variables and Mutability](https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html)

## Your Task

The code in `src/main.rs` has compilation errors related to mutability. Fix these by:

1. **Making variables mutable** when they need to be modified
2. **Creating mutable references** when you need to modify through a reference  
3. **Fixing function signatures** to accept mutable references when needed

## Common Mistakes

- Forgetting to make the variable itself mutable (`let mut`)
- Using immutable references (`&T`) when you need mutable ones (`&mut T`)
- Not understanding that functions take ownership by default

## Testing

```bash
cargo run        # Should work after fixing errors
cargo test       # Run the automated tests
```

## Expected Output

```
=== Part 1: Mutable Variables ===
Initial temperature: 32°F
Updated temperature: 212°F

=== Part 2: References ===
Final score: 150

=== Part 3: Function Parameters ===
Counter after increment: 1
```

Remember: Rust's mutability rules prevent data races and ensure memory safety!