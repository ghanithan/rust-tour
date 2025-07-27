# Hints for Advanced Mutability

## Level 1: Conceptual Hint

Understanding Rust's mutability system requires grasping these key concepts:

**1. Variable Mutability**
- Variables are immutable by default
- Use `let mut` to make a variable mutable
- Only mutable variables can be modified

**2. Reference Mutability**
- `&T` is an immutable reference (you can read, not write)
- `&mut T` is a mutable reference (you can read and write)
- You can only create `&mut T` from a mutable variable

**3. Borrowing Rules**
- At any time, you can have either:
  - One mutable reference, OR
  - Any number of immutable references
- References must always be valid

**4. Function Parameters**
- `fn foo(x: T)` takes ownership of `x`
- `fn foo(x: &T)` borrows `x` immutably
- `fn foo(x: &mut T)` borrows `x` mutably

📖 **Essential Reading:** [Rust Book Chapter 4.2 - References and Borrowing](https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html)

## Level 2: Strategic Hint

Look at each error and understand what needs to be fixed:

**Part 1: Mutable Variables**
- The error says you can't assign to `temperature` twice
- Solution: Make the variable mutable with `mut`

**Part 2: Mutable References**
- You're trying to modify through a reference
- The reference needs to be mutable: `&mut`
- Remember: you can only create `&mut` from a mutable variable

**Part 3: Function Parameters**
- The function takes ownership of the parameter
- But ownership means the original variable isn't affected
- You need to pass a mutable reference instead

**Strategy:**
1. Identify what needs to be modified
2. Make sure the variable is mutable (`let mut`)
3. Use mutable references (`&mut`) when modifying through references
4. Update function signatures to accept references when needed

## Level 3: Implementation Hint

Here are the specific fixes for each part:

**Part 1 Fix:**
```rust
let mut temperature = 32;  // Add 'mut'
```

**Part 2 Fix:**
```rust
let score_ref = &mut score;  // Change to mutable reference
```

**Part 3 Fix:**
```rust
// Change the function signature:
fn increment_counter(counter: &mut i32) {
    *counter += 1;  // Dereference to modify the value
}

// And call it with a mutable reference:
increment_counter(&mut counter);
```

**Complete Working Code:**
```rust
fn main() {
    // Part 1: Mutable variables
    let mut temperature = 32;  // Made mutable
    println!("Initial temperature: {}°F", temperature);
    temperature = 212;
    println!("Updated temperature: {}°F", temperature);
    
    // Part 2: Mutable references
    let mut score = 100;
    let score_ref = &mut score;  // Mutable reference
    *score_ref += 50;
    println!("Final score: {}", score);
    
    // Part 3: Function with mutable reference
    let mut counter = 0;
    increment_counter(&mut counter);  // Pass mutable reference
    println!("Counter after increment: {}", counter);
}

fn increment_counter(counter: &mut i32) {  // Accept mutable reference
    *counter += 1;
}
```

**Key Points:**
- `let mut` makes the variable mutable
- `&mut` creates a mutable reference
- `*` dereferences to access/modify the value
- Function signatures must match how you call them