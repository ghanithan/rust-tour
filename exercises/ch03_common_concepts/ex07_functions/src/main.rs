// Exercise: Basic Functions
//
// Functions are fundamental building blocks in Rust.
// Complete the function definitions below.

fn main() {
    println!("=== Basic Functions ===");
    
    // Call the functions you'll implement
    greet();
    say_hello_to("Alice");
    
    let sum = add_numbers(5, 3);
    println!("Sum: {}", sum);
    
    let product = multiply(4, 7);
    println!("Product: {}", product);
    
    let is_even = check_even(10);
    println!("Is 10 even? {}", is_even);
}

// TODO: Implement a function that prints "Hello, world!"
fn greet() {
    // Your implementation here
}

// TODO: Implement a function that takes a name and prints "Hello, [name]!"
fn say_hello_to(name: &str) {
    // Your implementation here
}

// TODO: Implement a function that takes two i32 parameters and returns their sum
fn add_numbers(a: i32, b: i32) -> i32 {
    // Your implementation here
}

// TODO: Implement a function that multiplies two numbers and returns the result
fn multiply(x: i32, y: i32) -> i32 {
    // Your implementation here
}

// TODO: Implement a function that checks if a number is even (returns bool)
fn check_even(number: i32) -> bool {
    // Your implementation here - use the modulo operator %
}