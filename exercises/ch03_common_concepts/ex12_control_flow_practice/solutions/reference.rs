// Reference solution for Control Flow Practice

fn main() {
    println!("=== Number Guessing Game ===");
    
    let secret_number = 42;
    let mut attempts = 0;
    let max_attempts = 5;
    
    println!("I'm thinking of a number between 1 and 100!");
    println!("You have {} attempts to guess it.", max_attempts);
    
    let guesses = [25, 60, 40, 42];
    
    for guess in guesses.iter() {
        attempts += 1;
        println!("\nAttempt {}: Your guess is {}", attempts, guess);
        
        if !is_valid_guess(*guess) {
            println!("❌ Invalid guess! Please guess between 1 and 100.");
            continue;
        }
        
        let category = categorize_guess(*guess, secret_number);
        println!("Hint: {}", category);
        
        if *guess == secret_number {
            println!("🎉 Congratulations! You guessed it!");
            let score = calculate_score(attempts, max_attempts);
            println!("You won in {} attempts! Score: {}", attempts, score);
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
        if remaining > 0 {
            println!("You have {} attempts left.", remaining);
        }
    }
}

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