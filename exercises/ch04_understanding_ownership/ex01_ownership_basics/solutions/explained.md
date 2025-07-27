# Solution Explanation: Ownership Basics

## Understanding the Errors

This exercise demonstrates the fundamental ownership rules through common mistakes. Let's analyze each error:

### Error 1: Using a Moved Value
```rust
let s1 = String::from("Hello");
let s2 = s1;  // s1's ownership moves to s2
println!("First string: {}", s1);  // ERROR: s1 no longer owns the data
```

**Why this happens**: 
- String data lives on the heap
- Rust prevents double-free errors by allowing only one owner
- After `let s2 = s1;`, s1 is invalidated

**Solution**: Clone the data so each variable owns its own copy:
```rust
let s2 = s1.clone();
```

### Error 2: Using Values After Moving to Functions
```rust
let combined = combine_strings(s2, s3);  // s2 and s3 are moved
println!("Can I still use s2? {}", s2);  // ERROR: s2 was moved
```

**Why this happens**:
- Functions take ownership of their parameters (unless borrowing)
- Once moved, the original variables cannot be used

**Solution**: Either:
1. Clone before passing: `combine_strings(s2.clone(), s3)`
2. Remove the line trying to use s2 afterward
3. Use borrowing (covered in next exercises)

### Copy Types (No Error)
```rust
let x = 42;
let y = x;  // x is copied, not moved
println!("Integer value is still accessible: {}", x);  // This works!
```

**Why this works**:
- Simple types like i32, bool, char implement the Copy trait
- Assignment creates a bitwise copy instead of moving
- Both variables remain valid

### Ownership Transfer Pattern
The `takes_and_gives_back` function demonstrates a common pattern:
```rust
fn takes_and_gives_back(some_string: String) -> String {
    // Do something with some_string
    some_string  // Return ownership to caller
}
```

This pattern is cumbersome - that's why Rust has borrowing!

## Key Takeaways

1. **Move Semantics**: Non-Copy types (like String) are moved on assignment
2. **Function Ownership**: Functions take ownership of their non-Copy parameters
3. **Copy Trait**: Simple types are copied instead of moved
4. **Memory Safety**: These rules prevent double-frees and use-after-free bugs

## Alternative Solutions

### Using Borrowing (Preview)
```rust
fn combine_strings(s1: &String, s2: &String) -> String {
    format!("{}, {}!", s1, s2)
}
// Called as: combine_strings(&s2, &s3)
```

### Returning Multiple Values
```rust
fn combine_and_return(s1: String, s2: String) -> (String, String, String) {
    let combined = format!("{}, {}!", s1, s2);
    (s1, s2, combined)  // Return all three
}
```

These patterns will be explored in the borrowing exercises!