use std::process::Command;

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
fn test_program_runs_successfully() {
    let output = Command::new("cargo")
        .args(&["run", "--quiet"])
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output()
        .expect("Failed to execute cargo run");

    assert!(
        output.status.success(),
        "Program should run without panics. Stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
}

#[test]
fn test_correct_output() {
    let output = Command::new("cargo")
        .args(&["run", "--quiet"])
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output()
        .expect("Failed to execute cargo run");

    let stdout = String::from_utf8_lossy(&output.stdout);
    
    assert!(
        stdout.contains("Hello from Rust!"),
        "Output should contain 'Hello from Rust!'"
    );
    
    assert!(
        stdout.contains("The answer is: 42"),
        "Output should contain 'The answer is: 42'"
    );
    
    assert!(
        stdout.contains("Rust compilation is awesome!"),
        "Output should contain 'Rust compilation is awesome!'"
    );
    
    assert!(
        stdout.contains("Compilation errors are learning opportunities!"),
        "Output should contain 'Compilation errors are learning opportunities!'"
    );
}

#[test]
fn test_no_compilation_warnings() {
    let output = Command::new("cargo")
        .args(&["build", "--quiet"])
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output()
        .expect("Failed to execute cargo build");

    let stderr = String::from_utf8_lossy(&output.stderr);
    
    // Should compile with no warnings
    assert!(
        !stderr.contains("warning:"),
        "Code should compile without warnings. Found: {}",
        stderr
    );
}