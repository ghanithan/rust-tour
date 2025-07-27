// Solution: Advanced borrowing rules fixed

use std::collections::HashMap;

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

// Challenge 1: Fixed by copying the value instead of keeping reference
fn nested_scope_challenge() {
    let mut data = vec![1, 2, 3, 4, 5];
    let max_val;
    
    println!("Processing data: {:?}", data);
    
    {
        let data_ref = &data;
        max_val = *data_ref.iter().max().unwrap();  // Copy the value, not the reference
    }
    
    println!("Maximum value: {}", max_val);
    
    // Now we can modify data since max_val is a copy
    for item in &mut data {
        *item *= 2;
    }
    
    println!("Final result: {:?}", data);
}

// Challenge 2: Fixed by sequential borrowing
fn conditional_borrowing_challenge() {
    let mut user = User::new("alice".to_string(), 1500);
    
    // Extract information we need while borrowing immutably
    let (name, points, is_premium) = {
        let user_ref = &user;
        println!("User {} has {} points", user_ref.name, user_ref.points);
        (user_ref.name.clone(), user_ref.points, user_ref.points > 1000)
    };  // Immutable borrow ends here
    
    // Now we can use mutable reference
    if is_premium {
        println!("{} is a premium user", name);
        user.deduct_points(50);
    }
    
    println!("Balance after transaction: {}", user.points);
}

// Challenge 3: Fixed by deferring modifications
fn loop_borrowing_challenge() {
    let mut strings = vec!["Hello".to_string(), "World".to_string(), "Rust".to_string()];
    let mut count = 0;
    let mut should_add_programming = false;
    
    // First pass: read only
    for s in &strings {
        println!("Processing: {}", s);
        count += 1;
        
        if s == "World" {
            should_add_programming = true;  // Remember what to do later
        }
    }  // Immutable borrow ends here
    
    // Second pass: modify if needed
    if should_add_programming {
        strings.push("Programming".to_string());
    }
    
    println!("All strings processed: {}", count);
}

// Challenge 4: This was actually correct! No changes needed
fn function_return_challenge() {
    let text = "The Rust Programming Language".to_string();
    
    let longest = find_longest_word(&text);
    
    println!("Found longest word: {}", longest);
    println!("Context still available: {}", text);
}

// This function is correct - it returns a slice of the input string
fn find_longest_word(text: &str) -> &str {
    let words: Vec<&str> = text.split_whitespace().collect();
    let mut longest = "";
    
    for word in words {
        if word.len() > longest.len() {
            longest = word;  // word is a slice of the input text
        }
    }
    
    longest  // This reference is valid because it points to input text
}

// Challenge 5: Fixed by separating field access
fn struct_field_challenge() {
    let mut system = IoTSystem::new();
    
    // First, read the temperature value
    let temp = system.sensors.get("temperature").copied();
    
    // Then, conditionally modify actuators
    if let Some(temp_val) = temp {
        println!("Reading sensor: temperature = {}", temp_val);
        
        if temp_val > 20.0 {
            system.actuators.insert("fan_speed".to_string(), "75%".to_string());
            println!("Writing to actuator: Setting fan speed to 75%");
        }
    }
    
    // Finally, read system status
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