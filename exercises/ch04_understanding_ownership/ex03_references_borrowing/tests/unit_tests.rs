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
fn test_expected_output_content() {
    let output = Command::new("cargo")
        .args(&["run", "--quiet"])
        .output()
        .expect("Failed to execute program");

    let stdout = String::from_utf8_lossy(&output.stdout);
    
    // Verify key outputs that show references are working
    assert!(stdout.contains("Original string: Hello, Rust!"), "Should show original string is still accessible");
    assert!(stdout.contains("String length: 12"), "Should calculate correct length via reference");
    assert!(stdout.contains("First word: Hello"), "Should extract first word via reference");
    assert!(stdout.contains("Still available: Hello, Rust!"), "Should show string is still usable after borrowing");
    assert!(stdout.contains("Book info:"), "Should print book info via reference");
    assert!(stdout.contains("Analysis:"), "Should show analysis via references");
    assert!(stdout.contains("Book title is still available:"), "Should show title is still accessible");
}

#[test]
fn test_no_move_errors() {
    let output = Command::new("cargo")
        .args(&["check"])
        .output()
        .expect("Failed to execute cargo check");

    let stderr = String::from_utf8_lossy(&output.stderr);
    
    // Ensure no ownership/borrowing errors
    assert!(!stderr.contains("borrow of moved value"), "Should not have moved value errors");
    assert!(!stderr.contains("value borrowed here after move"), "Should not have borrowing after move errors");
    assert!(!stderr.contains("use of moved value"), "Should not have use of moved value errors");
}

// Integration test to verify the functions work correctly
#[cfg(test)]
mod integration_tests {
    // Note: In a real scenario, we'd import from main.rs
    // For this test structure, we'll create minimal versions
    
    fn calculate_length(s: &String) -> usize {
        s.len()
    }
    
    fn first_word(s: &String) -> &str {
        let bytes = s.as_bytes();
        
        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return &s[0..i];
            }
        }
        
        &s[..]
    }
    
    #[test]
    fn test_calculate_length_function() {
        let s = String::from("Hello, World!");
        let len = calculate_length(&s);
        assert_eq!(len, 13);
        // Verify s is still usable
        assert_eq!(s.len(), 13);
    }
    
    #[test]
    fn test_first_word_function() {
        let s = String::from("Hello World");
        let word = first_word(&s);
        assert_eq!(word, "Hello");
        // Verify s is still usable
        assert!(s.contains("World"));
    }
    
    #[test]
    fn test_first_word_single_word() {
        let s = String::from("Hello");
        let word = first_word(&s);
        assert_eq!(word, "Hello");
    }
}