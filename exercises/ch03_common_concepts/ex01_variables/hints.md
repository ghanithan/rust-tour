# Hints for Variables and Mutability

## Level 1: Conceptual Hint

This exercise is about understanding Rust's approach to **variable safety**. Rust prevents many common bugs by making variables immutable by default.

**Key concepts you need to understand:**

### Immutability by Default
- Variables in Rust are **immutable by default**
- This means once you assign a value, you can't change it
- This prevents accidental mutations that cause bugs

### The `mut` Keyword
- To make a variable changeable, use the `mut` keyword
- `let mut x = 5;` creates a mutable variable
- `let x = 5;` creates an immutable variable

### Shadowing vs Mutation
- **Shadowing**: Creating a new variable with the same name (`let x = x + 1;`)
- **Mutation**: Changing the value of a mutable variable (`x = x + 1;`)
- Shadowing can change the type, mutation cannot

**📖 Read:** [Rust Book 3.1 - Variables and Mutability](https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html)

**💭 Think about:** Why would making variables immutable by default help prevent bugs?

## Level 2: Strategic Hint

Look at each TODO comment and understand what needs to be implemented:

### Section 1: Variable Shadowing
```rust
let x = 5;
println!("The value of x is: {}", x);
// This should work but currently might cause an error
let x = 6;  // This is shadowing - creating a new variable with same name
println!("The value of x is: {}", x);
```

### Section 2: Mutable Variables
```rust
// TODO: Create a mutable variable called 'count' with initial value 0
// Then increment it by 1 and print the result
```
**Fix:** Create a mutable variable and demonstrate incrementing it.

### Section 3: Type Changing with Shadowing
```rust
let spaces = "   ";
println!("spaces: {}", spaces);
// TODO: Shadow 'spaces' with its length
let spaces = spaces.len();
println!("number of spaces: {}", spaces);
```

**Strategy:**
1. Read each TODO comment carefully
2. Understand the difference between mutation and shadowing
3. Implement each section step by step
4. Test with `cargo run` to see the output

## Level 3: Implementation Hint

Here are the specific implementations for each section:

### Section 1: Variable Shadowing (Already Working)
The shadowing example should already work:
```rust
let x = 5;
println!("The value of x is: {}", x);
let x = 6;  // This creates a new variable, doesn't mutate the old one
println!("The value of x is: {}", x);
```

### Section 2: Implement Mutable Variable
```rust
// Create a mutable variable
let mut count = 0;
println!("count: {}", count);

// Increment it
count += 1;  // or count = count + 1;
println!("count after increment: {}", count);
```

### Section 3: Type Conversion Through Shadowing (Already Working)
This should already work:
```rust
let spaces = "   ";
println!("spaces: {}", spaces);

let spaces = spaces.len();  // Shadow with new type (usize instead of &str)
println!("number of spaces: {}", spaces);
```

### Complete Working Program Output Should Show:
```
The value of x is: 5
The value of x is: 6
count: 0
count after increment: 1
spaces:    
number of spaces: 3
```

**Key Difference:**
- **Shadowing** (`let x = new_value`) creates a new variable, can change type
- **Mutation** (`x = new_value`) changes existing mutable variable, same type only

After implementing the mutable variable section, all tests should pass!