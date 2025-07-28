// Exercise: Boolean and Character Types
//
// Learn about Rust's bool and char types - essential building blocks
// for conditions and text processing.

fn main() {
    println!("=== Boolean Type ===");
    
    // Part 1: Basic boolean usage
    let is_rust_fun = true;
    let is_learning = true;
    
    // TODO: Create a boolean that represents "both are true"
    let both_true = /* complete logical AND operation */;
    println!("Both are true: {}", both_true);
    
    // TODO: Create a boolean for "at least one is true"  
    let at_least_one = /* complete logical OR operation */;
    println!("At least one true: {}", at_least_one);
    
    // TODO: Negate the is_rust_fun boolean
    let not_fun = /* complete negation */;
    println!("Not fun: {}", not_fun);
    
    // Part 2: Boolean from comparisons
    let age = 25;
    let voting_age = 18;
    
    // TODO: Check if age is greater than or equal to voting age
    let can_vote = /* complete comparison */;
    println!("Can vote: {}", can_vote);
    
    let temperature = 75;
    // TODO: Check if temperature is between 65 and 80 (inclusive)
    let comfortable = /* complete range check */;
    println!("Comfortable temperature: {}", comfortable);
    
    println!("\n=== Character Type ===");
    
    // Part 3: Basic character usage
    let letter = 'A';
    let emoji = '🦀';  // Rust crab!
    
    println!("Letter: {}", letter);
    println!("Emoji: {}", emoji);
    
    // TODO: Create a character variable for a digit
    let digit_char = /* assign a digit character like '5' */;
    println!("Digit character: {}", digit_char);
    
    // TODO: Create a character for a space
    let space_char = /* assign a space character */;
    println!("Space character: '{}'", space_char);
    
    // Part 4: Character methods and Unicode
    let unicode_char = '中';  // Chinese character
    println!("Unicode character: {}", unicode_char);
    
    // TODO: Check if a character is alphabetic using .is_alphabetic()
    let is_alpha = /* check if letter is alphabetic */;
    println!("'{}' is alphabetic: {}", letter, is_alpha);
    
    // TODO: Check if a character is numeric using .is_numeric()
    let is_digit = /* check if digit_char is numeric */;
    println!("'{}' is numeric: {}", digit_char, is_digit);
    
    // TODO: Convert character to lowercase using .to_lowercase()
    // Note: to_lowercase() returns an iterator, so collect to string
    let lowercase = /* convert letter to lowercase and collect to String */;
    println!("'{}' in lowercase: {}", letter, lowercase);
    
    // Part 5: Combining booleans and characters
    let test_char = '7';
    
    // TODO: Check if character is both alphabetic OR numeric
    let is_alphanumeric = /* check if alphabetic OR numeric */;
    println!("'{}' is alphanumeric: {}", test_char, is_alphanumeric);
    
    // TODO: Check if character is NOT a space
    let is_not_space = /* check if NOT equal to space */;
    println!("'{}' is not a space: {}", test_char, is_not_space);
}