// Exercise: Advanced Mutability Concepts
//
// This exercise explores deeper mutability concepts in Rust:
// 1. Mutable references
// 2. Mutable vs immutable borrowing
// 3. Interior mutability patterns
//
// Fix the compilation errors by applying proper mutability rules.

fn main() {
    // Part 1: Mutable variables and references
    println!("=== Part 1: Mutable Variables ===");
    
    // TODO: Make this variable mutable so we can change it
    let temperature = 32;
    println!("Initial temperature: {}°F", temperature);
    
    // This should work after making temperature mutable
    temperature = 212;
    println!("Updated temperature: {}°F", temperature);
    
    // Part 2: References and mutability
    println!("\n=== Part 2: References ===");
    
    let mut score = 100;
    
    // TODO: Create a mutable reference to score
    let score_ref = &score;
    
    // This should work after creating a mutable reference
    *score_ref += 50;
    println!("Final score: {}", score);
    
    // Part 3: Function parameters
    println!("\n=== Part 3: Function Parameters ===");
    
    let mut counter = 0;
    increment_counter(counter);
    println!("Counter after increment: {}", counter);
}

// TODO: Fix this function to actually modify the counter
fn increment_counter(counter: i32) {
    counter += 1;
}