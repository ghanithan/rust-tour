# Exercise 02: Move Semantics Deep Dive

## Learning Objectives
- Understand how moves work with collections (Vec)
- Practice ownership transfers between functions
- Learn when and why to return ownership
- Build intuition about heap-allocated data movement

## Introduction

Now that you understand basic ownership, let's explore how moves work in more complex scenarios. You'll build a `StringBuffer` that manages a collection of strings, demonstrating how ownership moves through different contexts.

## Your Task

Create a `StringBuffer` struct and implement the required methods. Pay careful attention to:
- When ownership is transferred
- When you need to return ownership
- How to handle collections of owned values

## Requirements

Implement the following:

```rust
struct StringBuffer {
    // Store a vector of strings
}

impl StringBuffer {
    // Create a new empty StringBuffer
    fn new() -> Self
    
    // Add a string to the buffer (taking ownership)
    fn add(&mut self, text: String)
    
    // Remove and return the last string (transferring ownership to caller)
    fn remove_last(&mut self) -> Option<String>
    
    // Combine all strings into one, clearing the buffer
    fn flush(&mut self) -> String
    
    // Get the number of strings currently in the buffer
    fn len(&self) -> usize
}
```

## Expected Behavior

Your implementation should pass all the tests, demonstrating:
1. Taking ownership of strings when adding
2. Returning ownership when removing
3. Proper handling of empty buffer cases
4. Clearing the buffer after flush

## Example Usage
```rust
let mut buffer = StringBuffer::new();
buffer.add(String::from("Hello"));
buffer.add(String::from("World"));

assert_eq!(buffer.len(), 2);

let combined = buffer.flush();
assert_eq!(combined, "HelloWorld");
assert_eq!(buffer.len(), 0);
```

## Running the Exercise
```bash
cargo run
```

## Testing Your Solution
```bash
cargo test
```