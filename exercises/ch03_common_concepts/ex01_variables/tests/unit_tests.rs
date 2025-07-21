use std::process::Command;

#[test]
fn test_code_compiles_successfully() {
    let output = Command::new("cargo")
        .args(&["check"])
        .current_dir(".")
        .output()
        .expect("Failed to execute cargo check");

    assert!(
        output.status.success(),
        "Code should compile without errors. Compiler output:\n{}\nError output:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
}

#[test]
fn test_program_runs_without_panics() {
    let output = Command::new("cargo")
        .args(&["run"])
        .current_dir(".")
        .output()
        .expect("Failed to execute cargo run");

    assert!(
        output.status.success(),
        "Program should run without panicking. Error output:\n{}",
        String::from_utf8_lossy(&output.stderr)
    );
}

#[test]
fn test_demonstrates_mutability() {
    let source_code = std::fs::read_to_string("src/main.rs")
        .expect("Failed to read src/main.rs");

    // Check that mut keyword is used appropriately
    assert!(
        source_code.contains("let mut"),
        "Code should demonstrate mutable variables using 'let mut'"
    );

    // Check that constants are used
    assert!(
        source_code.contains("const"),
        "Code should demonstrate constants using 'const'"
    );
}

#[test]
fn test_uses_proper_constant_syntax() {
    let source_code = std::fs::read_to_string("src/main.rs")
        .expect("Failed to read src/main.rs");

    // Check for proper constant naming (ALL_CAPS)
    let has_proper_const = source_code.contains("const") && 
                          (source_code.contains("SECONDS_IN_MINUTE") || 
                           source_code.contains("MAX_POINTS") ||
                           source_code.contains("_"));

    assert!(
        has_proper_const,
        "Constants should use ALL_CAPS naming convention"
    );
}

#[test]
fn test_demonstrates_shadowing() {
    let source_code = std::fs::read_to_string("src/main.rs")
        .expect("Failed to read src/main.rs");

    // Count occurrences of "let" to ensure shadowing is used
    let let_count = source_code.matches("let ").count();
    
    assert!(
        let_count > 5, // Should have multiple let statements for shadowing
        "Code should demonstrate shadowing by using 'let' multiple times with same variable name"
    );
}

#[test]
fn test_no_compilation_errors_remain() {
    let source_code = std::fs::read_to_string("src/main.rs")
        .expect("Failed to read src/main.rs");

    // These patterns would cause compilation errors if present
    let problematic_patterns = [
        "BUG:", // Should have removed bug comments
        "This will cause", // Should have fixed error-causing code
        "This should work, but doesn't", // Should have fixed this
    ];

    for pattern in &problematic_patterns {
        assert!(
            !source_code.contains(pattern),
            "Code should not contain the pattern '{}' - this suggests bugs weren't fixed",
            pattern
        );
    }
}

#[test]
fn test_output_contains_expected_sections() {
    let output = Command::new("cargo")
        .args(&["run"])
        .current_dir(".")
        .output()
        .expect("Failed to execute cargo run");

    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        
        // Check that all sections are represented in output
        let expected_sections = [
            "Section 1: Basic Variables",
            "Section 2: Shadowing", 
            "Section 3: Constants",
            "Section 4: Multiple Mutations",
            "Section 5: Type Changes"
        ];

        for section in &expected_sections {
            assert!(
                stdout.contains(section),
                "Program output should contain '{}' section",
                section
            );
        }
    }
}

#[test]
fn test_demonstrates_type_changing_with_shadowing() {
    let source_code = std::fs::read_to_string("src/main.rs")
        .expect("Failed to read src/main.rs");

    // Should demonstrate changing types with shadowing (string to number)
    assert!(
        source_code.contains("parse") || source_code.contains(".len()"),
        "Code should demonstrate type changes through shadowing (e.g., string to number conversion)"
    );
}