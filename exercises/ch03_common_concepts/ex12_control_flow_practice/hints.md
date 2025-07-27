# Hints for Control Flow Practice

## Level 1: Conceptual Hint

**Building a Complete Game with Control Flow**

This exercise combines all control flow concepts in a practical application:

**Game Structure:**
- Main loop processes each guess
- Conditional logic compares guesses
- Early return on correct guess
- Helper functions for game mechanics

**Key Components:**
- **Loop**: Process multiple guesses
- **Conditionals**: Check guess vs secret number
- **Early Returns**: Exit on win/lose
- **Functions**: Modular game logic
- **Boolean Logic**: Complex condition checking

**Game Flow:**
1. Initialize game state
2. Loop through guesses
3. Compare each guess to secret number
4. Provide feedback (too high/low/correct)
5. Track attempts and enforce limits
6. Handle win/lose conditions

**Rust Book References:**
- [Control Flow](https://doc.rust-lang.org/book/ch03-05-control-flow.html)
- [Programming a Guessing Game](https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html)

## Level 2: Strategic Hint

**Implementation Strategies**

**For Main Game Logic:**
```rust
for guess in guesses.iter() {
    attempts += 1;
    
    if guess == secret_number {
        // handle win
        return;
    } else if guess < secret_number {
        // too low
    } else {
        // too high
    }
    
    if attempts >= max_attempts {
        // handle loss
        return;
    }
}
```

**For Helper Functions:**
```rust
fn categorize_guess(guess: i32, secret: i32) -> &'static str {
    let difference = (guess - secret).abs();
    // return based on difference
}

fn is_valid_guess(guess: i32) -> bool {
    guess >= 1 && guess <= 100
}
```

**For Calculations:**
```rust
let remaining = max_attempts - attempts;
let difference = (guess - secret).abs();
```

**Common Patterns:**
- Use `==` for exact equality
- Use `<` and `>` for comparisons
- Use `>=` for limit checking
- Use `return` for early exit
- Use `.abs()` for absolute difference

## Level 3: Implementation Hint

**Complete Solutions**

**Main Game Logic:**
```rust
if *guess == secret_number {
    println!("🎉 Congratulations! You guessed it!");
    println!("You won in {} attempts!", attempts);
    return;
} else if *guess < secret_number {
    println!("📈 Too low! Try a higher number.");
} else {
    println!("📉 Too high! Try a lower number.");
}

if attempts >= max_attempts {
    println!("💔 Game over! You've used all {} attempts.", max_attempts);
    println!("The secret number was: {}", secret_number);
    return;
}

let remaining = max_attempts - attempts;
```

**Helper Function Implementations:**
```rust
fn categorize_guess(guess: i32, secret: i32) -> &'static str {
    let difference = (guess - secret).abs();
    
    if difference == 0 {
        "Perfect!"
    } else if difference <= 5 {
        "Very close!"
    } else if difference <= 15 {
        "Close!"
    } else {
        "Not close"
    }
}

fn is_valid_guess(guess: i32) -> bool {
    guess >= 1 && guess <= 100
}
```

**Calculation Examples:**
```rust
// Remaining attempts
let remaining = max_attempts - attempts;

// Absolute difference
let difference = (guess - secret).abs();

// Score calculation logic is already provided
```

**Key Techniques:**
- Use `*guess` to dereference iterator values
- Use `return` to exit early from main function
- Use `.abs()` for absolute value calculation
- Use `>=` and `<=` for range checking
- Use early returns to avoid deeply nested conditionals
- Structure if-else chains for clear decision logic