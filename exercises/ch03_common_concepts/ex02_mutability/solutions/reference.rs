// Reference solution for Advanced Mutability Concepts
//
// This solution demonstrates proper use of mutable variables,
// mutable references, and function parameters.

fn main() {
    // Part 1: Mutable variables and references
    println!("=== Part 1: Mutable Variables ===");
    
    // Made variable mutable so we can change it
    let mut temperature = 32;
    println!("Initial temperature: {}°F", temperature);
    
    // This works because temperature is mutable
    temperature = 212;
    println!("Updated temperature: {}°F", temperature);
    
    // Part 2: References and mutability
    println!("\n=== Part 2: References ===");
    
    let mut score = 100;
    
    // Created a mutable reference to score
    let score_ref = &mut score;
    
    // This works because we have a mutable reference
    *score_ref += 50;
    println!("Final score: {}", score);
    
    // Part 3: Function parameters
    println!("\n=== Part 3: Function Parameters ===");
    
    let mut counter = 0;
    increment_counter(&mut counter);  // Pass mutable reference
    println!("Counter after increment: {}", counter);
}

// Fixed function to accept a mutable reference and modify the value
fn increment_counter(counter: &mut i32) {
    *counter += 1;
}