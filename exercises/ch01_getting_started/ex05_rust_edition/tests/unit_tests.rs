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
        stdout.contains("Rust Edition Demonstration"),
        "Output should contain the program title"
    );
    
    assert!(
        stdout.contains("Array Iteration") && stdout.contains("2021 Edition Feature"),
        "Output should contain array iteration demonstration"
    );
    
    assert!(
        stdout.contains("Backward Compatibility"),
        "Output should contain backward compatibility section"
    );
    
    assert!(
        stdout.contains("Edition System Explanation"),
        "Output should contain edition system explanation"
    );
    
    assert!(
        stdout.contains("Edition Demo Complete"),
        "Output should contain completion message"
    );
}

#[test]
fn test_demonstrates_edition_features() {
    let output = Command::new("cargo")
        .args(&["run", "--quiet"])
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output()
        .expect("Failed to execute cargo run");

    let stdout = String::from_utf8_lossy(&output.stdout);
    
    // Should demonstrate array iteration
    assert!(
        stdout.contains("Direct iteration") && stdout.contains("Iterator method"),
        "Should demonstrate both direct and iterator-based array iteration"
    );
    
    // Should show edition information
    assert!(
        stdout.contains("Rust 2015") && stdout.contains("Rust 2018") && stdout.contains("Rust 2021"),
        "Should mention all three Rust editions"
    );
    
    // Should explain benefits
    assert!(
        stdout.contains("compatibility") || stdout.contains("migration") || stdout.contains("stability"),
        "Should explain edition system benefits"
    );
}

#[test]
fn test_cargo_toml_has_correct_edition() {
    let cargo_toml = std::fs::read_to_string("Cargo.toml")
        .expect("Should be able to read Cargo.toml");
    
    assert!(
        cargo_toml.contains("edition = \"2021\""),
        "Cargo.toml should specify edition = \"2021\""
    );
}

#[test]
fn test_source_demonstrates_edition_concepts() {
    let source = std::fs::read_to_string("src/main.rs")
        .expect("Should be able to read main.rs");
    
    // Should have module-level documentation
    assert!(
        source.contains("//!") && !source.contains("TODO: Add module-level documentation"),
        "Should have module-level documentation without TODO markers"
    );
    
    // Should demonstrate array iteration patterns
    assert!(
        source.contains("for") && source.contains("numbers"),
        "Should demonstrate iteration over arrays"
    );
    
    // Should use HashMap (2018+ module improvements)
    assert!(
        source.contains("HashMap") || source.contains("std::collections"),
        "Should demonstrate module system improvements with HashMap"
    );
    
    // Should call demonstration functions
    assert!(
        source.contains("demonstrate_array_iteration") && !source.contains("// demonstrate_array_iteration"),
        "Should call demonstrate_array_iteration function"
    );
    
    assert!(
        source.contains("demonstrate_backward_compatibility") && !source.contains("// demonstrate_backward_compatibility"),
        "Should call demonstrate_backward_compatibility function"
    );
    
    assert!(
        source.contains("explain_edition_system") && !source.contains("// explain_edition_system"),
        "Should call explain_edition_system function"
    );
}

#[test]
fn test_no_remaining_todos() {
    let source = std::fs::read_to_string("src/main.rs")
        .expect("Should be able to read main.rs");
    
    assert!(
        !source.contains("TODO:"),
        "All TODO markers should be replaced with actual implementation"
    );
}

#[test]
fn test_demonstrates_iteration_patterns() {
    let output = Command::new("cargo")
        .args(&["run", "--quiet"])
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output()
        .expect("Failed to execute cargo run");

    let stdout = String::from_utf8_lossy(&output.stdout);
    
    // Should show numbers being iterated in both ways
    assert!(
        stdout.contains("1 2 3 4 5") || stdout.contains("1") && stdout.contains("2") && stdout.contains("3"),
        "Should show array elements being iterated"
    );
}

#[test]
fn test_code_compiles_with_clippy() {
    let output = Command::new("cargo")
        .args(&["clippy", "--", "-D", "warnings"])
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output()
        .expect("Failed to execute cargo clippy");

    assert!(
        output.status.success(),
        "Code should pass clippy lints. Stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
}

#[test]
fn test_demonstrates_closure_usage() {
    let source = std::fs::read_to_string("src/main.rs")
        .expect("Should be able to read main.rs");
    
    // Should have a closure example
    assert!(
        source.contains("||") || source.contains("|"),
        "Should demonstrate closure usage (edition 2021 improvement)"
    );
}