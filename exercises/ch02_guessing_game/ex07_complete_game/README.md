# Exercise 2.7: Complete Guessing Game

Congratulations! You've learned all the fundamental concepts. Now it's time to put everything together and build a complete, polished guessing game that demonstrates mastery of Chapter 2 concepts.

## 🎯 Learning Objectives

By completing this exercise, you will:
- Integrate all Chapter 2 concepts into a cohesive application
- Practice software design and user experience considerations
- Build a complete, production-ready program from scratch
- Demonstrate mastery of Rust fundamentals through practical application
- Create something fun and interactive that showcases your skills

## 📚 Background

This is your capstone exercise for Chapter 2. You'll build a complete guessing game that combines:

1. **User Input** - Reading from stdin
2. **Random Numbers** - Using external crates
3. **String Parsing** - Converting user input to numbers
4. **Pattern Matching** - Using match expressions for comparisons
5. **Loops** - Creating a game loop that runs until completion
6. **Error Handling** - Graceful handling of all potential errors

The game should be:
- **Robust**: Handle all edge cases without crashing
- **User-Friendly**: Provide clear instructions and helpful feedback
- **Engaging**: Fun to play with good game mechanics
- **Professional**: Clean code that follows Rust best practices

## 📖 Rust Book References

This exercise integrates the entire Chapter 2 tutorial:
- [Chapter 2: Programming a Guessing Game](https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html) - Complete tutorial

## ✅ Your Task

Build a complete number guessing game from scratch with these specifications:

### Core Game Mechanics
1. **Generate a secret number** between 1 and 100
2. **Allow unlimited guesses** until the player wins
3. **Provide feedback** for each guess (too high, too low, correct)
4. **Track attempt count** and display it
5. **Handle all input gracefully** without crashing

### User Experience Requirements
1. **Welcome message** explaining the game
2. **Clear prompts** for user input
3. **Helpful error messages** for invalid input
4. **Encouraging feedback** throughout the game
5. **Victory celebration** when the player wins
6. **Option to play again** after each game

### Technical Requirements
1. **Use all Chapter 2 concepts**:
   - `std::io` for input
   - `rand` crate for random numbers
   - String parsing with error handling
   - `match` expressions for comparisons
   - `loop` for game control
   - Proper error handling (no `expect()` calls)

2. **Handle edge cases**:
   - Invalid input (non-numbers, negative numbers, numbers out of range)
   - IO errors
   - Empty input
   - Very large numbers

3. **Code quality**:
   - Clean, readable code
   - Appropriate comments
   - Good variable names
   - Logical structure

### Example Game Session
```
🎲 Welcome to the Number Guessing Game! 🎲

I'm thinking of a number between 1 and 100.
Can you guess what it is?

Attempt #1 - Enter your guess: 50
Too high! Try a smaller number.

Attempt #2 - Enter your guess: 25
Too low! Try a larger number.

Attempt #3 - Enter your guess: abc
Error: Please enter a valid number between 1 and 100.

Attempt #3 - Enter your guess: 37
🎉 Congratulations! You guessed it correctly!
You found the number 37 in 3 attempts.

Would you like to play again? (y/n): y

🎲 New game started! I'm thinking of a new number...
```

## 🧪 Testing Your Solution

Run the tests to verify completeness:
```bash
cargo test
```

Play your game extensively:
```bash
cargo run
```

Test all edge cases: invalid input, out-of-range numbers, play again functionality.

## 💡 Hints Available

Progressive hints are available if you need guidance:
1. **Conceptual**: Planning the overall game structure and flow
2. **Strategic**: Breaking down the implementation into manageable parts
3. **Implementation**: Complete code structure with all features

## 🌟 Key Concepts Integration

Your game should demonstrate:
- **`std::io`**: Reading user input safely
- **`rand` crate**: Generating random numbers
- **String parsing**: Converting input with error handling
- **`match` expressions**: Comparing guesses elegantly
- **`loop` control**: Managing game flow and replay
- **Error handling**: Graceful recovery from all errors
- **User experience**: Clear communication and feedback

## ⏭️ What's Next

After completing this comprehensive game, you'll move on to advanced features and enhancements that demonstrate even more sophisticated Rust programming techniques.

You're building a complete, real application! 🚀