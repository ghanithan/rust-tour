//! Simple Calculator Program
//! 
//! TODO: Add a description of what this program does and what concepts it demonstrates

use std::io;

fn main() {
    println!("=== Simple Calculator ===");
    println!("Enter two numbers and choose an operation!");
    
    // TODO: Get the first number from user input
    // Hint: Create a helper function get_number() that handles input validation
    let num1 = 0.0; // Replace this with actual input
    
    // TODO: Get the second number from user input
    let num2 = 0.0; // Replace this with actual input
    
    // TODO: Ask user for operation (+, -, *, /)
    // Hint: Create a helper function get_operation() that validates the input
    let operation = String::new(); // Replace this with actual input
    
    // TODO: Perform the calculation using a match statement or if/else
    // Hint: Create a calculate() function that returns Result<f64, String>
    // to handle errors like division by zero
    
    // TODO: Display the result or error message
    println!("Result would go here");
    
    println!("Thank you for using the calculator!");
}

// TODO: Create helper function for getting a number from user input
// This should:
// - Print a prompt
// - Read user input
// - Parse the input to f64
// - Handle invalid input by asking again
// - Return the valid number
// 
// fn get_number(prompt: &str) -> f64 {
//     // Your implementation here
// }

// TODO: Create helper function for getting operation choice
// This should:
// - Print available operations
// - Read user input  
// - Validate it's one of: +, -, *, /
// - Handle invalid input by asking again
// - Return the valid operation as String
//
// fn get_operation() -> String {
//     // Your implementation here
// }

// TODO: Create calculation function that performs the math
// This should:
// - Take two numbers and an operation
// - Use match statement to call appropriate operation function
// - Return Result<f64, String> to handle errors
//
// fn calculate(num1: f64, num2: f64, operation: &str) -> Result<f64, String> {
//     // Your implementation here
// }

// TODO: Create individual operation functions
// These should be simple functions that take two f64 parameters and return f64
// Exception: divide() should return Result<f64, String> to handle division by zero
//
// fn add(a: f64, b: f64) -> f64 {
//     // Your implementation here
// }
//
// fn subtract(a: f64, b: f64) -> f64 {
//     // Your implementation here  
// }
//
// fn multiply(a: f64, b: f64) -> f64 {
//     // Your implementation here
// }
//
// fn divide(a: f64, b: f64) -> Result<f64, String> {
//     // Your implementation here
//     // Remember to check for division by zero!
// }