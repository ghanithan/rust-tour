//! This program demonstrates Rust edition differences and features.
//! 
//! It shows how different editions enable different language features
//! while maintaining backward compatibility.

use std::collections::HashMap;

fn main() {
    println!("=== Rust Edition Demonstration ===");
    
    // Demonstrate edition 2021 features
    demonstrate_array_iteration();
    
    // Show backward compatibility 
    demonstrate_backward_compatibility();
    
    // Explain edition benefits
    explain_edition_system();
    
    println!("\n=== Edition Demo Complete ===");
}

/// Demonstrates Rust 2021 array iteration improvements
fn demonstrate_array_iteration() {
    println!("\n--- Array Iteration (2021 Edition Feature) ---");
    
    let numbers = [1, 2, 3, 4, 5];
    
    // 2021 edition allows direct iteration over arrays
    // In earlier editions, you'd need numbers.iter()
    print!("Direct iteration: ");
    for num in numbers {  // This works directly in 2021 edition
        print!("{} ", num);
    }
    println!();
    
    // Show that older patterns still work (backward compatibility)
    print!("Iterator method: ");
    for num in numbers.iter() {  // This still works in 2021
        print!("{} ", num);
    }
    println!();
}

/// Shows that older edition patterns still work in newer editions
fn demonstrate_backward_compatibility() {
    println!("\n--- Backward Compatibility ---");
    
    // These patterns work in all editions
    let data = vec![10, 20, 30];
    
    // Traditional iterator approach (works in all editions)
    let doubled: Vec<i32> = data.iter().map(|x| x * 2).collect();
    println!("Doubled values: {:?}", doubled);
    
    // Show module system improvements from 2018
    // In 2015: extern crate std; was often needed
    // In 2018+: automatically available
    let mut map = HashMap::new();
    map.insert("edition", "2021");
    println!("Using HashMap: {:?}", map);
}

/// Explains the edition system and its benefits
fn explain_edition_system() {
    println!("\n--- Edition System Explanation ---");
    
    // Explain the three editions
    println!("Rust 2015: Original edition, more verbose syntax");
    println!("Rust 2018: Module system improvements, better ergonomics");
    println!("Rust 2021: Closure captures, array iteration, new keywords");
    
    // Show practical benefits
    println!("\nEdition Benefits:");
    println!("- Backward compatibility: old code still compiles");
    println!("- Opt-in migration: choose when to upgrade");
    println!("- Interoperability: mix editions in dependencies");
    println!("- Stability: no forced breaking changes");
    
    // Demonstrate closure improvements (2021 edition)
    let name = String::from("Rust");
    let closure = || {
        // In 2021 edition, closures capture more precisely
        println!("Hello from {}!", name);
    };
    closure();
}