// Reference solution for Variable Shadowing
//
// This solution demonstrates proper use of variable shadowing
// for type conversions, calculations, and scope management.

fn main() {
    // Example: Basic shadowing
    let x = 5;
    println!("The value of x is: {}", x);
    
    let x = x + 1;  // This shadows the previous x
    println!("The value of x is: {}", x);
    
    // Part 1: Numeric calculations with shadowing
    let radius = 3;
    println!("Circle with radius: {}", radius);
    
    // Shadow radius with the area calculation (π * r²)
    let radius = 3.14159 * (radius as f64) * (radius as f64);
    println!("Circle area: {:.2}", radius);
    
    // Part 2: Type conversion through shadowing
    let user_input = "  42  ";
    println!("Raw input: '{}'", user_input);
    
    // Shadow user_input by trimming whitespace
    let user_input = user_input.trim();
    println!("Trimmed input: '{}'", user_input);
    
    // Shadow user_input again by parsing to a number
    let user_input = user_input.parse::<i32>().unwrap();
    println!("Parsed number: {}", user_input);
    
    // Part 3: Multiple type changes
    let data = "123";
    println!("Original data: {} (type: &str)", data);
    
    // Convert to integer
    let data = data.parse::<i32>().unwrap();
    println!("As integer: {} (type: i32)", data);
    
    // Convert to float by multiplying by 1.5
    let data = (data as f64) * 1.5;
    println!("As float: {} (type: f64)", data);
    
    // Convert back to string
    let data = data.to_string();
    println!("Back to string: {} (type: String)", data);
    
    // Part 4: Scope-based shadowing
    let message = "outer scope";
    println!("Outer message: {}", message);
    
    {
        // Shadow message with "inner scope" inside this block
        let message = "inner scope";
        println!("Inner message: {}", message);
    }
    
    // This prints the outer scope message again
    println!("Back to outer message: {}", message);
    
    // Demonstrate function parameter shadowing
    demonstrate_parameter_shadowing(10);
}

// Function to demonstrate shadowing doesn't affect function parameters
fn demonstrate_parameter_shadowing(value: i32) {
    println!("Parameter value: {}", value);
    
    // Shadow the parameter with value * 2
    let value = value * 2;
    println!("Shadowed value: {}", value);
}