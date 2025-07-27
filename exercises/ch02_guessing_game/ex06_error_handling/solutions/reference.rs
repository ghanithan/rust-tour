use std::io;
use std::fs;

fn main() {
    println!("Error Handling Practice Program");
    
    // Handle file reading with graceful fallback
    let config = match fs::read_to_string("config.txt") {
        Ok(contents) => {
            println!("Configuration loaded from file.");
            contents
        },
        Err(_) => {
            println!("Warning: Config file not found, using defaults.");
            String::from("default_value=50")
        }
    };
    
    // Parse a default number from config with error handling
    let default_number = parse_config_number(&config);
    
    loop {
        println!("Enter a number (or 'quit' to exit):");
        
        let mut input = String::new();
        
        // Handle input reading errors gracefully
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                // Successfully read input, continue processing
            },
            Err(error) => {
                println!("Error reading input: {}. Please try again.", error);
                continue;
            }
        }
        
        let input = input.trim();
        
        if input == "quit" {
            println!("Goodbye!");
            break;
        }
        
        // Handle number parsing with graceful error recovery
        let number: u32 = match input.parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Error: '{}' is not a valid number. Please enter a number or 'quit'.", input);
                continue; // Continue the loop to try again
            }
        };
        
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
    // Parse a number from config string with multiple fallback levels
    for line in config.lines() {
        if line.starts_with("default_value=") {
            let value_str = &line["default_value=".len()..];
            match value_str.parse::<u32>() {
                Ok(num) => {
                    println!("Using config default: {}", num);
                    return num;
                },
                Err(_) => {
                    println!("Invalid config value format, using fallback default.");
                }
            }
        }
    }
    
    // Fallback if no valid config value found
    println!("No config value found, using fallback default: 50");
    50
}