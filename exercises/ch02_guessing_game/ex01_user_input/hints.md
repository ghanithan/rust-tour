# Hints for Reading User Input

## Level 1: Conceptual Hint

This exercise introduces you to interactive programming in Rust. The key concepts you need to understand:

**Standard Input (stdin):**
- Every program has access to "standard input" - where user types text
- In Rust, we access this through `std::io::stdin()`
- Reading input is an operation that might fail, so it returns a `Result`

**Mutable Variables:**
- By default, variables in Rust are immutable (cannot be changed)
- To store user input, you need a mutable variable using the `mut` keyword
- Syntax: `let mut variable_name = initial_value;`

**String Storage:**
- User input is text, so we need a `String` to store it
- `String::new()` creates a new, empty string ready for input

**Key concepts to understand:**
- Input operations in programming languages
- Why variables might need to be mutable
- How text input works in console programs
- Basic error handling concepts

**📖 Read more:** [Rust Book Chapter 2 - Programming a Guessing Game](https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html)

## Level 2: Strategic Hint

You need to structure your program in these steps:

1. **Import necessary modules**: You'll need `use std::io;` at the top
2. **Create a mutable string**: Use `String::new()` to create storage for input
3. **Prompt the user**: Use `println!` to ask for their name
4. **Read the input**: Use `stdin().read_line()` to get the user's input
5. **Handle the Result**: Input operations return `Result` - use `.expect()` for now
6. **Use the input**: Print a greeting with the user's name

**Key patterns you'll need:**
```rust
use std::io;  // Import at the top

let mut input = String::new();  // Mutable string for storage

stdin().read_line(&mut input).expect("Failed to read line");  // Reading pattern
```

**Think about:**
- Where does the input get stored?
- How do you tell Rust the variable can be changed?
- What happens if reading input fails?

## Level 3: Implementation Hint

Here's the complete solution with explanations:

```rust
use std::io;

fn main() {
    println!("Please enter your name:");
    
    let mut name = String::new();
    
    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read line");
    
    // Remove the newline character at the end
    let name = name.trim();
    
    println!("Hello, {}! Welcome to Rust programming.", name);
}
```

**Line-by-line explanation:**

1. `use std::io;` - Imports the input/output module
2. `println!("Please enter your name:");` - Prompts the user
3. `let mut name = String::new();` - Creates a mutable, empty string
4. `io::stdin()` - Gets a handle to standard input
5. `.read_line(&mut name)` - Reads a line into our mutable string
6. `.expect("Failed to read line");` - Handles potential errors
7. `let name = name.trim();` - Removes whitespace (including newline)
8. Final `println!` - Uses the name in a greeting

**Key details:**
- `&mut name` passes a mutable reference to the string
- `read_line` includes the newline character, so we `trim()` it
- `expect()` will panic with a message if reading fails
- The trimmed name can shadow the original mutable string