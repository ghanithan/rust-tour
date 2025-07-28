use std::io;
// TODO: Import the Ordering enum from std::cmp
// Hint: use std::cmp::Ordering;

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
    
    // TODO: Create a match expression to compare the two numbers
    // Use number1.cmp(&number2) to get the comparison result
    // Handle all three Ordering cases:
    // - Ordering::Less (when number1 < number2)
    // - Ordering::Greater (when number1 > number2)  
    // - Ordering::Equal (when number1 == number2)
    // Provide descriptive messages for each case
    
    // Your match expression here:
    
}