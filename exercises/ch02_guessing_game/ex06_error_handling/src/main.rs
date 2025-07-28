use std::io;
use std::fs;

fn main() {
    println!("Error Handling Practice Program");
    
    // TODO: This code currently uses expect() which causes panics.
    // Replace all expect() calls with proper error handling using match.
    
    // BUG 1: This will panic if config.txt doesn't exist
    let config = fs::read_to_string("config.txt").expect("Failed to read config file");
    
    println!("Configuration loaded: {}", config);
    
    // Parse a default number from config (with more potential panics)
    let default_number = parse_config_number(&config);
    
    loop {
        println!("Enter a number (or 'quit' to exit):");
        
        let mut input = String::new();
        
        // BUG 2: This will panic if there's an IO error reading input
        io::stdin().read_line(&mut input).expect("Failed to read line");
        
        let input = input.trim();
        
        if input == "quit" {
            println!("Goodbye!");
            break;
        }
        
        // BUG 3: This will panic if the user enters invalid input
        let number: u32 = input.parse().expect("Please enter a valid number");
        
        println!("You entered: {}", number);
        println!("Double that number: {}", number * 2);
        
        if number > default_number {
            println!("That's higher than the config default ({})!", default_number);
        } else {
            println!("That's not higher than the config default ({}).", default_number);
        }
        break;
    }
}

fn parse_config_number(config: &str) -> u32 {
    // This function also has potential panics to fix
    for line in config.lines() {
        if line.starts_with("default_value=") {
            let value_str = &line["default_value=".len()..];
            // BUG 4: This will panic if the config value is invalid
            return value_str.parse().expect("Invalid config value");
        }
    }
    // BUG 5: This panics if no default_value is found in config
    panic!("No default_value found in config");
}

// TODO: Fix all the bugs above by:
// 1. Replace expect() calls with match statements
// 2. Provide fallback values when operations fail
// 3. Show helpful error messages to users
// 4. Continue program execution instead of panicking
// 5. Create a config.txt file with "default_value=50" for testing,
//    but make sure the program works even if the file doesn't exist