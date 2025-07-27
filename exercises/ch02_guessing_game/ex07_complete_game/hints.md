# Hints for Complete Guessing Game

## Level 1: Conceptual Hint

This is your capstone exercise where you demonstrate mastery of all Chapter 2 concepts by building a complete application.

**Understanding Application Design:**

**Game Flow Architecture:**
1. **Initialization**: Welcome message, setup
2. **Game Loop**: The main game until win
3. **Input Handling**: Get and validate user guesses
4. **Game Logic**: Compare guess to secret number
5. **Feedback**: Provide helpful responses
6. **Victory**: Celebrate and show statistics
7. **Replay**: Option to play again

**User Experience Principles:**
- **Clear Communication**: Users always know what to do next
- **Graceful Error Handling**: Errors don't frustrate or confuse
- **Encouraging Feedback**: Keep players engaged and motivated
- **Consistent Interface**: Predictable patterns throughout

**Code Organization:**
- **Separation of Concerns**: Each function has a clear purpose
- **Error Handling Strategy**: Consistent approach to all errors
- **Maintainable Structure**: Easy to understand and modify
- **Rust Best Practices**: Idiomatic code that leverages Rust's strengths

**Integration Points:**
You need to smoothly combine:
- Random number generation (once per game)
- Input reading (every guess)
- Parsing and validation (every guess)
- Comparison logic (every guess)
- Loop control (game flow and replay)
- Error handling (everywhere)

**Quality Indicators:**
- Program never panics under any input
- All error messages are helpful and user-friendly
- Game flow feels natural and engaging
- Code is clean and well-organized
- Edge cases are handled appropriately

**Key concepts to demonstrate:**
- How all Chapter 2 concepts work together
- Professional-quality error handling
- User-centered design thinking
- Rust's safety guarantees in practice

**📖 Reference:** Complete [Chapter 2 Tutorial](https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html) for comprehensive guidance

## Level 2: Strategic Hint

Break down the implementation into logical components:

**1. Program Structure:**
```rust
fn main() {
    print_welcome();
    
    loop {
        play_single_game();
        
        if !ask_play_again() {
            break;
        }
    }
    
    print_goodbye();
}
```

**2. Single Game Structure:**
```rust
fn play_single_game() {
    let secret_number = generate_secret_number();
    let mut attempts = 0;
    
    loop {
        attempts += 1;
        
        match get_valid_guess(attempts) {
            Some(guess) => {
                match compare_guess(guess, secret_number) {
                    GuessResult::Correct => {
                        celebrate_victory(guess, attempts);
                        break;
                    },
                    GuessResult::TooHigh => println!("Too high! Try a smaller number."),
                    GuessResult::TooLow => println!("Too low! Try a larger number."),
                }
            },
            None => {
                // Handle case where user wants to quit mid-game
                return;
            }
        }
    }
}
```

**3. Key Functions to Implement:**

**Input Handling:**
```rust
fn get_valid_guess(attempt_num: u32) -> Option<u32> {
    loop {
        print!("Attempt #{} - Enter your guess: ", attempt_num);
        
        let input = match read_line() {
            Ok(line) => line,
            Err(_) => {
                println!("Error reading input, please try again.");
                continue;
            }
        };
        
        if input.trim() == "quit" {
            return None; // Signal to quit
        }
        
        match parse_guess(&input) {
            Ok(guess) => return Some(guess),
            Err(msg) => {
                println!("Error: {}", msg);
                // Continue loop to try again
            }
        }
    }
}
```

**Comparison Logic:**
```rust
enum GuessResult {
    TooLow,
    TooHigh,
    Correct,
}

fn compare_guess(guess: u32, secret: u32) -> GuessResult {
    match guess.cmp(&secret) {
        Ordering::Less => GuessResult::TooLow,
        Ordering::Greater => GuessResult::TooHigh,
        Ordering::Equal => GuessResult::Correct,
    }
}
```

**Think about:**
- How to structure the code for readability
- Where to handle different types of errors
- How to make the user experience smooth
- What information to track across attempts

## Level 3: Implementation Hint

Here's a complete, production-quality implementation:

