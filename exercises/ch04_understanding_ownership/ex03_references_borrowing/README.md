# Exercise 03: References and Borrowing - Read Without Taking

## Learning Objectives
- Understand what references are and how they differ from ownership
- Learn the syntax for creating and using references (&)
- Practice passing references to functions without moving values
- Understand the concept of "borrowing"

## Introduction

Move semantics are powerful but sometimes cumbersome. What if you want to use a value in a function without taking ownership? That's where references come in!

A reference is like a pointer to data that someone else owns. You can read the data, but you don't own it. This is called "borrowing" because you're borrowing access to the data temporarily.

## Your Task

Complete the functions that use references instead of taking ownership. You'll see how references solve many of the ownership challenges from previous exercises.

## Key Concepts

- **Reference**: Created with `&value`, points to data without owning it
- **Borrowing**: The act of creating a reference to use someone else's data
- **Immutable Reference**: `&T` - allows reading but not modifying
- **Ampersand (&)**: The reference operator

## Expected Output
```
Original string: Hello, Rust!
String length: 12
First word: Hello
Still available: Hello, Rust!
Book info: The Rust Programming Language by Steve Klabnik
Analysis: Title 'The Rust Programming Language' has 30 characters
Book title is still available: The Rust Programming Language
```

## Running the Exercise
```bash
cargo run
```

## Testing Your Solution
```bash
cargo test
```