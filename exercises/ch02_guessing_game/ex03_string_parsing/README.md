# Exercise 2.3: String Parsing and Type Conversion

In this exercise, you'll learn how to convert string input into numbers and handle potential parsing errors safely.

## 🎯 Learning Objectives

By completing this exercise, you will:
- Understand the difference between strings and numbers in Rust
- Learn how to use the `parse()` method for type conversion
- Practice type annotations to specify target types
- Handle parsing errors with `expect()` and understand when parsing fails
- Combine user input with numeric operations

## 📚 Background

When users enter data, it comes as text (strings), but programs often need to work with numbers. Converting between these types is called "parsing" or "type conversion."

Key challenges in parsing:
1. **Type Safety**: Rust prevents automatic conversion between types
2. **Error Handling**: Not all strings can be converted to numbers
3. **Type Inference**: Sometimes Rust needs help knowing what type you want
4. **Whitespace**: Input often contains extra whitespace that must be handled

Common parsing scenarios:
- User enters "42" → convert to number 42
- User enters "abc" → parsing fails, need to handle error
- User enters " 123 " → need to trim whitespace first

## 📖 Rust Book References

Read these sections before starting:
- [Chapter 2: Programming a Guessing Game - Comparing Numbers](https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html#comparing-the-guess-to-the-secret-number) - Main content
- [Chapter 3.2: Data Types](https://doc.rust-lang.org/book/ch03-02-data-types.html) - Understanding types

## ✅ Your Task

The provided code has several bugs related to string parsing and type conversion. Your job is to fix them:

**The program should:**
1. **Prompt the user** for a number
2. **Read the input** as a string
3. **Convert the string to a number** properly
4. **Perform a calculation** with the number
5. **Display the result** clearly

**Current bugs to fix:**
- String concatenation instead of numeric addition
- Missing type annotations for parsing
- Improper handling of whitespace
- Incorrect error handling

Example working interaction:
```
Enter a number: 25
You entered: 25
Double the number: 50
Is it greater than 30? true
```

## 🧪 Testing Your Solution

Run the tests to check your fixes:
```bash
cargo test
```

Test your program interactively:
```bash
cargo run
```

Try different inputs including numbers, invalid text, and numbers with spaces.

## 💡 Hints Available

Progressive hints are available if you get stuck:
1. **Conceptual**: Understanding string parsing and type conversion
2. **Strategic**: Identifying the bugs and planning fixes
3. **Implementation**: Specific code corrections and patterns

## 🌟 Key Concepts

- **Type Conversion**: Using `parse()` to convert strings to numbers
- **Type Annotations**: Telling Rust what type you want (e.g., `: u32`)
- **Error Handling**: Using `expect()` to handle parsing failures
- **String Methods**: Using `trim()` to remove whitespace
- **Type Safety**: Rust prevents mixing strings and numbers incorrectly

## ⏭️ What's Next

After mastering string parsing, you'll learn about pattern matching with `match` expressions to handle different comparison results elegantly.

You're building the skills to handle user input robustly! 🔢