```rust
use std::io::{self, Write};
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    print_welcome();
    
    loop {
        play_single_game();
        
        if !ask_play_again() {
            break;
        }
        
        println!("\n🎲 New game started! I'm thinking of a new number...\n");
    }
    
    println!("Thanks for playing! Goodbye! 👋");
}

fn print_welcome() {
    println!("🎲 Welcome to the Number Guessing Game! 🎲");
    println!();
    println!("I'm thinking of a number between 1 and 100.");
    println!("Can you guess what it is?");
    println!("(Type 'quit' at any time to exit)");
    println!();
}

fn play_single_game() {
    let secret_number = rand::thread_rng().gen_range(1..=100);
    let mut attempts = 0;
    
    loop {
        attempts += 1;
        
        match get_valid_guess(attempts) {
            Some(guess) => {
                match compare_guess(guess, secret_number) {
                    GuessResult::Correct => {
                        celebrate_victory(guess, attempts);
                        break;
                    },
                    GuessResult::TooHigh => {
                        println!("Too high! Try a smaller number.\n");
                    },
                    GuessResult::TooLow => {
                        println!("Too low! Try a larger number.\n");
                    },
                }
            },
            None => {
                println!("Game quit. The number was {}.", secret_number);
                return; // User chose to quit
            }
        }
    }
}

#[derive(PartialEq)]
enum GuessResult {
    TooLow,
    TooHigh,
    Correct,
}

fn compare_guess(guess: u32, secret: u32) -> GuessResult {
    match guess.cmp(&secret) {
        Ordering::Less => GuessResult::TooLow,
        Ordering::Greater => GuessResult::TooHigh,
        Ordering::Equal => GuessResult::Correct,
    }
}

fn get_valid_guess(attempt_num: u32) -> Option<u32> {
    loop {
        print!("Attempt #{} - Enter your guess: ", attempt_num);
        io::stdout().flush().unwrap(); // Ensure prompt appears immediately
        
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {},
            Err(_) => {
                println!("Error reading input, please try again.");
                continue;
            }
        }
        
        let input = input.trim();
        
        // Handle quit command
        if input.eq_ignore_ascii_case("quit") {
            return None;
        }
        
        // Handle empty input
        if input.is_empty() {
            println!("Error: Please enter a number or 'quit'.\n");
            continue;
        }
        
        // Parse the number
        match input.parse::<u32>() {
            Ok(guess) => {
                if guess < 1 || guess > 100 {
                    println!("Error: Please enter a number between 1 and 100.\n");
                    continue;
                }
                return Some(guess);
            },
            Err(_) => {
                println!("Error: '{}' is not a valid number. Please enter a number between 1 and 100.\n", input);
                continue;
            }
        }
    }
}

fn celebrate_victory(guess: u32, attempts: u32) {
    println!("🎉 Congratulations! You guessed it correctly!");
    println!("You found the number {} in {} attempt{}.", 
             guess, 
             attempts, 
             if attempts == 1 { "" } else { "s" });
    
    // Add some encouraging messages based on performance
    match attempts {
        1 => println!("Incredible! You got it on your first try! 🎯"),
        2..=5 => println!("Excellent guessing! 🌟"),
        6..=10 => println!("Good job! 👍"),
        _ => println!("You got there in the end! 🎊"),
    }
    println!();
}

fn ask_play_again() -> bool {
    loop {
        print!("Would you like to play again? (y/n): ");
        io::stdout().flush().unwrap();
        
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {},
            Err(_) => {
                println!("Error reading input, please try again.");
                continue;
            }
        }
        
        match input.trim().to_lowercase().as_str() {
            "y" | "yes" => return true,
            "n" | "no" => return false,
            _ => {
                println!("Please enter 'y' for yes or 'n' for no.");
                continue;
            }
        }
    }
}
```

**Key implementation details:**

1. **Error Handling**: Every potential failure point is handled gracefully
2. **User Experience**: Clear prompts, helpful errors, encouraging feedback
3. **Code Organization**: Logical separation of concerns
4. **Edge Cases**: Empty input, out-of-range numbers, case-insensitive commands
5. **Performance Feedback**: Different messages based on number of attempts
6. **Input Validation**: Comprehensive checking with helpful error messages
7. **Game Flow**: Smooth transitions between states
8. **Professional Polish**: Emojis, proper formatting, attention to detail