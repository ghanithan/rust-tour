use std::process::Command;

#[test]
fn test_program_compiles_and_runs() {
    let output = Command::new("cargo")
        .args(&["run", "--quiet"])
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output()
        .expect("Failed to execute program");

    assert!(
        output.status.success(),
        "Program should compile and run successfully"
    );
}

#[test]
fn test_boolean_operations() {
    let output = Command::new("cargo")
        .args(&["run", "--quiet"])
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output()
        .expect("Failed to execute program");

    let stdout = String::from_utf8_lossy(&output.stdout);
    
    // Test logical operations
    assert!(
        stdout.contains("Both are true: true"),
        "Should demonstrate AND operation with both true values"
    );
    
    assert!(
        stdout.contains("At least one true: true"),
        "Should demonstrate OR operation"
    );
    
    assert!(
        stdout.contains("Not fun: false"),
        "Should demonstrate NOT operation"
    );
}

#[test]
fn test_comparison_operations() {
    let output = Command::new("cargo")
        .args(&["run", "--quiet"])
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output()
        .expect("Failed to execute program");

    let stdout = String::from_utf8_lossy(&output.stdout);
    
    // Test age comparison (25 >= 18 should be true)
    assert!(
        stdout.contains("Can vote: true"),
        "Should correctly compare age for voting eligibility"
    );
    
    // Test temperature range (75 should be comfortable between 65-80)
    assert!(
        stdout.contains("Comfortable temperature: true"),
        "Should correctly check temperature range"
    );
}

#[test]
fn test_character_operations() {
    let output = Command::new("cargo")
        .args(&["run", "--quiet"])
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output()
        .expect("Failed to execute program");

    let stdout = String::from_utf8_lossy(&output.stdout);
    
    // Test character display
    assert!(
        stdout.contains("Letter: A"),
        "Should display character correctly"
    );
    
    assert!(
        stdout.contains("Emoji: 🦀"),
        "Should display Unicode emoji correctly"
    );
    
    // Test character methods
    assert!(
        stdout.contains("'A' is alphabetic: true"),
        "Should correctly identify alphabetic character"
    );
    
    assert!(
        stdout.contains("is numeric: true"),
        "Should correctly identify numeric character"
    );
}

#[test]
fn test_character_transformations() {
    let output = Command::new("cargo")
        .args(&["run", "--quiet"])
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output()
        .expect("Failed to execute program");

    let stdout = String::from_utf8_lossy(&output.stdout);
    
    // Test lowercase conversion
    assert!(
        stdout.contains("'A' in lowercase: a"),
        "Should convert character to lowercase correctly"
    );
}

#[test]
fn test_combined_boolean_character_logic() {
    let output = Command::new("cargo")
        .args(&["run", "--quiet"])
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output()
        .expect("Failed to execute program");

    let stdout = String::from_utf8_lossy(&output.stdout);
    
    // Test alphanumeric check (should be true for digit '7')
    assert!(
        stdout.contains("'7' is alphanumeric: true"),
        "Should correctly identify alphanumeric character"
    );
    
    // Test not-space check
    assert!(
        stdout.contains("is not a space: true"),
        "Should correctly identify non-space character"
    );
}