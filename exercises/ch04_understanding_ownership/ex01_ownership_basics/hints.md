# Hints for Ownership Basics - The Foundation

## Level 1: Conceptual Hint

Understanding ownership is crucial to writing Rust code. Let's think about what's happening:

- **Ownership Transfer**: When you assign a String from one variable to another (like `let s2 = s1;`), the ownership moves. The original variable can no longer be used.

- **Why Does This Happen?**: Rust prevents double-free errors. If both s1 and s2 pointed to the same String data, when they go out of scope, Rust would try to free the same memory twice - a serious bug!

- **Stack vs Heap**: Simple types like integers are stored on the stack and implement the Copy trait. They're copied instead of moved. Complex types like String store data on the heap and are moved to prevent multiple owners.

Think about each error:
1. After `let s2 = s1;`, who owns the string data?
2. When you pass a String to a function, who gets ownership?
3. How can you use a value after passing it to a function?

Review Rust Book section 4.1 to understand the ownership rules in depth.

## Level 2: Strategic Hint

Here's how to approach fixing each part:

**Part 1 - Basic moves**: 
- You can't use `s1` after moving it to `s2`
- Solution pattern: Either clone the data or restructure your code
- Consider: `let s2 = s1.clone();` or just use `s2` instead of `s1`

**Part 2 - Function ownership**:
- Functions take ownership of their parameters (unless borrowing)
- After calling `combine_strings(s2, s3)`, both s2 and s3 are gone
- Pattern: Don't try to use moved values, or clone before passing

**Part 3 - Copy types**:
- This part actually works! Integers implement Copy
- No changes needed here - it demonstrates the difference

**Part 4 - Ownership transfer**:
- This part is correct - it shows taking and returning ownership
- The commented line would error if uncommented

Code structure hint:
```rust
// Instead of:
let s2 = s1;
println!("{}", s1);  // Error!

// Try:
let s2 = s1.clone();
println!("{}", s1);  // OK!
// Or just use s2 instead
```

## Level 3: Implementation Hint

Here's the complete solution with explanations:

```rust
fn main() {
    // Part 1: Basic move semantics
    let s1 = String::from("Hello");
    let s2 = s1.clone();  // Clone s1 so both variables have their own String
    
    println!("First string: {}", s1);  // Now s1 is still valid
    println!("Second string: {}", s2);
    
    // Part 2: Function calls and ownership
    let s3 = String::from("World");
    let combined = combine_strings(s2.clone(), s3);  // Clone s2 to keep using it
    
    println!("Combined: {}", combined);
    // Remove this line - s2 was moved even with the original code
    // Or change to use the cloned version
    
    // Part 3: Copy types behave differently
    let x = 42;
    let y = x;  // x is copied, not moved
    
    println!("Integer value is still accessible: {}", x);
    
    // Part 4: Taking and returning ownership
    let s4 = String::from("Rust");
    let s5 = takes_and_gives_back(s4);
    
    println!("After taking ownership: {}", s5);
    println!("String was moved and is no longer accessible");
}
```

Alternative approach for Part 2 - just remove the line trying to use s2:
```rust
// Simply remove this line:
// println!("Can I still use s2? {}", s2);
```

The key insight: Rust's ownership system prevents memory bugs by ensuring each piece of data has exactly one owner. When that owner goes out of scope, the memory is automatically freed. No double-frees, no use-after-free, no data races!