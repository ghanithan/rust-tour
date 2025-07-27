# Exercise 01: Ownership Basics - The Foundation

## Learning Objectives
- Understand Rust's three ownership rules
- Learn what happens when values are moved
- Distinguish between stack and heap allocated data
- Fix common ownership compilation errors

## Introduction

Ownership is Rust's most unique feature and has deep implications for the rest of the language. It enables Rust to make memory safety guarantees without needing a garbage collector.

In this exercise, you'll encounter several common ownership mistakes that new Rust programmers make. Your task is to fix these errors and understand WHY they occur.

## The Three Rules of Ownership

Remember these fundamental rules:
1. Each value in Rust has an owner
2. There can only be one owner at a time
3. When the owner goes out of scope, the value will be dropped

## Your Task

The code in `src/main.rs` contains several ownership-related bugs. Your goal is to:
1. Fix all compilation errors
2. Ensure the program runs and produces the expected output
3. Understand why each error occurred

The program should demonstrate:
- String ownership and moves
- The difference between Copy types (like integers) and non-Copy types (like String)
- How function calls transfer ownership
- Basic ownership patterns

## Expected Output
```
First string: Hello
Second string: World
Combined: Hello, World!
Integer value is still accessible: 42
After taking ownership: Rust
String was moved and is no longer accessible
```

## Running the Exercise
```bash
cargo run
```

## Testing Your Solution
```bash
cargo test
```