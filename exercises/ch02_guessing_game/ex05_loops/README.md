# Exercise 2.5: Infinite Loops and Breaking

In this exercise, you'll learn how to create programs that repeat until certain conditions are met using Rust's `loop` keyword and `break` statement.

## 🎯 Learning Objectives

By completing this exercise, you will:
- Understand the `loop` keyword for creating infinite loops
- Learn how to use `break` to exit loops conditionally
- Practice `continue` to skip to the next iteration
- Combine loops with user input for interactive programs
- Create programs that run until users decide to quit
- Build the foundation for game programming patterns

## 📚 Background

Many programs need to repeat actions until certain conditions are met. Games, in particular, often run in loops until the player wins, loses, or chooses to quit.

Rust provides several loop constructs:
1. **`loop`**: Creates an infinite loop that runs until explicitly broken
2. **`while`**: Runs while a condition is true
3. **`for`**: Iterates over collections or ranges

The `loop` keyword is particularly useful when:
- You don't know how many iterations you'll need
- The exit condition is complex or checked in multiple places
- You want to ensure the loop body runs at least once

Key concepts:
- **Infinite loops**: Loops that would run forever without break conditions
- **Break conditions**: Logic that determines when to exit the loop
- **Continue statements**: Skip the rest of the current iteration
- **Loop labels**: Named loops for complex nested scenarios

## 📖 Rust Book References

Read these sections before starting:
- [Chapter 2: Allowing Multiple Guesses with Looping](https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html#allowing-multiple-guesses-with-looping) - Main content
- [Chapter 3.5: Control Flow - Repetition with Loops](https://doc.rust-lang.org/book/ch03-05-control-flow.html#repetition-with-loops) - Loop details

## ✅ Your Task

Create a number guessing practice program that uses loops to allow multiple attempts:

**Program requirements:**
1. **Generate a target number** between 1 and 10
2. **Use an infinite loop** to repeatedly ask for guesses
3. **Get user input** and parse it safely
4. **Compare the guess** to the target number
5. **Provide feedback** (too high, too low, or correct)
6. **Break the loop** when the user guesses correctly
7. **Count attempts** and show the total when done
8. **Handle invalid input** gracefully (use `continue` to retry)

Example interaction:
```
I'm thinking of a number between 1 and 10!
Attempt #1 - Enter your guess: 5
Too low! Try again.
Attempt #2 - Enter your guess: 8
Too high! Try again.
Attempt #3 - Enter your guess: 7
Correct! You guessed it in 3 attempts.
```

**Additional requirements:**
- Use `rand` crate to generate the target number
- Handle non-numeric input by showing an error and continuing the loop
- Keep track of attempt count
- Provide encouraging messages

## 🧪 Testing Your Solution

Run the tests to check your solution:
```bash
cargo test
```

Play your guessing game:
```bash
cargo run
```

Try different strategies: correct guesses, wrong guesses, and invalid input.

## 💡 Hints Available

Progressive hints are available if you need guidance:
1. **Conceptual**: Understanding loops, break, and continue
2. **Strategic**: Planning the loop structure and flow control
3. **Implementation**: Complete code with proper loop logic

## 🌟 Key Concepts

- **`loop` Keyword**: Creates an infinite loop that runs until `break`
- **`break` Statement**: Exits the loop immediately
- **`continue` Statement**: Skips to the next iteration
- **Loop Variables**: Variables that track state across iterations
- **Input Validation**: Handling invalid input without crashing
- **Game Loop Pattern**: Common pattern in interactive programs

## ⏭️ What's Next

After mastering loops, you'll learn about robust error handling with the `Result` type to make your programs more resilient to unexpected input.

You're building the core mechanics of interactive programs! 🔄