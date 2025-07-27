# Exercise 2.4: Match Expressions and Pattern Matching

In this exercise, you'll learn about `match` expressions, one of Rust's most powerful and elegant control flow constructs for handling different cases.

## 🎯 Learning Objectives

By completing this exercise, you will:
- Understand `match` expressions and pattern matching
- Learn about the `Ordering` enum and comparison results
- Practice exhaustive pattern matching (covering all cases)
- Use `match` arms to handle different scenarios elegantly
- Compare `match` with `if-else` chains and understand when to use each

## 📚 Background

Pattern matching with `match` is a cornerstone of Rust programming. It allows you to compare a value against a series of patterns and execute code based on which pattern matches.

Key concepts:
1. **`match` Expression**: Tests a value against multiple patterns
2. **Patterns**: Templates that describe the shape of data
3. **Arms**: Each pattern and its associated code
4. **Exhaustiveness**: Must handle all possible cases
5. **`Ordering` Enum**: Result of comparing two values (Less, Greater, Equal)

Why `match` is powerful:
- **Exhaustive**: Compiler ensures you handle all cases
- **Readable**: Clear mapping between conditions and actions
- **Efficient**: Optimized by the compiler
- **Safe**: No possibility of unhandled cases

## 📖 Rust Book References

Read these sections before starting:
- [Chapter 2: Comparing the Guess](https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html#comparing-the-guess-to-the-secret-number) - Main content
- [Chapter 6.2: The match Control Flow Construct](https://doc.rust-lang.org/book/ch06-02-match.html) - Deep dive into match

## ✅ Your Task

Create a number comparison program that uses `match` expressions to provide different responses based on how two numbers compare:

**Program requirements:**
1. **Ask for two numbers** from the user
2. **Parse both inputs** safely
3. **Compare the numbers** using the `cmp()` method
4. **Use a `match` expression** to handle all three comparison results:
   - When first number is less than second
   - When first number is greater than second  
   - When both numbers are equal
5. **Provide descriptive messages** for each case

Example interaction:
```
Enter the first number: 25
Enter the second number: 30
Comparing 25 with 30...
The first number (25) is smaller than the second number (30).
```

**Key requirements:**
- Use `std::cmp::Ordering` for comparison results
- Use `number1.cmp(&number2)` to get the comparison result
- Handle all three `Ordering` variants: `Less`, `Greater`, `Equal`
- Provide meaningful messages for each case

## 🧪 Testing Your Solution

Run the tests to check your solution:
```bash
cargo test
```

Test your program with different number combinations:
```bash
cargo run
```

Try cases where first < second, first > second, and first == second.

## 💡 Hints Available

Progressive hints are available if you need help:
1. **Conceptual**: Understanding match expressions and pattern matching
2. **Strategic**: Planning the match structure and comparison logic
3. **Implementation**: Complete code with all match arms

## 🌟 Key Concepts

- **`match` Expression**: Compares a value against patterns and executes matching code
- **`Ordering` Enum**: Standard library enum for comparison results (`Less`, `Greater`, `Equal`)
- **Pattern Matching**: Rust's way of destructuring and testing data
- **Exhaustive Matching**: Compiler ensures all possible cases are handled
- **Match Arms**: Each `pattern => code` combination in a match

## ⏭️ What's Next

After mastering pattern matching, you'll learn about loops to create programs that can run repeatedly until certain conditions are met.

You're learning the elegant way to handle multiple conditions in Rust! 🔀