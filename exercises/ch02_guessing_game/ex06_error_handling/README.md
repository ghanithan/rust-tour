# Exercise 2.6: Error Handling with Result

In this exercise, you'll learn how to handle errors gracefully using Rust's `Result` type instead of panicking with `expect()`. This is crucial for creating robust, production-ready programs.

## 🎯 Learning Objectives

By completing this exercise, you will:
- Understand the `Result<T, E>` type and its `Ok` and `Err` variants
- Learn to use `match` expressions with `Result` for error handling
- Practice graceful error recovery instead of program termination
- Distinguish between recoverable and unrecoverable errors
- Build resilient programs that handle unexpected input elegantly
- Understand when to use `expect()` vs. proper error handling

## 📚 Background

So far, we've used `expect()` to handle potential errors, but this causes the program to panic (crash) when errors occur. In real applications, we usually want to handle errors gracefully and continue running.

Rust's `Result` type is an enum with two variants:
- `Ok(value)` - Contains the successful result
- `Err(error)` - Contains error information

Key concepts:
1. **Recoverable vs. Unrecoverable Errors**: Some errors can be handled, others should crash
2. **Graceful Degradation**: Continuing operation despite errors
3. **User Experience**: Providing helpful error messages without crashing
4. **Error Propagation**: Letting calling code decide how to handle errors

Why proper error handling matters:
- **Robustness**: Programs don't crash on unexpected input
- **User Experience**: Friendly error messages instead of panic traces
- **Debugging**: Better information about what went wrong
- **Production Readiness**: Servers and applications stay running

## 📖 Rust Book References

Read these sections before starting:
- [Chapter 2: Handling Invalid Input](https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html#handling-invalid-input) - Main content
- [Chapter 9.2: Recoverable Errors with Result](https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html) - Deep dive into Result

## ✅ Your Task

Transform a panic-prone program into one that handles errors gracefully:

**The provided code has several `expect()` calls that cause panics. Your job is to:**

1. **Replace `expect()` with proper `match` statements** on `Result` types
2. **Handle input/output errors** gracefully (file reading, stdin operations)
3. **Handle parsing errors** without crashing (invalid number input)
4. **Provide helpful error messages** to guide users
5. **Continue program execution** when possible instead of terminating
6. **Use a loop** to retry operations when appropriate

**Program should:**
- Read numbers from user input without panicking on invalid input
- Attempt to read from a file, handling missing files gracefully
- Parse configuration values with fallback defaults
- Provide clear error messages for each type of failure
- Allow users to retry failed operations

Example interaction (with error handling):
```
Enter a number: abc
Error: Invalid number format. Please enter a valid number.
Enter a number: 42
You entered: 42

Reading configuration file...
Warning: Config file not found, using defaults.
Configuration loaded successfully.
```

## 🧪 Testing Your Solution

Run the tests to check your error handling:
```bash
cargo test
```

Test your program with various inputs:
```bash
cargo run
```

Try invalid numbers, missing files, and other error conditions.

## 💡 Hints Available

Progressive hints are available if you need guidance:
1. **Conceptual**: Understanding Result, Ok, Err, and error handling philosophy
2. **Strategic**: Planning where and how to replace expect() calls
3. **Implementation**: Complete code with proper error handling patterns

## 🌟 Key Concepts

- **`Result<T, E>` Type**: Enum representing either success (`Ok`) or failure (`Err`)
- **Pattern Matching on Results**: Using `match` to handle both success and error cases
- **Graceful Error Recovery**: Continuing program execution despite errors
- **Error Messages**: Providing helpful feedback without technical panic traces
- **Fallback Strategies**: Using default values when operations fail
- **Retry Logic**: Allowing users to try again after errors

## ⏭️ What's Next

After mastering error handling, you'll put everything together to build a complete, robust guessing game that demonstrates all the concepts you've learned.

You're building the reliability that production software demands! 🛡️