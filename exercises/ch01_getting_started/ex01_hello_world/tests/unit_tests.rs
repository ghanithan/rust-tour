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
fn test_program_runs_successfully() {
    let output = Command::new("cargo")
        .args(&["run"])
        .current_dir(".")
        .output()
        .expect("Failed to execute cargo run");

    assert!(
        output.status.success(),
        "Program should run successfully without errors. Error output:\n{}",
        String::from_utf8_lossy(&output.stderr)
    );
}

#[test]
fn test_program_outputs_hello_world() {
    let output = Command::new("cargo")
        .args(&["run"])
        .current_dir(".")
        .output()
        .expect("Failed to execute cargo run");

    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(
            stdout.contains("Hello, world!"),
            "Program output should contain 'Hello, world!'. Actual output:\n{}",
            stdout
        );
    }
}

#[test]
fn test_output_is_properly_formatted() {
    let output = Command::new("cargo")
        .args(&["run"])
        .current_dir(".")
        .output()
        .expect("Failed to execute cargo run");

    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        let stdout_trimmed = stdout.trim();

        // Test that output contains the greeting
        assert!(
            stdout_trimmed.contains("Hello, world!"),
            "Output should contain the greeting 'Hello, world!'"
        );

        // Test that output doesn't contain compilation or debug information
        assert!(
            !stdout_trimmed.contains("Compiling") && !stdout_trimmed.contains("Finished"),
            "Output should be clean program output without build information"
        );
    }
}

#[test]
fn test_program_terminates_normally() {
    let output = Command::new("cargo")
        .args(&["run"])
        .current_dir(".")
        .output()
        .expect("Failed to execute cargo run");

    assert_eq!(
        output.status.code(),
        Some(0),
        "Program should terminate with exit code 0 (normal termination)"
    );
}

#[test]
fn test_program_produces_text_output() {
    let output = Command::new("cargo")
        .args(&["run"])
        .current_dir(".")
        .output()
        .expect("Failed to execute cargo run");

    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);

        // Test that program actually produces output (not silent)
        assert!(
            !stdout.trim().is_empty(),
            "Program should produce visible output, not run silently"
        );

        // Test that output is text-based (contains printable characters)
        assert!(
            stdout
                .chars()
                .any(|c| c.is_ascii_graphic() || c.is_whitespace()),
            "Program should produce readable text output"
        );
    }
}

#[test]
fn test_program_behavior_is_consistent() {
    // Run the program multiple times to ensure consistent behavior
    for i in 1..=3 {
        let output = Command::new("cargo")
            .args(&["run"])
            .current_dir(".")
            .output()
            .expect("Failed to execute cargo run");

        assert!(
            output.status.success(),
            "Program should run consistently on iteration {}",
            i
        );

        if output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout);
            assert!(
                stdout.contains("Hello, world!"),
                "Program should produce consistent output on iteration {}",
                i
            );
        }
    }
}
