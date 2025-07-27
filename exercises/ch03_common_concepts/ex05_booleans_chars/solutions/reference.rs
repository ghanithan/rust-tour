// Reference solution for Boolean and Character Types

fn main() {
    println!("=== Boolean Type ===");
    
    let is_rust_fun = true;
    let is_learning = true;
    
    // Logical AND
    let both_true = is_rust_fun && is_learning;
    println!("Both are true: {}", both_true);
    
    // Logical OR
    let at_least_one = is_rust_fun || is_learning;
    println!("At least one true: {}", at_least_one);
    
    // Negation
    let not_fun = !is_rust_fun;
    println!("Not fun: {}", not_fun);
    
    // Comparisons
    let age = 25;
    let voting_age = 18;
    let can_vote = age >= voting_age;
    println!("Can vote: {}", can_vote);
    
    let temperature = 75;
    let comfortable = temperature >= 65 && temperature <= 80;
    println!("Comfortable temperature: {}", comfortable);
    
    println!("\n=== Character Type ===");
    
    let letter = 'A';
    let emoji = '🦀';
    println!("Letter: {}", letter);
    println!("Emoji: {}", emoji);
    
    let digit_char = '5';
    println!("Digit character: {}", digit_char);
    
    let space_char = ' ';
    println!("Space character: '{}'", space_char);
    
    let unicode_char = '中';
    println!("Unicode character: {}", unicode_char);
    
    // Character methods
    let is_alpha = letter.is_alphabetic();
    println!("'{}' is alphabetic: {}", letter, is_alpha);
    
    let is_digit = digit_char.is_numeric();
    println!("'{}' is numeric: {}", digit_char, is_digit);
    
    let lowercase = letter.to_lowercase().collect::<String>();
    println!("'{}' in lowercase: {}", letter, lowercase);
    
    let test_char = '7';
    let is_alphanumeric = test_char.is_alphabetic() || test_char.is_numeric();
    println!("'{}' is alphanumeric: {}", test_char, is_alphanumeric);
    
    let is_not_space = test_char != ' ';
    println!("'{}' is not a space: {}", test_char, is_not_space);
}