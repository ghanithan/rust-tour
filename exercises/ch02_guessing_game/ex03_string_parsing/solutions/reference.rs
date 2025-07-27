use std::io;

fn main() {
    println!("Enter a number:");
    
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    
    // Fix: Add type annotation, trim whitespace, and parse to number
    let number: u32 = input.trim().parse().expect("Please enter a valid number");
    
    println!("You entered: {}", number);
    
    // Fix: Now we can do numeric multiplication instead of string concatenation
    let doubled = number * 2;
    println!("Double the number: {}", doubled);
    
    // Fix: Numeric comparison works correctly
    let is_big = number > 30;
    println!("Is it greater than 30? {}", is_big);
}