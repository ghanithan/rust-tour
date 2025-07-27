# Hints for Match Expressions and Pattern Matching

## Level 1: Conceptual Hint

This exercise introduces you to one of Rust's most distinctive and powerful features: pattern matching with `match` expressions.

**Understanding `match`:**
- `match` is like a supercharged `if-else` statement
- It compares a value against multiple patterns
- Each pattern is associated with code to run
- The compiler ensures you handle ALL possible cases (exhaustive matching)

**The `Ordering` Enum:**
- When you compare two values, you get an `Ordering` result
- `Ordering` has three variants: `Less`, `Greater`, `Equal`
- These represent all possible comparison outcomes
- `number1.cmp(&number2)` returns an `Ordering`

**Pattern Matching Benefits:**
- **Clarity**: Clear mapping between conditions and actions
- **Safety**: Compiler prevents missing cases
- **Performance**: Optimized by the compiler
- **Expressiveness**: Handles complex patterns elegantly

**Match vs. If-Else:**
```rust
// With if-else (verbose, error-prone)
if number1 < number2 {
    println!("Less");
} else if number1 > number2 {
    println!("Greater");  
} else {
    println!("Equal");
}

// With match (concise, guaranteed complete)
match number1.cmp(&number2) {
    Ordering::Less => println!("Less"),
    Ordering::Greater => println!("Greater"),
    Ordering::Equal => println!("Equal"),
}
```

**Key concepts to understand:**
- How pattern matching makes code more reliable
- Why exhaustive matching prevents bugs
- The relationship between enums and pattern matching
- How `cmp()` method works with references

**📖 Read more:** [Rust Book Chapter 6.2 - The match Control Flow Construct](https://doc.rust-lang.org/book/ch06-02-match.html)

## Level 2: Strategic Hint

Your program needs to be structured in these main parts:

**Part 1: Input Collection**
- Get two numbers from the user (similar to previous exercises)
- Parse both strings to numbers with proper error handling
- Use the same input reading and parsing patterns you've learned

**Part 2: Comparison Setup**
- Import `std::cmp::Ordering` at the top of your file
- Use the `cmp()` method to compare the numbers
- Store the comparison result in a variable

**Part 3: Match Expression**
- Create a `match` expression on the comparison result
- Handle all three `Ordering` variants:
  - `Ordering::Less` - first number is smaller
  - `Ordering::Greater` - first number is larger  
  - `Ordering::Equal` - numbers are the same

**Key patterns you'll need:**
```rust
use std::cmp::Ordering;

// Comparison pattern
let result = number1.cmp(&number2);

// Match pattern
match result {
    Ordering::Less => {
        // Code for when number1 < number2
    },
    Ordering::Greater => {
        // Code for when number1 > number2
    },
    Ordering::Equal => {
        // Code for when number1 == number2
    },
}
```

**Think about:**
- How do you import `Ordering` so you can use it?
- What does `&number2` mean in `number1.cmp(&number2)`?
- How can you make your match arms provide useful feedback?
- Why does Rust require you to handle all three cases?

## Level 3: Implementation Hint

Here's the complete solution with detailed explanations:

```rust
use std::io;
use std::cmp::Ordering;

fn main() {
    println!("Let's compare two numbers!");
    
    // Get first number
    println!("Enter the first number:");
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("Failed to read line");
    let number1: u32 = input1.trim().parse().expect("Please enter a valid number");
    
    // Get second number
    println!("Enter the second number:");
    let mut input2 = String::new();
    io::stdin().read_line(&mut input2).expect("Failed to read line");
    let number2: u32 = input2.trim().parse().expect("Please enter a valid number");
    
    // Compare and match
    println!("Comparing {} with {}...", number1, number2);
    
    match number1.cmp(&number2) {
        Ordering::Less => {
            println!("The first number ({}) is smaller than the second number ({}).", number1, number2);
        },
        Ordering::Greater => {
            println!("The first number ({}) is greater than the second number ({}).", number1, number2);
        },
        Ordering::Equal => {
            println!("Both numbers are equal! {} = {}", number1, number2);
        },
    }
}
```

**Detailed explanation:**

1. **Imports:**
   - `use std::io;` - For input operations
   - `use std::cmp::Ordering;` - For the comparison enum

2. **Input handling:**
   - Same pattern as previous exercises
   - Two separate input operations for two numbers
   - Parse each string to `u32` with error handling

3. **Comparison:**
   - `number1.cmp(&number2)` compares the numbers
   - `&number2` passes a reference (required by `cmp` method)
   - Returns an `Ordering` enum variant

4. **Match expression:**
   - `match` keyword followed by the value to match
   - Three arms for the three possible `Ordering` variants
   - Each arm has pattern `=>` code structure
   - Curly braces `{}` for multi-line code blocks

5. **Match arms:**
   - `Ordering::Less` - handles first < second case
   - `Ordering::Greater` - handles first > second case
   - `Ordering::Equal` - handles first == second case

**Alternative syntax (single-line arms):**
```rust
match number1.cmp(&number2) {
    Ordering::Less => println!("First is smaller"),
    Ordering::Greater => println!("First is greater"),
    Ordering::Equal => println!("Numbers are equal"),
}
```

**Why references matter:**
- `cmp()` takes `&self` and `&T` parameters
- `number1.cmp(&number2)` means "compare number1 with a reference to number2"
- This avoids moving ownership of `number2`