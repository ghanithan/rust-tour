use std::process::Command;

#[test]
fn test_program_runs_successfully() {
    let output = Command::new("cargo")
        .args(&["run", "--quiet"])
        .current_dir(".")
        .output()
        .expect("Failed to execute cargo run");
    
    assert!(output.status.success(), "Program should compile and run successfully");
}

#[test]
fn test_basic_constructor_output() {
    let output = Command::new("cargo")
        .args(&["run", "--quiet"])
        .current_dir(".")
        .output()
        .expect("Failed to execute cargo run");
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Alice Johnson (25 years old, alice@email.com)"), 
            "Should create Alice with basic constructor");
}

#[test]
fn test_child_constructor_output() {
    let output = Command::new("cargo")
        .args(&["run", "--quiet"])
        .current_dir(".")
        .output()
        .expect("Failed to execute cargo run");
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Bobby Smith (8 years old, no email)"), 
            "Should create Bobby with child constructor (no email)");
}

#[test]
fn test_adult_constructor_output() {
    let output = Command::new("cargo")
        .args(&["run", "--quiet"])
        .current_dir(".")
        .output()
        .expect("Failed to execute cargo run");
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Carol Davis (30 years old, carol@email.com)"), 
            "Should create Carol with adult constructor");
}

#[test]
fn test_validation_constructor_success() {
    let output = Command::new("cargo")
        .args(&["run", "--quiet"])
        .current_dir(".")
        .output()
        .expect("Failed to execute cargo run");
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("David Wilson (45 years old, david@email.com)"), 
            "Should create David with validated constructor when input is valid");
}

#[test]
fn test_validation_constructor_age_error() {
    let output = Command::new("cargo")
        .args(&["run", "--quiet"])
        .current_dir(".")
        .output()
        .expect("Failed to execute cargo run");
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Error creating person: Age must be between 0 and 150"), 
            "Should show age validation error for age > 150");
}

#[test]
fn test_validation_constructor_email_error() {
    let output = Command::new("cargo")
        .args(&["run", "--quiet"])
        .current_dir(".")
        .output()
        .expect("Failed to execute cargo run");
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Error creating person: Email must contain @ symbol"), 
            "Should show email validation error for email without @");
}

#[test]
fn test_from_name_constructor_output() {
    let output = Command::new("cargo")
        .args(&["run", "--quiet"])
        .current_dir(".")
        .output()
        .expect("Failed to execute cargo run");
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Eve Brown (0 years old, no email)"), 
            "Should create Eve with from_name constructor (age 0, no email)");
}

#[test]
fn test_method_calls_on_instances() {
    let output = Command::new("cargo")
        .args(&["run", "--quiet"])
        .current_dir(".")
        .output()
        .expect("Failed to execute cargo run");
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    
    // Test that methods work correctly on created instances
    assert!(stdout.contains("Alice is an adult: true"), 
            "Alice (25) should be classified as adult");
    assert!(stdout.contains("Bobby is a child: true"), 
            "Bobby (8) should be classified as child");
    assert!(stdout.contains("Carol can vote: true"), 
            "Carol (30) should be able to vote");
    assert!(stdout.contains("David can vote: true"), 
            "David (45) should be able to vote");
    assert!(stdout.contains("Eve is a child: true"), 
            "Eve (0) should be classified as child");
}

#[test]
fn test_expected_output_structure() {
    let output = Command::new("cargo")
        .args(&["run", "--quiet"])
        .current_dir(".")
        .output()
        .expect("Failed to execute cargo run");
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    
    // Verify the overall structure matches expected output
    assert!(stdout.contains("Creating people with different constructors..."), 
            "Should display opening message");
    assert!(stdout.contains("Basic constructor:"), 
            "Should show basic constructor section");
    assert!(stdout.contains("Child constructor:"), 
            "Should show child constructor section");
    assert!(stdout.contains("Adult constructor:"), 
            "Should show adult constructor section");
    assert!(stdout.contains("Validation constructor (valid):"), 
            "Should show validation success section");
    assert!(stdout.contains("Validation constructor (invalid age):"), 
            "Should show validation age error section");
    assert!(stdout.contains("Validation constructor (invalid email):"), 
            "Should show validation email error section");
    assert!(stdout.contains("From name constructor:"), 
            "Should show from_name constructor section");
    assert!(stdout.contains("Testing methods on created instances..."), 
            "Should show method testing section");
}

#[test]
fn test_compilation_passes() {
    let output = Command::new("cargo")
        .args(&["check", "--quiet"])
        .current_dir(".")
        .output()
        .expect("Failed to execute cargo check");
    
    assert!(output.status.success(), 
            "Code should compile without errors. Compilation output: {}",
            String::from_utf8_lossy(&output.stderr));
}

#[test]
fn test_unit_tests_pass() {
    let output = Command::new("cargo")
        .args(&["test", "--bin", "ex04_associated_functions", "--quiet"])
        .current_dir(".")
        .output()
        .expect("Failed to execute cargo test");
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    
    assert!(output.status.success(), 
            "All unit tests in main.rs should pass. stdout: {}\nstderr: {}",
            stdout, stderr);
}