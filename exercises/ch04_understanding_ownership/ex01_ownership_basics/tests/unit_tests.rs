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
fn test_expected_output() {
    let output = Command::new("cargo")
        .args(&["run", "--quiet"])
        .output()
        .expect("Failed to execute program");

    let stdout = String::from_utf8_lossy(&output.stdout);
    
    // Check for key output lines that demonstrate ownership concepts
    assert!(stdout.contains("First string: Hello"), "Should print first string");
    assert!(stdout.contains("Second string:"), "Should print second string");
    assert!(stdout.contains("Combined:") && stdout.contains("Hello"), "Should show combined strings");
    assert!(stdout.contains("Integer value is still accessible: 42"), "Should demonstrate Copy trait");
    assert!(stdout.contains("After taking ownership: Rust"), "Should show ownership transfer");
}

#[test]
fn test_no_ownership_errors_in_output() {
    let output = Command::new("cargo")
        .args(&["run", "--quiet"])
        .output()
        .expect("Failed to execute program");

    let stderr = String::from_utf8_lossy(&output.stderr);
    
    // Make sure there are no compilation errors
    assert!(!stderr.contains("borrow of moved value"), "Should not have 'borrow of moved value' errors");
    assert!(!stderr.contains("value borrowed here after move"), "Should not have move errors");
}

#[test]
fn test_demonstrates_function_ownership() {
    let output = Command::new("cargo")
        .args(&["run", "--quiet"])
        .output()
        .expect("Failed to execute program");

    let stdout = String::from_utf8_lossy(&output.stdout);
    
    // The takes_and_gives_back function should print
    assert!(stdout.contains("Inside function:"), "Should demonstrate function taking ownership");
}