# Hints for Hello, World!

## Level 1: Conceptual Hint

This exercise is about understanding the basic structure of a Rust program. Every Rust program needs a `main` function, which is where the program starts executing.

The `println!` macro is Rust's way of printing text to the console. The exclamation mark (`!`) indicates that this is a macro, not a regular function.

**Key concepts to understand:**
- Functions in Rust are defined with the `fn` keyword
- The `main` function is special - it's where your program begins
- Macros like `println!` expand into more complex code at compile time
- String literals are enclosed in double quotes

**📖 Read more:** [Rust Book Chapter 1.2 - Hello, World!](https://doc.rust-lang.org/book/ch01-02-hello-world.html)

## Level 2: Strategic Hint

You need to use the `println!` macro inside the `main` function to print the text "Hello, world!".

The basic syntax for `println!` is:
```rust
println!("your text here");
```

Remember:
- The text goes inside double quotes
- Don't forget the semicolon at the end of the line
- Make sure the text matches exactly: "Hello, world!"

**Think about:** What goes between the parentheses of `println!()`?

## Level 3: Implementation Hint

Replace the TODO comment with this line:

```rust
println!("Hello, world!");
```

**Complete solution structure:**
```rust
fn main() {
    println!("Hello, world!");
}
```

**Explanation:**
- `fn main()` - Defines the main function
- `println!("Hello, world!");` - Prints the text followed by a newline
- The semicolon (`;`) ends the statement

After adding this line, save the file and run `cargo run` to see your program in action!