# Exercise 06: The Slice Type - Views Into Data

## Learning Objectives
- Understand what slices are and why they're useful
- Learn string slice (`&str`) and array slice (`&[T]`) syntax
- Practice creating slices from owned data
- Understand the relationship between slices and ownership

## Introduction

Slices are a powerful feature that let you reference a contiguous sequence of elements in a collection rather than the whole collection. They're a kind of reference, so they don't have ownership.

This exercise will teach you how to work with:
- **String slices** (`&str`) - views into String or string literals
- **Array slices** (`&[T]`) - views into arrays or vectors
- **Slice ranges** - `[start..end]`, `[..end]`, `[start..]`, `[..]`

## Your Task

Complete the functions that work with slices. Learn how slices provide efficient access to parts of data without needing to copy or take ownership.

## Key Concepts

- **Slice**: A reference to a contiguous sequence of elements
- **String slice** (`&str`): A view into string data
- **Array slice** (`&[T]`): A view into array/vector data  
- **Range syntax**: `[start..end]` creates a slice from index start to end-1

## Expected Output
```
=== String Slices ===
Original: The quick brown fox jumps over the lazy dog
First word: The
Last word: dog
Middle section: quick brown fox jumps over the lazy
Words starting with 'o': over

=== Array Slices ===
Numbers: [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
First half: [1, 2, 3, 4, 5]
Last half: [6, 7, 8, 9, 10]
Middle section: [3, 4, 5, 6, 7, 8]
Even numbers found: [2, 4, 6, 8, 10]

=== Practical Examples ===
File extension: txt
Domain: example.com
Safe substring: Hello
Chunk 1: [1, 2, 3]
Chunk 2: [4, 5, 6]
Chunk 3: [7, 8, 9]
Remaining: [10]
```

## Running the Exercise
```bash
cargo run
```

## Testing Your Solution
```bash
cargo test
```