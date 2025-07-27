# Hints for Error Handling with Result

## Level 1: Conceptual Hint

This exercise teaches you the difference between "failing fast" (panicking) and "failing gracefully" (error handling).

**Understanding Error Handling Philosophy:**

**Panicking vs. Error Handling:**
- `expect()` causes panic = program crashes immediately
- `match` on `Result` = program continues and handles the error
- Panics are for unrecoverable errors (programming bugs)
- Result handling is for recoverable errors (user input, network, files)

**The Result Type:**
```rust
enum Result<T, E> {
    Ok(T),    // Success case with value of type T
    Err(E),   // Error case with error of type E
}
```

**Error Handling Strategies:**

**1. Match on Result:**
```rust
match some_operation() {
    Ok(value) => {
        // Use the successful value
    },
    Err(error) => {
        // Handle the error gracefully
    }
}
```

**2. Provide Fallbacks:**
- Use default values when operations fail
- Retry operations that might succeed later
- Offer alternative approaches

**3. User-Friendly Messages:**
- Explain what went wrong in simple terms
- Suggest what the user can do to fix it
- Avoid technical jargon and stack traces

**When to Use Each Approach:**
- `expect()`: When failure is a programming error
- `match Result`: When failure is expected and recoverable
- `unwrap()`: Only in examples or when you're certain of success
- `?` operator: When you want to propagate errors up the call stack

**Key concepts to understand:**
- Why most errors should be handled, not panicked
- How error handling improves user experience
- The difference between errors and bugs
- Why graceful degradation is important in real applications

**📖 Read more:** [Rust Book Chapter 9.2 - Recoverable Errors with Result](https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html)

## Level 2: Strategic Hint

You need to identify all the `expect()` calls in the provided code and replace them with proper error handling:

**Common expect() Patterns to Replace:**

**1. Input Reading:**
```rust
// Instead of this (panics on IO error):
io::stdin().read_line(&mut input).expect("Failed to read line");

// Use this (handles IO errors gracefully):
match io::stdin().read_line(&mut input) {
    Ok(_) => {
        // Successfully read input, continue processing
    },
    Err(error) => {
        println!("Error reading input: {}", error);
        // Handle the error (retry, use default, etc.)
    }
}
```

**2. String Parsing:**
```rust
// Instead of this (panics on invalid input):
let number: u32 = input.trim().parse().expect("Please enter a valid number");

// Use this (handles parsing errors gracefully):
let number: u32 = match input.trim().parse() {
    Ok(num) => num,
    Err(_) => {
        println!("Invalid number format. Please try again.");
        continue; // Or use a default value
    }
};
```

**3. File Operations:**
```rust
// Instead of this (panics if file doesn't exist):
let contents = fs::read_to_string("config.txt").expect("Failed to read config file");

// Use this (handles missing files gracefully):
let contents = match fs::read_to_string("config.txt") {
    Ok(content) => content,
    Err(_) => {
        println!("Config file not found, using defaults.");
        String::from("default config") // Fallback
    }
};
```

**Error Handling Strategies:**
- **Continue with defaults** when optional operations fail
- **Retry loops** for operations that might succeed later
- **Clear error messages** that help users understand what to do
- **Graceful degradation** - reduce functionality rather than crash

**Think about:**
- Which operations in the code might fail?
- What should happen when each type of error occurs?
- How can you provide helpful feedback to users?
- When should the program retry vs. continue vs. exit?

## Level 3: Implementation Hint

Here's how to systematically replace expect() calls with proper error handling:

**Complete error handling transformation:**

```rust
use std::io;
use std::fs;

fn main() {
    println!("Error Handling Demo");
    
    // 1. Handle file reading errors gracefully
    let config = match fs::read_to_string("config.txt") {
        Ok(contents) => {
            println!("Configuration loaded from file.");
            contents
        },
        Err(_) => {
            println!("Warning: Config file not found, using defaults.");
            String::from("default_value=42")
        }
    };
    
    // 2. Parse config with error handling
    let default_number = parse_config_number(&config);
    
    // 3. Main input loop with error handling
    loop {
        println!("Enter a number (or 'quit' to exit):");
        
        let mut input = String::new();
        
        // Handle input reading errors
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                // Successfully read input
            },
            Err(error) => {
                println!("Error reading input: {}. Please try again.", error);
                continue;
            }
        }
        
        let input = input.trim();
        
        // Handle quit command
        if input == "quit" {
            println!("Goodbye!");
            break;
        }
        
        // Handle number parsing with error recovery
        match input.parse::<u32>() {
            Ok(number) => {
                println!("You entered: {}", number);
                println!("Double that number: {}", number * 2);
                
                // Compare with config value
                if number > default_number {
                    println!("That's higher than the config default ({})!", default_number);
                } else {
                    println!("That's not higher than the config default ({}).", default_number);
                }
                break; // Exit after successful processing
            },
            Err(_) => {
                println!("Error: '{}' is not a valid number. Please enter a number or 'quit'.", input);
                // Continue the loop to try again
            }
        }
    }
}

fn parse_config_number(config: &str) -> u32 {
    // Parse a number from config string with fallback
    for line in config.lines() {
        if line.starts_with("default_value=") {
            let value_str = &line["default_value=".len()..];
            match value_str.parse::<u32>() {
                Ok(num) => {
                    println!("Using config default: {}", num);
                    return num;
                },
                Err(_) => {
                    println!("Invalid config value, using fallback default.");
                }
            }
        }
    }
    
    println!("No config value found, using fallback default: 50");
    50 // Ultimate fallback
}
```

**Key error handling patterns explained:**

1. **File Reading with Fallback:**
   - Try to read config file
   - If it fails, use default configuration
   - Continue program execution either way

2. **Input Reading with Retry:**
   - Try to read from stdin
   - If IO fails, show error and retry
   - Don't crash the program

3. **Parsing with Retry Loop:**
   - Try to parse user input
   - If parsing fails, show helpful error and retry
   - Allow user to quit gracefully

4. **Layered Fallbacks:**
   - Try to parse config file value
   - If that fails, use hardcoded default
   - Always have a working fallback

**Error Message Best Practices:**
- Be specific about what went wrong
- Suggest what the user can do
- Use friendly, non-technical language
- Provide examples when helpful

**Testing your error handling:**
- Try invalid number input
- Delete the config file and run
- Create an invalid config file
- Test the quit functionality
- Verify the program never panics