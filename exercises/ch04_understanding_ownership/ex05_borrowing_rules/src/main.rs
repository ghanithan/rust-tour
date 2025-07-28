// Advanced borrowing scenarios - analyze and fix!
// Each section demonstrates different aspects of the borrow checker

fn main() {
    println!("=== Nested Scope Analysis ===");
    nested_scope_challenge();
    
    println!("\n=== Conditional Borrowing ===");
    conditional_borrowing_challenge();
    
    println!("\n=== Loop Borrowing ===");
    loop_borrowing_challenge();
    
    println!("\n=== Function Return Borrowing ===");
    function_return_challenge();
    
    println!("\n=== Struct Field Borrowing ===");
    struct_field_challenge();
}

// Challenge 1: Nested scopes and reference validity
fn nested_scope_challenge() {
    let mut data = vec![1, 2, 3, 4, 5];
    let max_val;
    
    println!("Processing data: {:?}", data);
    
    {
        let data_ref = &data;
        max_val = data_ref.iter().max().unwrap();
        // ERROR: max_val borrows from data_ref, which goes out of scope
    }
    
    println!("Maximum value: {}", max_val);  // ERROR: max_val references dropped data
    
    // Try to modify data after analysis
    for item in &mut data {
        *item *= 2;
    }
    
    println!("Final result: {:?}", data);
}

// Challenge 2: Conditional borrowing patterns
fn conditional_borrowing_challenge() {
    let mut user = User::new("alice".to_string(), 1500);
    
    let user_ref = &user;
    println!("User {} has {} points", user_ref.name, user_ref.points);
    
    // ERROR: Can't have both immutable and mutable references
    let user_mut_ref = &mut user;
    
    if user_ref.points > 1000 {  // Using immutable reference
        println!("{} is a premium user", user_ref.name);
        user_mut_ref.deduct_points(50);  // ERROR: Using mutable reference simultaneously
    }
    
    println!("Balance after transaction: {}", user.points);
}

// Challenge 3: Loop borrowing complications
fn loop_borrowing_challenge() {
    let mut strings = vec!["Hello".to_string(), "World".to_string(), "Rust".to_string()];
    let mut count = 0;
    
    for s in &strings {  // Immutable borrow for iteration
        println!("Processing: {}", s);
        count += 1;
        
        // ERROR: Can't modify the vector while iterating
        if s == "World" {
            strings.push("Programming".to_string());
        }
    }
    
    println!("All strings processed: {}", count);
}

// Challenge 4: Returning references from functions
fn function_return_challenge() {
    let text = "The Rust Programming Language".to_string();
    
    // ERROR: This function tries to return a reference to local data
    let longest = find_longest_word(&text);
    
    println!("Found longest word: {}", longest);
    println!("Context still available: {}", text);
}

// This function has a lifetime issue
fn find_longest_word(text: &str) -> &str {
    let words: Vec<&str> = text.split_whitespace().collect();
    let mut longest = "";
    
    for word in words {
        if word.len() > longest.len() {
            longest = word;
        }
    }
    
    longest  // This is actually OK, but students might think it's wrong
}

// Challenge 5: Struct field borrowing
fn struct_field_challenge() {
    let mut system = IoTSystem::new();
    
    // ERROR: Multiple mutable borrows of different fields
    let sensor_ref = &mut system.sensors;
    let actuator_ref = &mut system.actuators;
    
    // Try to use both simultaneously
    if let Some(temp) = sensor_ref.get("temperature") {
        println!("Reading sensor: temperature = {}", temp);
        
        if *temp > 20.0 {
            actuator_ref.insert("fan_speed".to_string(), "75%".to_string());
            println!("Writing to actuator: Setting fan speed to 75%");
        }
    }
    
    // ERROR: Can't borrow as immutable while mutable borrows exist
    println!("System status: {}", system.status());
}

// Supporting types
struct User {
    name: String,
    points: u32,
}

impl User {
    fn new(name: String, points: u32) -> Self {
        User { name, points }
    }
    
    fn deduct_points(&mut self, amount: u32) {
        if self.points >= amount {
            self.points -= amount;
        }
    }
}

use std::collections::HashMap;

struct IoTSystem {
    sensors: HashMap<String, f64>,
    actuators: HashMap<String, String>,
}

impl IoTSystem {
    fn new() -> Self {
        let mut sensors = HashMap::new();
        sensors.insert("temperature".to_string(), 23.5);
        sensors.insert("humidity".to_string(), 65.0);
        
        IoTSystem {
            sensors,
            actuators: HashMap::new(),
        }
    }
    
    fn status(&self) -> &str {
        "Active"
    }
}