use std::process::Command;

#[test]
fn test_program_compiles_and_runs() {
    let output = Command::new("cargo")
        .args(&["run", "--quiet"])
        .current_dir(".")
        .output()
        .expect("Failed to execute cargo run");

    assert!(
        output.status.success(),
        "Program should compile and run successfully. Stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
}

#[test] 
fn test_program_output() {
    let output = Command::new("cargo")
        .args(&["run", "--quiet"])
        .current_dir(".")
        .output()
        .expect("Failed to execute cargo run");

    let stdout = String::from_utf8_lossy(&output.stdout);
    
    // Check for key output elements
    assert!(stdout.contains("Creating users..."), "Should print 'Creating users...'");
    assert!(stdout.contains("alice@example.com"), "Should display Alice's email");
    assert!(stdout.contains("Alice Smith"), "Should display Alice's username");
    assert!(stdout.contains("bob@example.com"), "Should display Bob's email");
    assert!(stdout.contains("Bob Jones"), "Should display Bob's username");
    assert!(stdout.contains("After transferring alice..."), "Should print transfer message");
    assert!(stdout.contains("Alice was moved"), "Should print ownership message");
}

#[test]
fn test_struct_definition_exists() {
    let output = Command::new("cargo")
        .args(&["check"])
        .current_dir(".")
        .output()
        .expect("Failed to execute cargo check");

    assert!(
        output.status.success(),
        "Code should compile without errors. Make sure User struct is properly defined. Stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
}

#[test]
fn test_user_display_format() {
    let output = Command::new("cargo")
        .args(&["run", "--quiet"])
        .current_dir(".")
        .output()
        .expect("Failed to execute cargo run");

    let stdout = String::from_utf8_lossy(&output.stdout);
    
    // Check that display_user function formats output correctly
    assert!(
        stdout.contains("User: alice@example.com (Alice Smith) - Active: true, Sign-in count: 1"),
        "Alice should be displayed with correct format"
    );
    assert!(
        stdout.contains("User: bob@example.com (Bob Jones) - Active: false, Sign-in count: 0"),
        "Bob should be displayed with correct format"
    );
}

#[test]
fn test_unit_tests_pass() {
    let output = Command::new("cargo")
        .args(&["test", "--quiet", "test_user_creation"])
        .current_dir(".")
        .output()
        .expect("Failed to execute cargo test");

    assert!(
        output.status.success(),
        "Unit tests should pass. This means the User struct is properly defined. Stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
}