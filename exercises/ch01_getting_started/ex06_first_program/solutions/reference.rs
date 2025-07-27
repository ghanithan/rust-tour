//! Simple Calculator Program
//! 
//! This program demonstrates basic Rust concepts including functions,
//! variables, user input, string parsing, and error handling.

use std::io;

fn main() {
    println!("=== Simple Calculator ===");
    println!("Enter two numbers and choose an operation!");
    
    // Get user input for numbers and operation
    let num1 = get_number("Enter the first number:");
    let num2 = get_number("Enter the second number:");
    let operation = get_operation();
    
    // Perform calculation and display result
    match calculate(num1, num2, &operation) {
        Ok(result) => {
            println!("\nResult: {} {} {} = {}", num1, operation, num2, result);
        }
        Err(error) => {
            println!("\nError: {}", error);
        }
    }
    
    println!("Thank you for using the calculator!");
}

/// Gets a number from user input with validation and retry logic
fn get_number(prompt: &str) -> f64 {
    loop {
        println!("{}", prompt);
        let mut input = String::new();
        
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                match input.trim().parse::<f64>() {
                    Ok(number) => return number,
                    Err(_) => println!("Invalid input! Please enter a valid number."),
                }
            }
            Err(_) => println!("Failed to read input! Please try again."),
        }
    }
}

/// Gets operation choice from user with validation
fn get_operation() -> String {
    loop {
        println!("Enter operation (+, -, *, /):");
        let mut input = String::new();
        
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                let operation = input.trim();
                if ["+", "-", "*", "/"].contains(&operation) {
                    return operation.to_string();
                } else {
                    println!("Invalid operation! Please enter +, -, *, or /");
                }
            }
            Err(_) => println!("Failed to read input! Please try again."),
        }
    }
}

/// Performs the calculation based on the operation
fn calculate(num1: f64, num2: f64, operation: &str) -> Result<f64, String> {
    match operation {
        "+" => Ok(add(num1, num2)),
        "-" => Ok(subtract(num1, num2)),
        "*" => Ok(multiply(num1, num2)),
        "/" => divide(num1, num2),
        _ => Err(format!("Unknown operation: {}", operation)),
    }
}

/// Addition function
fn add(a: f64, b: f64) -> f64 {
    a + b
}

/// Subtraction function  
fn subtract(a: f64, b: f64) -> f64 {
    a - b
}

/// Multiplication function
fn multiply(a: f64, b: f64) -> f64 {
    a * b
}

/// Division function with zero-check
fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err("Cannot divide by zero!".to_string())
    } else {
        Ok(a / b)
    }
}