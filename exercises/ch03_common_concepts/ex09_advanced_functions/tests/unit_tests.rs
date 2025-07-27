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
fn test_demonstrates_expressions_vs_statements() {
    let output = Command::new("cargo")
        .args(&["run", "--quiet"])
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output()
        .expect("Failed to execute program");

    let stdout = String::from_utf8_lossy(&output.stdout);
    
    assert!(
        stdout.contains("Statement-based addition: 8"),
        "Should demonstrate statement-based function"
    );
    
    assert!(
        stdout.contains("Expression-based addition: 8"),
        "Should demonstrate expression-based function"
    );
}

#[test]
fn test_early_returns_classification() {
    let output = Command::new("cargo")
        .args(&["run", "--quiet"])
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output()
        .expect("Failed to execute program");

    let stdout = String::from_utf8_lossy(&output.stdout);
    
    assert!(
        stdout.contains("Number -5: negative"),
        "Should classify negative numbers correctly"
    );
    
    assert!(
        stdout.contains("Number 0: zero"),
        "Should classify zero correctly"
    );
    
    assert!(
        stdout.contains("Number 10: small positive"),
        "Should classify small positive numbers correctly"
    );
    
    assert!(
        stdout.contains("Number 20: large positive"),
        "Should classify large positive numbers correctly"
    );
}

#[test]
fn test_option_returns() {
    let output = Command::new("cargo")
        .args(&["run", "--quiet"])
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output()
        .expect("Failed to execute program");

    let stdout = String::from_utf8_lossy(&output.stdout);
    
    assert!(
        stdout.contains("√10 = 3.16") || stdout.contains("√10 = 3.17"),
        "Should calculate square root for valid numbers"
    );
    
    assert!(
        stdout.contains("150 is too large"),
        "Should return None for numbers > 100"
    );
}

#[test]
fn test_mutable_references() {
    let output = Command::new("cargo")
        .args(&["run", "--quiet"])
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output()
        .expect("Failed to execute program");

    let stdout = String::from_utf8_lossy(&output.stdout);
    
    assert!(
        stdout.contains("Initial counter: 0"),
        "Should show initial counter value"
    );
    
    assert!(
        stdout.contains("After increment: 1"),
        "Should increment counter correctly"
    );
    
    assert!(
        stdout.contains("After doubling: 2"),
        "Should double counter correctly"
    );
}

#[test]
fn test_recursive_functions() {
    let output = Command::new("cargo")
        .args(&["run", "--quiet"])
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output()
        .expect("Failed to execute program");

    let stdout = String::from_utf8_lossy(&output.stdout);
    
    assert!(
        stdout.contains("3^4 = 81"),
        "Should calculate power correctly using recursion"
    );
    
    assert!(
        stdout.contains("5! = 120"),
        "Should calculate factorial correctly using recursion"
    );
}

#[test]
fn test_function_composition() {
    let output = Command::new("cargo")
        .args(&["run", "--quiet"])
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output()
        .expect("Failed to execute program");

    let stdout = String::from_utf8_lossy(&output.stdout);
    
    // Input 5: double = 10, add 10 = 20, square = 400
    assert!(
        stdout.contains("Square: 400"),
        "Should demonstrate function composition: 5 → 10 → 20 → 400"
    );
    
    assert!(
        stdout.contains("Composed result: 400"),
        "Should show same result with composed function calls"
    );
}