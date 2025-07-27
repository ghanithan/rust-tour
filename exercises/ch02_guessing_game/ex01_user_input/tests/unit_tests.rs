use std::process::{Command, Stdio};
use std::io::Write;

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
fn test_program_runs_without_hanging() {
    // Test that program doesn't hang when given input
    let mut child = Command::new("cargo")
        .args(&["run"])
        .current_dir(".")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Failed to start program");

    // Send input to the program
    let stdin = child.stdin.as_mut().expect("Failed to open stdin");
    stdin.write_all(b"TestUser\n").expect("Failed to write to stdin");
    drop(stdin); // Close stdin to signal end of input

    let output = child.wait_with_output().expect("Failed to wait for program");
    
    assert!(
        output.status.success(),
        "Program should run successfully. Error output:\n{}",
        String::from_utf8_lossy(&output.stderr)
    );
}

#[test]
fn test_program_prompts_for_input() {
    let mut child = Command::new("cargo")
        .args(&["run"])
        .current_dir(".")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Failed to start program");

    let stdin = child.stdin.as_mut().expect("Failed to open stdin");
    stdin.write_all(b"Alice\n").expect("Failed to write to stdin");
    drop(stdin);

    let output = child.wait_with_output().expect("Failed to wait for program");
    
    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(
            stdout.to_lowercase().contains("name") || stdout.to_lowercase().contains("enter"),
            "Program should prompt the user for input. Output:\n{}",
            stdout
        );
    }
}

#[test]
fn test_program_uses_input_in_output() {
    let mut child = Command::new("cargo")
        .args(&["run"])
        .current_dir(".")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Failed to start program");

    let stdin = child.stdin.as_mut().expect("Failed to open stdin");
    stdin.write_all(b"Bob\n").expect("Failed to write to stdin");
    drop(stdin);

    let output = child.wait_with_output().expect("Failed to wait for program");
    
    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(
            stdout.contains("Bob"),
            "Program should use the input name 'Bob' in its output. Output:\n{}",
            stdout
        );
    }
}

#[test]
fn test_program_produces_greeting() {
    let mut child = Command::new("cargo")
        .args(&["run"])
        .current_dir(".")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Failed to start program");

    let stdin = child.stdin.as_mut().expect("Failed to open stdin");
    stdin.write_all(b"Charlie\n").expect("Failed to write to stdin");
    drop(stdin);

    let output = child.wait_with_output().expect("Failed to wait for program");
    
    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout).to_lowercase();
        assert!(
            stdout.contains("hello") || stdout.contains("welcome") || stdout.contains("hi"),
            "Program should produce a greeting message. Output:\n{}",
            String::from_utf8_lossy(&output.stdout)
        );
    }
}

#[test]
fn test_different_names_work() {
    let test_names = ["Alice", "Bob", "TestUser123", "José"];
    
    for name in &test_names {
        let mut child = Command::new("cargo")
            .args(&["run"])
            .current_dir(".")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .expect("Failed to start program");

        let stdin = child.stdin.as_mut().expect("Failed to open stdin");
        stdin.write_all(format!("{}\n", name).as_bytes()).expect("Failed to write to stdin");
        drop(stdin);

        let output = child.wait_with_output().expect("Failed to wait for program");
        
        if output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout);
            assert!(
                stdout.contains(name),
                "Program should work with name '{}' and include it in output. Output:\n{}",
                name, stdout
            );
        }
    }
}

#[test]
fn test_program_handles_whitespace_properly() {
    let mut child = Command::new("cargo")
        .args(&["run"])
        .current_dir(".")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Failed to start program");

    let stdin = child.stdin.as_mut().expect("Failed to open stdin");
    // Test with name that has spaces and extra whitespace
    stdin.write_all(b"  John Doe  \n").expect("Failed to write to stdin");
    drop(stdin);

    let output = child.wait_with_output().expect("Failed to wait for program");
    
    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        // Should contain the name but ideally without excessive whitespace
        assert!(
            stdout.contains("John Doe"),
            "Program should handle names with spaces properly. Output:\n{}",
            stdout
        );
    }
}