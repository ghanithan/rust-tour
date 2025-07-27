# Exercise 04: Mutable References - Controlled Modification

## Learning Objectives
- Understand mutable references (`&mut T`)
- Learn the borrowing rules that prevent data races
- Practice fixing borrowing checker errors
- Understand why Rust allows only one mutable reference at a time

## Introduction

Sometimes you need to modify data through a reference. Rust provides mutable references (`&mut T`) for this purpose, but with strict rules to prevent data races and ensure memory safety.

This exercise contains several borrowing violations. Your task is to fix them while understanding WHY Rust enforces these rules.

## The Borrowing Rules

Rust enforces these rules at compile time:
1. At any given time, you can have EITHER one mutable reference OR any number of immutable references
2. References must always be valid (no dangling references)

These rules prevent data races and ensure memory safety!

## Your Task

Fix all the borrowing errors in the code. Each error teaches an important lesson about Rust's borrowing system.

## Expected Output
```
Original value: 42
After increment: 43
Processing item: 1
Processing item: 2
Processing item: 3
Final list: [2, 4, 6]
Account balance: $1000
After deposit: $1150
After withdrawal: $1050
Transaction history: [Deposit(150), Withdrawal(100)]
```

## Running the Exercise
```bash
cargo run
```

## Testing Your Solution
```bash
cargo test
```