// Reference solution for Variables and Mutability Exercise
//
// This solution demonstrates the three key concepts:
// 1. Variable shadowing
// 2. Type conversion through shadowing
// 3. Mutable variables

fn main() {
    // Part 1: Variable Shadowing
    // This is allowed - we're creating a new variable with the same name
    let x = 5;
    println!("The value of x is: {}", x);
    
    // Shadow the variable x by binding it to x + 1
    let x = x + 1;
    println!("The value of x is: {}", x);
    
    // Part 2: Type Conversion through Shadowing
    // Shadowing allows us to change the type of a variable
    let spaces = "   ";
    println!("spaces contains: '{}'", spaces);
    
    // Shadow 'spaces' with its length (use .len() method)
    let spaces = spaces.len();
    println!("Number of spaces: {}", spaces);
    
    // Part 3: Mutable Variables
    // Sometimes we need to modify a variable's value
    // Make this variable mutable so we can increment it
    let mut count = 0;
    println!("Initial count: {}", count);
    
    // This line will cause a compilation error if count is not mutable
    count += 1;
    println!("Updated count: {}", count);
}