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
        "Program should compile and run successfully. Fix the struct update syntax and field shorthand bugs. Stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
}

#[test]
fn test_struct_update_syntax_output() {
    let output = Command::new("cargo")
        .args(&["run", "--quiet"])
        .current_dir(".")
        .output()
        .expect("Failed to execute cargo run");

    let stdout = String::from_utf8_lossy(&output.stdout);
    
    // Test that struct update syntax works correctly
    assert!(
        stdout.contains("Alice Johnson"),
        "Should show updated username 'Alice Johnson' from struct update syntax"
    );
    
    assert!(
        stdout.contains("alice@example.com"),
        "Should preserve original email when using struct update syntax"
    );
}

#[test]
fn test_field_init_shorthand_output() {
    let output = Command::new("cargo")
        .args(&["run", "--quiet"])
        .current_dir(".")
        .output()
        .expect("Failed to execute cargo run");

    let stdout = String::from_utf8_lossy(&output.stdout);
    
    // Test that field init shorthand works
    assert!(
        stdout.contains("Bob Wilson"),
        "Should create user with field init shorthand"
    );
    
    assert!(
        stdout.contains("bob@example.com"),
        "Should create user with field init shorthand"
    );
}

#[test]
fn test_admin_user_creation() {
    let output = Command::new("cargo")
        .args(&["run", "--quiet"])
        .current_dir(".")
        .output()
        .expect("Failed to execute cargo run");

    let stdout = String::from_utf8_lossy(&output.stdout);
    
    // Test admin user created with update syntax
    assert!(
        stdout.contains("admin@company.com"),
        "Admin user should have correct email"
    );
    
    assert!(
        stdout.contains("Sign-in count: 100"),
        "Admin user should have sign-in count of 100"
    );
}

#[test]
fn test_ownership_after_update() {
    let output = Command::new("cargo")
        .args(&["run", "--quiet"])
        .current_dir(".")
        .output()
        .expect("Failed to execute cargo run");

    let stdout = String::from_utf8_lossy(&output.stdout);
    
    // Test that template_user is still accessible after update
    assert!(
        stdout.contains("Template user email:"),
        "Should be able to access template user after struct update"
    );
    
    assert!(
        stdout.contains("Admin was created using update syntax"),
        "Should print final message about update syntax"
    );
}

#[test]
fn test_unit_tests_pass() {
    let output = Command::new("cargo")
        .args(&["test", "--quiet", "test_struct_update_syntax"])
        .current_dir(".")
        .output()
        .expect("Failed to execute cargo test");

    assert!(
        output.status.success(),
        "Unit test for struct update syntax should pass. Stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    
    let output = Command::new("cargo")
        .args(&["test", "--quiet", "test_field_init_shorthand"])
        .current_dir(".")
        .output()
        .expect("Failed to execute cargo test");

    assert!(
        output.status.success(),
        "Unit test for field init shorthand should pass. Stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
}