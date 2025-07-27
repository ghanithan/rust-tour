use std::process::Command;

#[test]
fn test_program_runs_successfully() {
    let output = Command::new("cargo")
        .args(&["run", "--quiet"])
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output()
        .expect("Failed to execute cargo run");

    assert!(
        output.status.success(),
        "Program should run without errors. Stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
}

#[test]
fn test_program_output() {
    let output = Command::new("cargo")
        .args(&["run", "--quiet"])
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output()
        .expect("Failed to execute cargo run");

    let stdout = String::from_utf8_lossy(&output.stdout);
    
    assert!(
        stdout.contains("Geometry Area Calculator"),
        "Output should contain the program title"
    );
    
    assert!(
        stdout.contains("Circle") && stdout.contains("78.54"),
        "Output should contain circle calculation (π × 5² ≈ 78.54)"
    );
    
    assert!(
        stdout.contains("Rectangle") && stdout.contains("24.00"),
        "Output should contain rectangle calculation (4 × 6 = 24)"
    );
    
    assert!(
        stdout.contains("Triangle") && stdout.contains("12.00"),
        "Output should contain triangle calculation (8 × 3 ÷ 2 = 12)"
    );
}

#[test]
fn test_documentation_exists() {
    // Check that the source file has been modified from the TODO template
    let source = std::fs::read_to_string("src/main.rs")
        .expect("Should be able to read main.rs");
    
    // Should have module-level documentation
    assert!(
        source.contains("//!") && !source.contains("TODO: Add a brief description"),
        "Should have module-level documentation (//!) without TODO markers"
    );
    
    // Should have function documentation
    assert!(
        source.contains("///") && source.lines().filter(|line| line.contains("///")).count() >= 3,
        "Should have documentation comments (///) for functions"
    );
    
    // Should have regular comments
    assert!(
        source.lines().filter(|line| line.trim_start().starts_with("//") && !line.contains("///") && !line.contains("//!")).count() >= 3,
        "Should have regular line comments (//)"
    );
    
    // Should have block comments
    assert!(
        source.contains("/*") && source.contains("*/"),
        "Should have block comments (/* */)"
    );
}

#[test]
fn test_no_remaining_todos() {
    let source = std::fs::read_to_string("src/main.rs")
        .expect("Should be able to read main.rs");
    
    assert!(
        !source.contains("TODO:"),
        "All TODO markers should be replaced with actual comments"
    );
}

#[test]
fn test_documentation_generation() {
    let output = Command::new("cargo")
        .args(&["doc", "--no-deps", "--quiet"])
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output()
        .expect("Failed to execute cargo doc");

    assert!(
        output.status.success(),
        "Documentation should generate without errors. Stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
}