# Hints for Variables and Mutability

## Level 1: Conceptual Hint

This exercise explores three fundamental concepts in Rust's variable system:

**1. Variable Shadowing**
- You can declare a new variable with the same name as a previous variable
- The new variable "shadows" the previous one
- This is different from mutation - you're creating a new variable
- Shadowing can even change the type of the value

**2. Type Conversion through Shadowing**
- Since shadowing creates a new variable, you can change types
- Example: `let x = "hello"; let x = x.len();` changes from &str to usize
- The `.len()` method returns the length of a string

**3. Mutable Variables**
- By default, variables are immutable (can't be changed)
- Add `mut` keyword to make a variable mutable: `let mut x = 5;`
- Only mutable variables can be modified with operations like `+=`

**📖 Essential Reading:** [Rust Book Chapter 3.1 - Variables and Mutability](https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html)

**Key Question:** What's the difference between creating a new variable (shadowing) and changing an existing one (mutation)?

## Level 2: Strategic Hint

Let's break down what needs to be done for each TODO:

**TODO 1: Shadow the variable x**
- You have `let x = 5;` already defined
- Create a new variable with the same name that equals `x + 1`
- Syntax: `let variable_name = expression;`
- This demonstrates shadowing - creating a new variable

**TODO 2: Shadow 'spaces' with its length**
- The variable `spaces` contains a string with 3 spaces
- Use the `.len()` method to get the number of characters
- Shadow the variable by creating a new one: `let spaces = spaces.len();`
- This shows how shadowing can change types (from &str to usize)

**TODO 3: Make count mutable**
- The variable `count` needs to be modified later
- Add the `mut` keyword after `let`
- Syntax: `let mut variable_name = value;`
- This allows the `count += 1` operation to work

**Key Strategy:**
- Shadowing uses `let` to create a new variable
- Mutation requires `mut` and modifies the existing variable
- Test your changes with `cargo run` after each fix

## Level 3: Implementation Hint

Here are the exact solutions for each TODO:

**TODO 1 Solution:**
```rust
let x = x + 1;
```
This creates a new variable `x` with value 6, shadowing the previous `x`.

**TODO 2 Solution:**
```rust
let spaces = spaces.len();
```
This shadows the string variable with a number (its length). The type changes from `&str` to `usize`.

**TODO 3 Solution:**
```rust
let mut count = 0;
```
Adding `mut` makes the variable mutable, allowing the `count += 1` operation.

**Complete working code:**
```rust
fn main() {
    // Part 1: Variable Shadowing
    let x = 5;
    println!("The value of x is: {}", x);
    
    let x = x + 1;  // Shadow x
    println!("The value of x is: {}", x);
    
    // Part 2: Type Conversion through Shadowing
    let spaces = "   ";
    println!("spaces contains: '{}'", spaces);
    
    let spaces = spaces.len();  // Shadow with different type
    println!("Number of spaces: {}", spaces);
    
    // Part 3: Mutable Variables
    let mut count = 0;  // Make it mutable
    println!("Initial count: {}", count);
    
    count += 1;
    println!("Updated count: {}", count);
}
```

**Expected output:**
```
The value of x is: 5
The value of x is: 6
spaces contains: '   '
Number of spaces: 3
Initial count: 0
Updated count: 1
```

**Remember:** Shadowing creates new variables, while mutation changes existing ones!