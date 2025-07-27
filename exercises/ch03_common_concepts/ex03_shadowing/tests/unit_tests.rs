use std::process::Command;

#[test]
fn test_code_compiles() {
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
fn test_program_runs_successfully() {
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
fn test_area_calculation() {
    let output = Command::new("cargo")
        .args(&["run"])
        .current_dir(".")
        .output()
        .expect("Failed to execute cargo run");

    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        
        // Area of circle with radius 3: π * 3² = 3.14159 * 9 = 28.27451
        assert!(
            stdout.contains("Circle area: 28.27"),
            "Should calculate circle area as 28.27 (π * 3²). Output:\n{}",
            stdout
        );
    }
}

#[test]
fn test_string_processing() {
    let output = Command::new("cargo")
        .args(&["run"])
        .current_dir(".")
        .output()
        .expect("Failed to execute cargo run");

    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        
        // Should trim "  42  " to "42"
        assert!(
            stdout.contains("Trimmed input: '42'"),
            "Should trim whitespace to show '42'. Output:\n{}",
            stdout
        );
        
        // Should parse "42" to number 42
        assert!(
            stdout.contains("Parsed number: 42"),
            "Should parse string to number 42. Output:\n{}",
            stdout
        );
    }
}

#[test]
fn test_type_conversions() {
    let output = Command::new("cargo")
        .args(&["run"])
        .current_dir(".")
        .output()
        .expect("Failed to execute cargo run");

    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        
        // "123" → 123 (i32)
        assert!(
            stdout.contains("As integer: 123"),
            "Should convert '123' to integer 123. Output:\n{}",
            stdout
        );
        
        // 123 * 1.5 = 184.5 (f64)
        assert!(
            stdout.contains("As float: 184.5"),
            "Should convert 123 * 1.5 = 184.5. Output:\n{}",
            stdout
        );
        
        // Back to string "184.5"
        assert!(
            stdout.contains("Back to string: 184.5"),
            "Should convert 184.5 back to string. Output:\n{}",
            stdout
        );
    }
}

#[test]
fn test_scope_shadowing() {
    let output = Command::new("cargo")
        .args(&["run"])
        .current_dir(".")
        .output()
        .expect("Failed to execute cargo run");

    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        
        // Should show outer, inner, then outer again
        assert!(
            stdout.contains("Outer message: outer scope") &&
            stdout.contains("Inner message: inner scope") &&
            stdout.contains("Back to outer message: outer scope"),
            "Should demonstrate scope-based shadowing: outer → inner → outer. Output:\n{}",
            stdout
        );
    }
}