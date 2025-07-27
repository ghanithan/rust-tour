use std::process::Command;
use std::io::Write;

#[test]
fn test_program_compiles() {
    let output = Command::new("cargo")
        .args(&["build", "--quiet"])
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output()
        .expect("Failed to execute cargo build");

    assert!(
        output.status.success(),
        "Program should compile without errors. Stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
}

#[test]
fn test_addition_operation() {
    let mut child = Command::new("cargo")
        .args(&["run", "--quiet"])
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .spawn()
        .expect("Failed to execute cargo run");

    let stdin = child.stdin.as_mut().expect("Failed to open stdin");
    // Provide input: 10, 5, +
    stdin.write_all(b"10\n5\n+\n").expect("Failed to write to stdin");
    
    let output = child.wait_with_output().expect("Failed to wait for output");
    let stdout = String::from_utf8_lossy(&output.stdout);
    
    assert!(
        stdout.contains("15") || stdout.contains("15.0"),
        "Addition of 10 + 5 should equal 15. Output: {}",
        stdout
    );
}

#[test]
fn test_subtraction_operation() {
    let mut child = Command::new("cargo")
        .args(&["run", "--quiet"])
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .spawn()
        .expect("Failed to execute cargo run");

    let stdin = child.stdin.as_mut().expect("Failed to open stdin");
    // Provide input: 20, 8, -
    stdin.write_all(b"20\n8\n-\n").expect("Failed to write to stdin");
    
    let output = child.wait_with_output().expect("Failed to wait for output");
    let stdout = String::from_utf8_lossy(&output.stdout);
    
    assert!(
        stdout.contains("12") || stdout.contains("12.0"),
        "Subtraction of 20 - 8 should equal 12. Output: {}",
        stdout
    );
}

#[test]
fn test_multiplication_operation() {
    let mut child = Command::new("cargo")
        .args(&["run", "--quiet"])
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .spawn()
        .expect("Failed to execute cargo run");

    let stdin = child.stdin.as_mut().expect("Failed to open stdin");
    // Provide input: 6, 7, *
    stdin.write_all(b"6\n7\n*\n").expect("Failed to write to stdin");
    
    let output = child.wait_with_output().expect("Failed to wait for output");
    let stdout = String::from_utf8_lossy(&output.stdout);
    
    assert!(
        stdout.contains("42") || stdout.contains("42.0"),
        "Multiplication of 6 * 7 should equal 42. Output: {}",
        stdout
    );
}

#[test]
fn test_division_operation() {
    let mut child = Command::new("cargo")
        .args(&["run", "--quiet"])
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .spawn()
        .expect("Failed to execute cargo run");

    let stdin = child.stdin.as_mut().expect("Failed to open stdin");
    // Provide input: 50, 10, /
    stdin.write_all(b"50\n10\n/\n").expect("Failed to write to stdin");
    
    let output = child.wait_with_output().expect("Failed to wait for output");
    let stdout = String::from_utf8_lossy(&output.stdout);
    
    assert!(
        stdout.contains("5") || stdout.contains("5.0"),
        "Division of 50 / 10 should equal 5. Output: {}",
        stdout
    );
}

#[test]
fn test_division_by_zero_handling() {
    let mut child = Command::new("cargo")
        .args(&["run", "--quiet"])
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .spawn()
        .expect("Failed to execute cargo run");

    let stdin = child.stdin.as_mut().expect("Failed to open stdin");
    // Provide input: 10, 0, /
    stdin.write_all(b"10\n0\n/\n").expect("Failed to write to stdin");
    
    let output = child.wait_with_output().expect("Failed to wait for output");
    let stdout = String::from_utf8_lossy(&output.stdout);
    
    assert!(
        stdout.contains("cannot") || stdout.contains("zero") || stdout.contains("error") || stdout.contains("Error"),
        "Division by zero should be handled with an error message. Output: {}",
        stdout
    );
}

#[test]
fn test_program_structure() {
    let source = std::fs::read_to_string("src/main.rs")
        .expect("Should be able to read main.rs");
    
    // Should have helper functions
    assert!(
        source.contains("fn ") && source.lines().filter(|line| line.contains("fn ")).count() >= 2,
        "Should have helper functions in addition to main"
    );
    
    // Should have proper documentation
    assert!(
        source.contains("///") || source.contains("//!"),
        "Should have documentation comments"
    );
    
    // Should handle user input
    assert!(
        source.contains("read_line") || source.contains("stdin"),
        "Should read user input"
    );
    
    // Should parse strings to numbers
    assert!(
        source.contains("parse"),
        "Should parse string input to numbers"
    );
}

#[test]
fn test_invalid_number_handling() {
    let mut child = Command::new("cargo")
        .args(&["run", "--quiet"])
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .spawn()
        .expect("Failed to execute cargo run");

    let stdin = child.stdin.as_mut().expect("Failed to open stdin");
    // Provide input: invalid, 5, +
    stdin.write_all(b"abc\n5\n+\n").expect("Failed to write to stdin");
    
    let output = child.wait_with_output().expect("Failed to wait for output");
    let stdout = String::from_utf8_lossy(&output.stdout);
    
    // Program should either handle the error gracefully or re-prompt
    assert!(
        stdout.contains("invalid") || stdout.contains("error") || stdout.contains("number") || 
        stdout.contains("Error") || stdout.contains("try again"),
        "Should handle invalid number input gracefully. Output: {}",
        stdout
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