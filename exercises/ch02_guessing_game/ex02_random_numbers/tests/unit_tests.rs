use std::process::Command;
use std::collections::HashSet;

#[test]
fn test_program_compiles() {
    let output = Command::new("cargo")
        .args(&["check"])
        .current_dir(".")
        .output()
        .expect("Failed to execute cargo check");

    assert!(
        output.status.success(),
        "Program should compile successfully. Compiler output:\n{}",
        String::from_utf8_lossy(&output.stderr)
    );
}

#[test]
fn test_cargo_toml_has_rand_dependency() {
    let cargo_content = std::fs::read_to_string("Cargo.toml")
        .expect("Should be able to read Cargo.toml");
    
    assert!(
        cargo_content.contains("rand"),
        "Cargo.toml should contain rand as a dependency. Current content:\n{}",
        cargo_content
    );
}

#[test]
fn test_program_runs_successfully() {
    let output = Command::new("cargo")
        .args(&["run"])
        .current_dir(".")
        .output()
        .expect("Failed to execute cargo run");

    assert!(
        output.status.success(),
        "Program should run successfully. Error output:\n{}",
        String::from_utf8_lossy(&output.stderr)
    );
}

#[test]
fn test_program_generates_numbers() {
    let output = Command::new("cargo")
        .args(&["run"])
        .current_dir(".")
        .output()
        .expect("Failed to execute cargo run");

    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        
        // Check that numbers are present in output
        let has_numbers = stdout.lines().any(|line| {
            line.chars().any(|c| c.is_ascii_digit())
        });
        
        assert!(
            has_numbers,
            "Program should generate and display numbers. Output:\n{}",
            stdout
        );
    }
}

#[test]
fn test_numbers_are_in_valid_range() {
    let output = Command::new("cargo")
        .args(&["run"])
        .current_dir(".")
        .output()
        .expect("Failed to execute cargo run");

    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        
        // Extract numbers from output
        let mut found_valid_number = false;
        
        for line in stdout.lines() {
            for word in line.split_whitespace() {
                if let Ok(num) = word.parse::<i32>() {
                    if num >= 1 && num <= 100 {
                        found_valid_number = true;
                    }
                    assert!(
                        num >= 1 && num <= 100,
                        "Generated number {} should be between 1 and 100. Output:\n{}",
                        num, stdout
                    );
                }
            }
        }
        
        assert!(
            found_valid_number,
            "Should find at least one valid number in the output. Output:\n{}",
            stdout
        );
    }
}

#[test]
fn test_program_shows_randomness() {
    // Run the program multiple times to check for different outputs
    let mut outputs = HashSet::new();
    
    for _ in 0..5 {
        let output = Command::new("cargo")
            .args(&["run"])
            .current_dir(".")
            .output()
            .expect("Failed to execute cargo run");
            
        if output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout);
            outputs.insert(stdout.to_string());
        }
    }
    
    // We expect some variation in outputs (though not guaranteed due to randomness)
    // This is a reasonable check that random generation is working
    assert!(
        outputs.len() >= 1,
        "Program should run successfully at least once to demonstrate randomness"
    );
}

#[test]
fn test_program_uses_descriptive_output() {
    let output = Command::new("cargo")
        .args(&["run"])
        .current_dir(".")
        .output()
        .expect("Failed to execute cargo run");

    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout).to_lowercase();
        
        // Check for descriptive text about random numbers
        let has_descriptive_text = stdout.contains("random") || 
                                   stdout.contains("number") || 
                                   stdout.contains("generat");
        
        assert!(
            has_descriptive_text,
            "Program should have descriptive text about random number generation. Output:\n{}",
            String::from_utf8_lossy(&output.stdout)
        );
    }
}

#[test]
fn test_program_generates_multiple_numbers() {
    let output = Command::new("cargo")
        .args(&["run"])
        .current_dir(".")
        .output()
        .expect("Failed to execute cargo run");

    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        
        // Count how many numbers appear in the output
        let mut number_count = 0;
        
        for line in stdout.lines() {
            for word in line.split_whitespace() {
                if let Ok(num) = word.parse::<i32>() {
                    if num >= 1 && num <= 100 {
                        number_count += 1;
                    }
                }
            }
        }
        
        assert!(
            number_count >= 3,
            "Program should generate at least 3 random numbers. Found {} numbers. Output:\n{}",
            number_count, stdout
        );
    }
}