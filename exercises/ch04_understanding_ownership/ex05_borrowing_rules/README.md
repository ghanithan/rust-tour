# Exercise 05: Advanced Borrowing Rules - The Borrow Checker in Action

## Learning Objectives
- Understand how the borrow checker analyzes code
- Learn about reference scopes and when they end
- Practice with nested borrowing scenarios
- Understand conditional borrowing and complex patterns

## Introduction

The borrow checker is one of Rust's most sophisticated features. It analyzes your code to ensure memory safety without runtime overhead. This exercise presents several challenging scenarios that will deepen your understanding of how borrowing works in complex situations.

## Your Task

Analyze each scenario in the code and determine whether it will compile or not. For the ones that don't compile, fix them using the most appropriate technique.

The code contains several different borrowing patterns:
1. **Nested scopes** - How references interact across different scopes
2. **Conditional borrowing** - Borrowing inside if/else blocks
3. **Loop borrowing** - References in loops and iterations
4. **Return value borrowing** - Returning references from functions
5. **Struct field borrowing** - Partial borrowing of struct fields

## Expected Behavior

After fixing all issues, the program should demonstrate:
- Safe borrowing patterns that the borrow checker accepts
- Efficient memory usage without unnecessary clones
- Clear understanding of when references are valid

## Expected Output
```
=== Nested Scope Analysis ===
Processing data: [1, 2, 3, 4, 5]
Maximum value: 5
Final result: [2, 4, 6, 8, 10]

=== Conditional Borrowing ===
User alice has 1500 points
Alice is a premium user
Balance after transaction: 1450

=== Loop Borrowing ===
Processing: Hello
Processing: World
Processing: Rust
All strings processed: 3

=== Function Return Borrowing ===
Found longest word: Programming
Context still available: The Rust Programming Language

=== Struct Field Borrowing ===
Reading sensor: temperature = 23.5
Writing to actuator: Setting fan speed to 75%
System status: Active
```

## Running the Exercise
```bash
cargo run
```

## Testing Your Solution
```bash
cargo test
```