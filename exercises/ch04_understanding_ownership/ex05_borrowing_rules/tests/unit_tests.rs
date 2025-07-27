use std::process::Command;

#[test]
fn test_program_compiles_and_runs() {
    let output = Command::new("cargo")
        .args(&["run", "--quiet"])
        .output()
        .expect("Failed to execute program");

    assert!(output.status.success(), "Program should compile and run successfully");
}

#[test]
fn test_expected_output_structure() {
    let output = Command::new("cargo")
        .args(&["run", "--quiet"])
        .output()
        .expect("Failed to execute program");

    let stdout = String::from_utf8_lossy(&output.stdout);
    
    // Check for all major sections
    assert!(stdout.contains("=== Nested Scope Analysis ==="), "Should have nested scope section");
    assert!(stdout.contains("=== Conditional Borrowing ==="), "Should have conditional borrowing section");
    assert!(stdout.contains("=== Loop Borrowing ==="), "Should have loop borrowing section");
    assert!(stdout.contains("=== Function Return Borrowing ==="), "Should have function return section");
    assert!(stdout.contains("=== Struct Field Borrowing ==="), "Should have struct field section");
}

#[test]
fn test_nested_scope_output() {
    let output = Command::new("cargo")
        .args(&["run", "--quiet"])
        .output()
        .expect("Failed to execute program");

    let stdout = String::from_utf8_lossy(&output.stdout);
    
    assert!(stdout.contains("Processing data: [1, 2, 3, 4, 5]"), "Should show initial data");
    assert!(stdout.contains("Maximum value: 5"), "Should find maximum value");
    assert!(stdout.contains("Final result: [2, 4, 6, 8, 10]"), "Should show doubled values");
}

#[test]
fn test_conditional_borrowing_output() {
    let output = Command::new("cargo")
        .args(&["run", "--quiet"])
        .output()
        .expect("Failed to execute program");

    let stdout = String::from_utf8_lossy(&output.stdout);
    
    assert!(stdout.contains("User alice has 1500 points"), "Should show user info");
    assert!(stdout.contains("premium user"), "Should identify premium user");
    assert!(stdout.contains("Balance after transaction: 1450"), "Should show updated balance");
}

#[test]
fn test_loop_borrowing_output() {
    let output = Command::new("cargo")
        .args(&["run", "--quiet"])
        .output()
        .expect("Failed to execute program");

    let stdout = String::from_utf8_lossy(&output.stdout);
    
    assert!(stdout.contains("Processing: Hello"), "Should process Hello");
    assert!(stdout.contains("Processing: World"), "Should process World");
    assert!(stdout.contains("Processing: Rust"), "Should process Rust");
    assert!(stdout.contains("All strings processed:"), "Should show count");
}

#[test]
fn test_function_return_output() {
    let output = Command::new("cargo")
        .args(&["run", "--quiet"])
        .output()
        .expect("Failed to execute program");

    let stdout = String::from_utf8_lossy(&output.stdout);
    
    assert!(stdout.contains("Found longest word: Programming"), "Should find longest word");
    assert!(stdout.contains("Context still available: The Rust Programming Language"), "Should show text is still available");
}

#[test]
fn test_struct_field_output() {
    let output = Command::new("cargo")
        .args(&["run", "--quiet"])
        .output()
        .expect("Failed to execute program");

    let stdout = String::from_utf8_lossy(&output.stdout);
    
    assert!(stdout.contains("Reading sensor: temperature = 23.5"), "Should read temperature");
    assert!(stdout.contains("Writing to actuator: Setting fan speed to 75%"), "Should set fan speed");
    assert!(stdout.contains("System status: Active"), "Should show system status");
}

#[test]
fn test_no_borrowing_errors() {
    let output = Command::new("cargo")
        .args(&["check"])
        .output()
        .expect("Failed to execute cargo check");

    let stderr = String::from_utf8_lossy(&output.stderr);
    
    // Ensure no borrowing errors
    assert!(!stderr.contains("cannot borrow"), "Should not have borrowing errors");
    assert!(!stderr.contains("borrow of moved value"), "Should not have moved value errors");
    assert!(!stderr.contains("does not live long enough"), "Should not have lifetime errors");
    assert!(!stderr.contains("borrowed value does not live long enough"), "Should not have borrowed value lifetime errors");
}

// Test individual components in isolation
#[cfg(test)]
mod component_tests {
    use std::collections::HashMap;

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

    #[test]
    fn test_user_operations() {
        let mut user = User::new("test".to_string(), 100);
        assert_eq!(user.points, 100);
        
        user.deduct_points(30);
        assert_eq!(user.points, 70);
    }

    #[test]
    fn test_iot_system() {
        let system = IoTSystem::new();
        assert_eq!(system.status(), "Active");
        assert_eq!(system.sensors.get("temperature"), Some(&23.5));
    }

    #[test]
    fn test_find_longest_word() {
        fn find_longest_word(text: &str) -> &str {
            let words: Vec<&str> = text.split_whitespace().collect();
            let mut longest = "";
            
            for word in words {
                if word.len() > longest.len() {
                    longest = word;
                }
            }
            
            longest
        }

        let text = "The Rust Programming Language";
        let longest = find_longest_word(text);
        assert_eq!(longest, "Programming");
    }
}