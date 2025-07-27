// Reference solution for Loops

fn main() {
    println!("=== Loop Constructs ===");
    
    // Part 1: Basic loop with break
    println!("\n--- Infinite Loop with Break ---");
    let mut counter = 0;
    
    loop {
        println!("Counter: {}", counter);
        counter += 1;
        
        if counter >= 3 {
            break;
        }
    }
    
    // Part 2: Loop with return value
    println!("\n--- Loop with Return Value ---");
    let mut x = 0;
    
    let result = loop {
        x += 1;
        if x == 5 {
            break x * 2;
        }
    };
    
    println!("Loop result: {}", result);
    
    // Part 3: While loop
    println!("\n--- While Loop ---");
    let mut number = 10;
    
    while number > 0 {
        println!("Countdown: {}", number);
        number -= 1;
    }
    println!("Liftoff!");
    
    // Part 4: For loop with range
    println!("\n--- For Loop with Range ---");
    
    for i in 1..=5 {
        println!("Number: {}", i);
    }
    
    // Part 5: For loop with array
    println!("\n--- For Loop with Array ---");
    let fruits = ["apple", "banana", "cherry"];
    
    for fruit in fruits.iter() {
        println!("Fruit: {}", fruit);
    }
    
    // Part 6: Nested loops
    println!("\n--- Nested Loops ---");
    
    for i in 1..=3 {
        for j in 1..=3 {
            print!("{} ", i * j);
        }
        println!();
    }
    
    // Part 7: Loop control with continue
    println!("\n--- Loop with Continue ---");
    
    for num in 1..=10 {
        if num % 2 == 0 {
            continue;
        }
        println!("Odd number: {}", num);
    }
    
    // Part 8: While let pattern
    println!("\n--- While Let Pattern ---");
    let mut stack = vec![1, 2, 3, 4, 5];
    
    while let Some(value) = stack.pop() {
        println!("Popped: {}", value);
    }
}