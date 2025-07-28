// Exercise: Control Flow Practice
//
// This comprehensive exercise combines all control flow concepts:
// if/else, loops, functions, and boolean logic.
// Implement a simple number guessing game from scratch.

fn main() {
    println!("=== Number Guessing Game ===");
    
    let secret_number = 42;
    let mut attempts = 0;
    let max_attempts = 5;
    
    println!("I'm thinking of a number between 1 and 100!");
    println!("You have {} attempts to guess it.", max_attempts);
    
    // Simulate user guesses for testing
    let guesses = [25, 60, 40, 42]; // Last guess is correct
    
    // TODO: Implement the game loop
    for guess in guesses.iter() {
        attempts += 1;
        println!("\nAttempt {}: Your guess is {}", attempts, guess);
        
        // TODO: Check the guess and provide feedback
        if /* guess is correct */ {
            println!("🎉 Congratulations! You guessed it!");
            println!("You won in {} attempts!", attempts);
            return; // Exit the game
        } else if /* guess is too low */ {
            println!("📈 Too low! Try a higher number.");
        } else {
            println!("📉 Too high! Try a lower number.");
        }
        
        // TODO: Check if max attempts reached
        if /* max attempts reached */ {
            println!("💔 Game over! You've used all {} attempts.", max_attempts);
            println!("The secret number was: {}", secret_number);
            return;
        }
        
        // TODO: Show remaining attempts
        let remaining = /* calculate remaining attempts */;
        if remaining > 0 {
            println!("You have {} attempts left.", remaining);
        }
    }
}

// TODO: Implement a function to categorize the guess
fn categorize_guess(guess: i32, secret: i32) -> &'static str {
    let difference = /* calculate absolute difference */;
    
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

// TODO: Implement a function to check if number is in valid range
fn is_valid_guess(guess: i32) -> bool {
    /* check if guess is between 1 and 100 inclusive */
}

// TODO: Implement a function to calculate score based on attempts
fn calculate_score(attempts: i32, max_attempts: i32) -> i32 {
    if attempts <= 1 {
        100
    } else if attempts <= 2 {
        80
    } else if attempts <= 3 {
        60
    } else if attempts <= max_attempts {
        40
    } else {
        0
    }
}