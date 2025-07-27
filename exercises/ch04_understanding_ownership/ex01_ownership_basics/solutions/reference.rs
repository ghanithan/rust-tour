// Solution: Understanding and fixing ownership errors

fn main() {
    // Part 1: Basic move semantics
    // Solution: Clone the string so both variables own their own data
    let s1 = String::from("Hello");
    let s2 = s1.clone();  // Create a deep copy
    
    println!("First string: {}", s1);  // s1 is still valid
    println!("Second string: {}", s2);
    
    // Part 2: Function calls and ownership
    // Solution: Either clone before passing or don't use after moving
    let s3 = String::from("World");
    let combined = combine_strings(s2, s3);  // s2 and s3 are moved
    
    println!("Combined: {}", combined);
    // Removed the line trying to use s2 - it was moved!
    
    // Part 3: Copy types behave differently
    let x = 42;
    let y = x;  // x is copied, not moved (i32 implements Copy)
    
    println!("Integer value is still accessible: {}", x);
    
    // Part 4: Taking and returning ownership
    let s4 = String::from("Rust");
    let s5 = takes_and_gives_back(s4);  // s4 is moved to the function
    
    println!("After taking ownership: {}", s5);
    println!("String was moved and is no longer accessible");
    
    // s4 cannot be used here - it was moved
}

fn combine_strings(s1: String, s2: String) -> String {
    format!("{}, {}!", s1, s2)
}

fn takes_and_gives_back(some_string: String) -> String {
    println!("Inside function: {}", some_string);
    some_string  // Ownership is returned to the caller
}