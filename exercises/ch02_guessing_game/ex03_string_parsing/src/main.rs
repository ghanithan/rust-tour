use std::io;

fn main() {
    println!("Enter a number:");
    
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    
    // BUG 1: This line has several issues:
    // - Missing type annotation for parsing
    // - Not trimming whitespace before parsing
    // - Need to convert string to number
    let number = input;
    
    println!("You entered: {}", number);
    
    // BUG 2: This will do string concatenation instead of numeric addition
    // because 'number' is still a string, not a number
    let doubled = number + number;
    println!("Double the number: {}", doubled);
    
    // BUG 3: This comparison won't work correctly with strings
    // and the comparison logic is wrong anyway
    let is_big = number > "30";
    println!("Is it greater than 30? {}", is_big);
    
    // TODO: Fix all the bugs above to make this program work correctly
    // The program should:
    // 1. Read a number from the user
    // 2. Parse it from string to integer 
    // 3. Perform numeric operations correctly
    // 4. Make proper numeric comparisons
}