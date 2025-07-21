use std::process::Command;

#[test]
fn test_program_compiles() {
    let output = Command::new("cargo")
        .args(&["check"])
        .current_dir(".")
        .output()
        .expect("Failed to execute cargo check");

    assert!(
        output.status.success(),
        "Program should compile successfully. Compiler output:\n{}",
        String::from_utf8_lossy(&output.stderr)
    );
}

#[test]
fn test_program_runs_and_outputs_hello_world() {
    let output = Command::new("cargo")
        .args(&["run"])
        .current_dir(".")
        .output()
        .expect("Failed to execute cargo run");

    assert!(
        output.status.success(),
        "Program should run successfully. Error output:\n{}",
        String::from_utf8_lossy(&output.stderr)
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("Hello, world!"),
        "Program output should contain 'Hello, world!'. Actual output:\n{}",
        stdout
    );
}

#[test]
fn test_output_format_is_correct() {
    let output = Command::new("cargo")
        .args(&["run"])
        .current_dir(".")
        .output()
        .expect("Failed to execute cargo run");

    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout).trim().to_string();
        assert_eq!(
            stdout,
            "Hello, world!",
            "Output should be exactly 'Hello, world!' (with no extra text or formatting)"
        );
    }
}

#[test]
fn test_no_hardcoded_solutions() {
    let source_code = std::fs::read_to_string("src/main.rs")
        .expect("Failed to read main.rs");

    // This test ensures students actually implement the solution
    // rather than just removing the TODO comment
    assert!(
        !source_code.contains("TODO"),
        "Remove the TODO comment and implement the solution"
    );

    assert!(
        source_code.contains("println!"),
        "Solution should use the println! macro"
    );

    assert!(
        source_code.contains("Hello, world!"),
        "Solution should print the correct text"
    );
}

#[test]
fn test_uses_main_function() {
    let source_code = std::fs::read_to_string("src/main.rs")
        .expect("Failed to read main.rs");

    assert!(
        source_code.contains("fn main()"),
        "Program should have a main function"
    );
}