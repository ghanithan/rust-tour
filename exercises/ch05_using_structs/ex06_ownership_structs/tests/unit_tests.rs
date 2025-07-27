use std::process::Command;

#[test]
fn test_compilation_success() {
    let output = Command::new("cargo")
        .args(&["check"])
        .current_dir(".")
        .output()
        .expect("Failed to execute cargo check");
    
    assert!(
        output.status.success(),
        "Code should compile successfully after fixing ownership issues. Stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
}

#[test]
fn test_program_runs_without_panic() {
    let output = Command::new("cargo")
        .args(&["run", "--quiet"])
        .current_dir(".")
        .output()
        .expect("Failed to execute cargo run");
    
    assert!(
        output.status.success(),
        "Program should run without panicking. Stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
}

#[test]
fn test_expected_output() {
    let output = Command::new("cargo")
        .args(&["run", "--quiet"])
        .current_dir(".")
        .output()
        .expect("Failed to execute cargo run");
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    
    // Check for expected output indicating the program completed successfully
    assert!(
        stdout.contains("Welcome to City Library"),
        "Output should contain library welcome message"
    );
    
    assert!(
        stdout.contains("Books processed successfully!"),
        "Output should contain success message"
    );
}

#[test]
fn test_no_ownership_violations() {
    // This test ensures the code compiles, which means ownership issues are resolved
    let output = Command::new("cargo")
        .args(&["clippy", "--", "-D", "warnings"])
        .current_dir(".")
        .output()
        .expect("Failed to execute cargo clippy");
    
    assert!(
        output.status.success(),
        "Code should pass clippy checks. Stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
}