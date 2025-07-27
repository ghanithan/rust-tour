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
fn test_cargo_toml_has_rand_dependency() {
    let cargo_content = std::fs::read_to_string("Cargo.toml")
        .expect("Should be able to read Cargo.toml");
    
    assert!(
        cargo_content.contains("rand"),
        "Cargo.toml should contain rand as a dependency for random number generation"
    );
}

#[test]
fn test_program_accepts_multiple_guesses() {
    let mut child = Command::new("cargo")
        .args(&["run"])
        .current_dir(".")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Failed to start program");

    let stdin = child.stdin.as_mut().expect("Failed to open stdin");
    // Send multiple guesses to test loop behavior
    stdin.write_all(b"1\n2\n3\n4\n5\n6\n7\n8\n9\n10\n").expect("Failed to write to stdin");
    drop(stdin);

    let output = child.wait_with_output().expect("Failed to wait for program");
    
    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        
        // Should show multiple attempt numbers, indicating loop behavior
        let attempt_mentions = stdout.matches("Attempt").count();
        assert!(
            attempt_mentions >= 2,
            "Program should show multiple attempts, indicating loop usage. Output:\n{}",
            stdout
        );
    }
}

#[test]
fn test_program_provides_feedback() {
    let mut child = Command::new("cargo")
        .args(&["run"])
        .current_dir(".")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Failed to start program");

    let stdin = child.stdin.as_mut().expect("Failed to open stdin");
    // Try a range of numbers to likely get feedback
    stdin.write_all(b"1\n2\n3\n4\n5\n6\n7\n8\n9\n10\n").expect("Failed to write to stdin");
    drop(stdin);

    let output = child.wait_with_output().expect("Failed to wait for program");
    
    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout).to_lowercase();
        
        // Should provide feedback like "too high", "too low", or "correct"
        let has_feedback = stdout.contains("too high") || 
                          stdout.contains("too low") || 
                          stdout.contains("correct") ||
                          stdout.contains("high") ||
                          stdout.contains("low");
        
        assert!(
            has_feedback,
            "Program should provide feedback about guesses. Output:\n{}",
            String::from_utf8_lossy(&output.stdout)
        );
    }
}

#[test]
fn test_program_shows_attempt_count() {
    let mut child = Command::new("cargo")
        .args(&["run"])
        .current_dir(".")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Failed to start program");

    let stdin = child.stdin.as_mut().expect("Failed to open stdin");
    stdin.write_all(b"5\n1\n10\n3\n7\n").expect("Failed to write to stdin");
    drop(stdin);

    let output = child.wait_with_output().expect("Failed to wait for program");
    
    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        
        // Should show attempt numbers
        let has_attempt_numbers = stdout.contains("Attempt #1") || 
                                 stdout.contains("Attempt 1") ||
                                 stdout.contains("attempt") && stdout.contains("1");
        
        assert!(
            has_attempt_numbers,
            "Program should show attempt numbers. Output:\n{}",
            stdout
        );
    }
}

#[test]
fn test_program_handles_correct_guess() {
    // This test tries to verify the program can handle a correct guess
    // Since the target is random, we'll test with a broad range
    let mut child = Command::new("cargo")
        .args(&["run"])
        .current_dir(".")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Failed to start program");

    let stdin = child.stdin.as_mut().expect("Failed to open stdin");
    // Try all possible numbers to ensure we hit the target
    stdin.write_all(b"1\n2\n3\n4\n5\n6\n7\n8\n9\n10\n").expect("Failed to write to stdin");
    drop(stdin);

    let output = child.wait_with_output().expect("Failed to wait for program");
    
    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout).to_lowercase();
        
        // Should show success message when correct
        let shows_success = stdout.contains("correct") || 
                           stdout.contains("got it") ||
                           stdout.contains("right") ||
                           stdout.contains("guessed it");
        
        assert!(
            shows_success,
            "Program should show success message for correct guess. Output:\n{}",
            String::from_utf8_lossy(&output.stdout)
        );
    }
}

#[test]
fn test_source_code_uses_loop() {
    let source_code = std::fs::read_to_string("src/main.rs")
        .expect("Should be able to read source code");
    
    assert!(
        source_code.contains("loop"),
        "Source code should use the 'loop' keyword for infinite loop. Current source:\n{}",
        source_code
    );
}

#[test]
fn test_source_code_uses_break() {
    let source_code = std::fs::read_to_string("src/main.rs")
        .expect("Should be able to read source code");
    
    assert!(
        source_code.contains("break"),
        "Source code should use 'break' to exit the loop. Current source:\n{}",
        source_code
    );
}

#[test]
fn test_program_handles_invalid_input_gracefully() {
    let mut child = Command::new("cargo")
        .args(&["run"])
        .current_dir(".")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Failed to start program");

    let stdin = child.stdin.as_mut().expect("Failed to open stdin");
    // Send invalid input followed by valid numbers
    stdin.write_all(b"abc\nxyz\n5\n1\n2\n3\n4\n6\n7\n8\n9\n10\n").expect("Failed to write to stdin");
    drop(stdin);

    let output = child.wait_with_output().expect("Failed to wait for program");
    
    // Program should handle invalid input without crashing
    assert!(
        output.status.success(),
        "Program should handle invalid input gracefully without crashing. Error output:\n{}",
        String::from_utf8_lossy(&output.stderr)
    );
}

#[test]
fn test_program_shows_range_information() {
    let mut child = Command::new("cargo")
        .args(&["run"])
        .current_dir(".")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Failed to start program");

    let stdin = child.stdin.as_mut().expect("Failed to open stdin");
    stdin.write_all(b"5\n").expect("Failed to write to stdin");
    drop(stdin);

    let output = child.wait_with_output().expect("Failed to wait for program");
    
    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        
        // Should mention the range (1 to 10 or similar)
        let mentions_range = stdout.contains("1") && stdout.contains("10") ||
                            stdout.contains("between") ||
                            stdout.contains("range");
        
        assert!(
            mentions_range,
            "Program should indicate the range of valid numbers. Output:\n{}",
            stdout
        );
    }
}

#[test]
fn test_program_tracks_attempts_correctly() {
    let mut child = Command::new("cargo")
        .args(&["run"])
        .current_dir(".")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Failed to start program");

    let stdin = child.stdin.as_mut().expect("Failed to open stdin");
    // Send a few guesses
    stdin.write_all(b"1\n5\n3\n8\n2\n9\n4\n7\n6\n10\n").expect("Failed to write to stdin");
    drop(stdin);

    let output = child.wait_with_output().expect("Failed to wait for program");
    
    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        
        // Look for sequential attempt numbers
        let has_sequential_attempts = (stdout.contains("Attempt #1") || stdout.contains("Attempt 1")) &&
                                     (stdout.contains("Attempt #2") || stdout.contains("Attempt 2"));
        
        assert!(
            has_sequential_attempts,
            "Program should track attempts with sequential numbering. Output:\n{}",
            stdout
        );
    }
}