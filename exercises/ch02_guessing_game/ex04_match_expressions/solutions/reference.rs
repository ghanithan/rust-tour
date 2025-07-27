use std::io;
use std::cmp::Ordering;

fn main() {
    println!("Let's compare two numbers!");
    
    // Get first number
    println!("Enter the first number:");
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("Failed to read line");
    let number1: u32 = input1.trim().parse().expect("Please enter a valid number");
    
    // Get second number  
    println!("Enter the second number:");
    let mut input2 = String::new();
    io::stdin().read_line(&mut input2).expect("Failed to read line");
    let number2: u32 = input2.trim().parse().expect("Please enter a valid number");
    
    println!("Comparing {} with {}...", number1, number2);
    
    // Use match expression to handle all comparison cases
    match number1.cmp(&number2) {
        Ordering::Less => {
            println!("The first number ({}) is smaller than the second number ({}).", number1, number2);
        },
        Ordering::Greater => {
            println!("The first number ({}) is greater than the second number ({}).", number1, number2);
        },
        Ordering::Equal => {
            println!("Both numbers are equal! {} = {}", number1, number2);
        },
    }
}