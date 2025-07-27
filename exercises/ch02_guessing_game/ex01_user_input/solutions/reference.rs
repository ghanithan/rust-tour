use std::io;

fn main() {
    println!("Please enter your name:");

    // Create a mutable String to store the user's input
    let mut name = String::new();

    // Read a line from standard input into our string variable
    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read line");

    // Remove any trailing whitespace (like newline characters)
    let name = name.trim();

    // Print a personalized greeting using the user's name
    println!("Hello, {}! Welcome to Rust programming.", name);
}