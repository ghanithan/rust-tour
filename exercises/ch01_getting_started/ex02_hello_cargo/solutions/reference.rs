// Reference solution for Hello, Cargo!
//
// This solution demonstrates proper use of Cargo dependencies
// and shows how to structure a basic Rust project.

use colored::*;

fn main() {
    // Colorful greeting
    println!("{}", "Hello, Cargo!".green().bold());
    
    // Informational message about the project
    println!("{}", "This is a Rust project managed by Cargo!".blue());
    
    // Demonstrate what Cargo does
    println!("{}", "Cargo handles:".yellow().bold());
    println!("  {} {}", "•".red(), "Building your code".white());
    println!("  {} {}", "•".red(), "Managing dependencies".white());
    println!("  {} {}", "•".red(), "Running tests".white());
    println!("  {} {}", "•".red(), "Publishing packages".white());
    
    // Final message
    println!();
    println!("{}", "Welcome to the Rust ecosystem! 🦀".magenta().bold());
    println!("{}", "Happy coding with Cargo!".cyan());
}