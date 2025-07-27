use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("I'm thinking of a number between 1 and 10!");
    
    // Generate a random target number between 1 and 10 (inclusive)
    let target = rand::thread_rng().gen_range(1..=10);
    let mut attempt_count = 0;
    
    // Create an infinite loop
    loop {
        // Increment the attempt counter at the start of each iteration
        attempt_count += 1;
        
        // Prompt the user for their guess (show attempt number)
        println!("Attempt #{} - Enter your guess:", attempt_count);
        
        // Read the user's input
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        
        // Parse the input to a number, handling parsing errors
        let guess: u32 = match input.trim().parse() {
            Ok(num) => {
                // Check if the number is in the valid range
                if num < 1 || num > 10 {
                    println!("Please enter a number between 1 and 10.");
                    attempt_count -= 1; // Don't count invalid attempts
                    continue;
                }
                num
            },
            Err(_) => {
                println!("Please enter a valid number.");
                attempt_count -= 1; // Don't count invalid attempts
                continue;
            }
        };
        
        // Compare the guess to the target number using match
        match guess.cmp(&target) {
            Ordering::Less => println!("Too low! Try again."),
            Ordering::Greater => println!("Too high! Try again."),
            Ordering::Equal => {
                println!("Correct! You guessed it in {} attempts.", attempt_count);
                break; // Exit the loop
            }
        }
    }
}