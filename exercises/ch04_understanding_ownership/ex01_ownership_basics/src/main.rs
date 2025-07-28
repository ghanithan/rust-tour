// Fix the ownership errors in this program
// Remember: String is not Copy, but integers are!

fn main() {
    // Part 1: Basic move semantics
    let s1 = String::from("Hello");
    let s2 = s1;  // s1 is moved to s2
    
    println!("First string: {}", s1);  // ERROR: s1 was moved
    println!("Second string: {}", s2);
    
    // Part 2: Function calls and ownership
    let s3 = String::from("World");
    let combined = combine_strings(s2, s3);
    
    println!("Combined: {}", combined);
    println!("Can I still use s2? {}", s2);  // ERROR: s2 was moved
    
    // Part 3: Copy types behave differently
    let x = 42;
    let y = x;  // x is copied, not moved
    
    println!("Integer value is still accessible: {}", x);
    
    // Part 4: Taking and returning ownership
    let s4 = String::from("Rust");
    let s5 = takes_and_gives_back(s4);
    
    println!("After taking ownership: {}", s5);
    println!("String was moved and is no longer accessible");
    
    // Try to use s4 here - what happens?
    // println!("Original string: {}", s4);  // ERROR: s4 was moved
}

fn combine_strings(s1: String, s2: String) -> String {
    format!("{}, {}!", s1, s2)
}

fn takes_and_gives_back(some_string: String) -> String {
    println!("Inside function: {}", some_string);
    some_string  // Ownership is returned
}