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