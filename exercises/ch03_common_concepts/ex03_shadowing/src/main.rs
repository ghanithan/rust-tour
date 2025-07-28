// Exercise: Variable Shadowing
//
// Variable shadowing allows you to declare a new variable with the same name 
// as a previous variable. The new variable "shadows" the previous one.
// This is different from mutation - you're creating entirely new variables.
//
// Complete the code below by filling in the missing parts.

fn main() {
    // Example: Basic shadowing
    let x = 5;
    println!("The value of x is: {}", x);
    
    let x = x + 1;  // This shadows the previous x
    println!("The value of x is: {}", x);
    
    // Part 1: Numeric calculations with shadowing
    let radius = 3;
    println!("Circle with radius: {}", radius);
    
    // TODO: Shadow radius with the area calculation (π * r²)
    // Use 3.14159 for π and shadow radius with the area
    let radius = 0.0; // TODO: complete this calculation
    println!("Circle area: {:.2}", radius);
    
    // Part 2: Type conversion through shadowing
    let user_input = "  42  ";
    println!("Raw input: '{}'", user_input);
    
    // TODO: Shadow user_input by trimming whitespace (use .trim())
    let user_input = ""; // TODO: trim the string
    println!("Trimmed input: '{}'", user_input);
    
    // TODO: Shadow user_input again by parsing to a number
    // Use .parse::<i32>().unwrap() to convert string to integer
    let user_input = 0; // TODO: parse to integer
    println!("Parsed number: {}", user_input);
    
    // Part 3: Multiple type changes
    let data = "123";
    println!("Original data: {} (type: &str)", data);
    
    // TODO: Convert to integer
    let data = 0; // TODO: parse to i32
    println!("As integer: {} (type: i32)", data);
    
    // TODO: Convert to float by multiplying by 1.5
    let data = 0.0; // TODO: convert to float calculation
    println!("As float: {} (type: f64)", data);
    
    // TODO: Convert back to string using .to_string()
    let data = String::new(); // TODO: convert to string
    println!("Back to string: {} (type: String)", data);
    
    // Part 4: Scope-based shadowing
    let message = "outer scope";
    println!("Outer message: {}", message);
    
    {
        // TODO: Shadow message with "inner scope" inside this block
        // TODO: create shadowed variable here
        println!("Inner message: {}", message);
    }
    
    // This should print the outer scope message again
    println!("Back to outer message: {}", message);
}

// Function to demonstrate shadowing doesn't affect function parameters
fn demonstrate_parameter_shadowing(value: i32) {
    println!("Parameter value: {}", value);
    
    // TODO: Shadow the parameter with value * 2
    // TODO: create shadowed parameter here
    println!("Shadowed value: {}", value);
}