// Reference solution for Variables and Mutability
//
// This solution demonstrates proper use of mutability, shadowing, and constants
// in Rust while fixing all the compilation errors from the original exercise.

// Constants belong at module level and must have proper naming and type annotations
const SECONDS_IN_MINUTE: u32 = 60;

fn main() {
    // Section 1: Basic immutability - FIXED by adding mut
    println!("=== Section 1: Basic Variables ===");
    let mut x = 5;  // Added mut to allow mutation
    println!("The value of x is: {}", x);
    
    x = 6;  // This now works because x is mutable
    println!("The value of x is now: {}", x);
    
    // Section 2: Shadowing vs Mutability - this was already correct
    println!("\n=== Section 2: Shadowing ===");
    let y = 5;
    let y = y + 1;  // Shadowing: creates new variable
    let y = y * 2;  // Shadowing again
    println!("The value of y is: {}", y);
    
    // FIXED: Use shadowing to change type from &str to usize
    let spaces = "   ";
    let spaces = spaces.len();  // Shadow with new type
    println!("Number of spaces: {}", spaces);
    
    // Section 3: Constants - FIXED by using proper const syntax
    println!("\n=== Section 3: Constants ===");
    const MAX_POINTS: u32 = 100_000;  // Proper constant declaration
    println!("Maximum points: {}", MAX_POINTS);
    
    // Section 4: Multiple mutations - FIXED by making variable mutable
    println!("\n=== Section 4: Multiple Mutations ===");
    let mut counter = 0;  // Added mut to allow mutations
    println!("Counter starts at: {}", counter);
    
    counter = counter + 1;  // Now works because counter is mutable
    println!("Counter is now: {}", counter);
    
    counter = counter + 1;  // Multiple mutations are allowed
    println!("Counter is now: {}", counter);
    
    // Section 5: Shadowing with different types - FIXED syntax
    println!("\n=== Section 5: Type Changes with Shadowing ===");
    let value = "42";
    println!("Value as string: {}", value);
    
    // Shadow to convert type, then make the new variable mutable
    let mut value = value.parse::<i32>().expect("Not a number!");
    println!("Value as number: {}", value);
    
    // This works because we made the new shadowed variable mutable
    value = value * 2;
    println!("Value doubled: {}", value);
    
    // Demonstrate the module-level constant
    println!("\nThere are {} seconds in a minute", SECONDS_IN_MINUTE);
}