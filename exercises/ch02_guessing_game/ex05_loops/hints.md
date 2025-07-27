# Hints for Infinite Loops and Breaking

## Level 1: Conceptual Hint

This exercise teaches you about loop control flow, which is fundamental to creating interactive programs and games.

**Understanding Loops in Programming:**
- Most programs need to repeat actions until certain conditions are met
- Games particularly need "game loops" that run until the game ends
- User input programs often need to retry when input is invalid
- Loops provide elegant solutions to repetitive tasks

**Rust's Loop Constructs:**

**`loop` Keyword:**
- Creates an infinite loop that runs forever (until `break`)
- Best when you don't know how many iterations you'll need
- Clearer than `while true` for intentionally infinite loops
- Compiler optimizations work better with `loop`

**`break` Statement:**
- Immediately exits the loop
- Can be used anywhere inside the loop body
- Often used with conditional logic (`if` statements)
- Transfers control to the first statement after the loop

**`continue` Statement:**
- Skips the rest of the current iteration
- Jumps back to the beginning of the loop
- Useful for handling invalid input or special cases
- Continues the loop rather than exiting it

**Common Loop Patterns:**
```rust
// Infinite loop with conditional break
loop {
    // Get input
    // Process input
    if some_condition {
        break;  // Exit the loop
    }
    // More processing
}

// Input validation with continue
loop {
    // Get input
    if input_is_invalid {
        continue;  // Skip to next iteration
    }
    // Process valid input
    if done {
        break;  // Exit when finished
    }
}
```

**Key concepts to understand:**
- When to use infinite loops vs. conditional loops
- How `break` and `continue` affect program flow
- Why loop control is essential for interactive programs
- How to avoid accidental infinite loops

**📖 Read more:** [Rust Book Chapter 3.5 - Repetition with Loops](https://doc.rust-lang.org/book/ch03-05-control-flow.html#repetition-with-loops)

## Level 2: Strategic Hint

Your guessing game needs to be structured with careful loop control:

**Overall Program Structure:**
1. **Setup**: Generate target number, initialize attempt counter
2. **Game Loop**: Infinite loop that handles each guess attempt
3. **Input**: Get and validate user input inside the loop
4. **Processing**: Compare guess to target
5. **Feedback**: Tell user if guess is too high, low, or correct
6. **Exit Condition**: Break when guess is correct

**Key Loop Control Points:**

**Main Game Loop:**
```rust
loop {
    // Increment attempt counter
    // Get user input
    // Validate input (use continue if invalid)
    // Compare guess to target
    // Provide feedback
    // Break if correct
}
```

**Input Validation Pattern:**
```rust
let guess = loop {
    // Get input string
    // Try to parse it
    match result {
        Ok(num) => break num,  // Valid input, break with value
        Err(_) => {
            println!("Invalid input, try again");
            continue;  // Invalid input, try again
        }
    }
};
```

**Comparison and Feedback:**
```rust
match guess.cmp(&target) {
    Ordering::Less => println!("Too low!"),
    Ordering::Greater => println!("Too high!"),
    Ordering::Equal => {
        println!("Correct!");
        break;  // Exit the main game loop
    }
}
```

**Think about:**
- Where do you increment the attempt counter?
- How do you handle parsing errors without crashing?
- When exactly should you `break` vs. `continue`?
- How do you maintain state (like attempt count) across iterations?

## Level 3: Implementation Hint

Here's the complete solution with detailed explanations:

```rust
use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("I'm thinking of a number between 1 and 10!");
    
    // Generate target number
    let target = rand::thread_rng().gen_range(1..=10);
    let mut attempt_count = 0;
    
    // Main game loop
    loop {
        attempt_count += 1;
        print!("Attempt #{} - Enter your guess: ", attempt_count);
        
        // Get and validate input
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        
        // Parse input with error handling
        let guess: u32 = match input.trim().parse() {
            Ok(num) => {
                // Validate range
                if num < 1 || num > 10 {
                    println!("Please enter a number between 1 and 10.");
                    attempt_count -= 1;  // Don't count invalid attempts
                    continue;
                }
                num
            },
            Err(_) => {
                println!("Please enter a valid number.");
                attempt_count -= 1;  // Don't count invalid attempts
                continue;
            }
        };
        
        // Compare guess to target
        match guess.cmp(&target) {
            Ordering::Less => println!("Too low! Try again."),
            Ordering::Greater => println!("Too high! Try again."),
            Ordering::Equal => {
                println!("Correct! You guessed it in {} attempts.", attempt_count);
                break;  // Exit the game loop
            }
        }
    }
}
```

**Detailed explanation:**

1. **Setup:**
   - Generate random target with `rand::thread_rng().gen_range(1..=10)`
   - Initialize `attempt_count` to track attempts

2. **Main Game Loop:**
   - `loop` creates infinite loop
   - Increment `attempt_count` at start of each iteration
   - Prompt shows current attempt number

3. **Input Handling:**
   - Read input string as usual
   - Use `match` on `parse()` result to handle errors
   - `Ok(num)` for successful parsing
   - `Err(_)` for parsing failures

4. **Input Validation:**
   - Check if number is in valid range (1-10)
   - Use `continue` to retry for invalid input
   - Decrement `attempt_count` so invalid attempts don't count

5. **Comparison Logic:**
   - Use `match` on `guess.cmp(&target)` for clean comparison
   - Provide feedback for each case
   - `break` only when guess is correct

6. **Loop Control:**
   - `continue` skips to next iteration for invalid input
   - `break` exits loop when game is won
   - Attempt counter maintains state across iterations

**Alternative input validation approach:**
```rust
// Simpler version without range checking
let guess: u32 = match input.trim().parse() {
    Ok(num) => num,
    Err(_) => {
        println!("Invalid input!");
        continue;
    }
};
```

**Why this pattern is powerful:**
- Separates input validation from game logic
- Handles errors gracefully without crashing
- Provides clear feedback to users
- Maintains game state across iterations
- Easy to extend with additional features