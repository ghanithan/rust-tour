// Exercise: Loops
//
// Master Rust's loop constructs: loop, while, and for.
// Complete the missing loop implementations.

fn main() {
    println!("=== Loop Constructs ===");
    
    // Part 1: Basic loop with break
    println!("\n--- Infinite Loop with Break ---");
    let mut counter = 0;
    
    loop {
        println!("Counter: {}", counter);
        counter += 1;
        
        // TODO: Break when counter reaches 3
        if /* condition */ {
            break;
        }
    }
    
    // Part 2: Loop with return value
    println!("\n--- Loop with Return Value ---");
    let mut x = 0;
    
    // TODO: Complete this loop that returns a value
    let result = loop {
        x += 1;
        if x == 5 {
            /* return x * 2 from the loop */
        }
    };
    
    println!("Loop result: {}", result);
    
    // Part 3: While loop
    println!("\n--- While Loop ---");
    let mut number = 10;
    
    // TODO: Complete while loop that counts down from 10 to 1
    while /* condition */ {
        println!("Countdown: {}", number);
        /* decrement number */
    }
    println!("Liftoff!");
    
    // Part 4: For loop with range
    println!("\n--- For Loop with Range ---");
    
    // TODO: Use for loop to print numbers 1 through 5
    for /* range */ {
        println!("Number: {}", i);
    }
    
    // Part 5: For loop with array
    println!("\n--- For Loop with Array ---");
    let fruits = ["apple", "banana", "cherry"];
    
    // TODO: Use for loop to iterate over the array
    for /* array iteration */ {
        println!("Fruit: {}", fruit);
    }
    
    // Part 6: Nested loops with labels
    println!("\n--- Nested Loops ---");
    
    // TODO: Complete nested loops to create a multiplication table
    for i in 1..=3 {
        for j in 1..=3 {
            print!("{} ", /* calculate i * j */);
        }
        println!(); // New line after each row
    }
    
    // Part 7: Loop control with continue
    println!("\n--- Loop with Continue ---");
    
    // TODO: Use for loop with continue to skip even numbers
    for num in 1..=10 {
        // Skip even numbers
        if /* check if even */ {
            continue;
        }
        println!("Odd number: {}", num);
    }
    
    // Part 8: While let pattern
    println!("\n--- While Let Pattern ---");
    let mut stack = vec![1, 2, 3, 4, 5];
    
    // TODO: Use while let to pop elements until stack is empty
    while let Some(value) = /* pop from stack */ {
        println!("Popped: {}", value);
    }
}