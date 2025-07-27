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
fn test_program_shows_welcome_message() {
    let mut child = Command::new("cargo")
        .args(&["run"])
        .current_dir(".")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Failed to start program");

    let stdin = child.stdin.as_mut().expect("Failed to open stdin");
    stdin.write_all(b"quit\n").expect("Failed to write to stdin");
    drop(stdin);

    let output = child.wait_with_output().expect("Failed to wait for program");
    
    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout).to_lowercase();
        
        // Should have a welcome message mentioning the game
        let has_welcome = stdout.contains("welcome") || 
                         stdout.contains("guessing") ||
                         stdout.contains("game") ||
                         stdout.contains("number");
        
        assert!(
            has_welcome,
            "Program should show a welcome message. Output:\n{}",
            String::from_utf8_lossy(&output.stdout)
        );
    }
}

#[test]
fn test_program_handles_valid_guesses() {
    let mut child = Command::new("cargo")
        .args(&["run"])
        .current_dir(".")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Failed to start program");

    let stdin = child.stdin.as_mut().expect("Failed to open stdin");
    // Try several guesses to test game functionality
    stdin.write_all(b"50\n75\n25\n60\n40\n30\n35\n32\n36\n34\n33\nquit\n").expect("Failed to write to stdin");
    drop(stdin);

    let output = child.wait_with_output().expect("Failed to wait for program");
    
    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout).to_lowercase();
        
        // Should provide feedback about guesses
        let has_feedback = stdout.contains("too high") || 
                          stdout.contains("too low") ||
                          stdout.contains("correct") ||
                          stdout.contains("high") ||
                          stdout.contains("low");
        
        assert!(
            has_feedback,
            "Program should provide feedback about guesses (too high/low). Output:\n{}",
            String::from_utf8_lossy(&output.stdout)
        );
    }
}

#[test]
fn test_program_tracks_attempts() {
    let mut child = Command::new("cargo")
        .args(&["run"])
        .current_dir(".")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Failed to start program");

    let stdin = child.stdin.as_mut().expect("Failed to open stdin");
    stdin.write_all(b"1\n2\n3\nquit\n").expect("Failed to write to stdin");
    drop(stdin);

    let output = child.wait_with_output().expect("Failed to wait for program");
    
    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        
        // Should show attempt numbers
        let has_attempt_tracking = stdout.contains("Attempt #1") || 
                                  stdout.contains("Attempt 1") ||
                                  stdout.contains("attempt") && stdout.contains("1");
        
        assert!(
            has_attempt_tracking,
            "Program should track and display attempt numbers. Output:\n{}",
            stdout
        );
    }
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
    // Send various invalid inputs
    stdin.write_all(b"abc\nxyz\n-5\n101\n0\n50\nquit\n").expect("Failed to write to stdin");
    drop(stdin);

    let output = child.wait_with_output().expect("Failed to wait for program");
    
    // Program should not crash on invalid input
    assert!(
        output.status.success(),
        "Program should handle invalid input gracefully without crashing. Error output:\n{}",
        String::from_utf8_lossy(&output.stderr)
    );
    
    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout).to_lowercase();
        
        // Should show error messages for invalid input
        let has_error_handling = stdout.contains("error") || 
                               stdout.contains("invalid") ||
                               stdout.contains("please");
        
        assert!(
            has_error_handling,
            "Program should show error messages for invalid input. Output:\n{}",
            String::from_utf8_lossy(&output.stdout)
        );
    }
}

#[test]
fn test_program_handles_quit_command() {
    let mut child = Command::new("cargo")
        .args(&["run"])
        .current_dir(".")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Failed to start program");

    let stdin = child.stdin.as_mut().expect("Failed to open stdin");
    stdin.write_all(b"quit\n").expect("Failed to write to stdin");
    drop(stdin);

    let output = child.wait_with_output().expect("Failed to wait for program");
    
    assert!(
        output.status.success(),
        "Program should handle quit command properly. Error output:\n{}",
        String::from_utf8_lossy(&output.stderr)
    );
}

#[test]
fn test_program_supports_play_again() {
    let mut child = Command::new("cargo")
        .args(&["run"])
        .current_dir(".")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Failed to start program");

    let stdin = child.stdin.as_mut().expect("Failed to open stdin");
    // Try to simulate winning and then saying no to play again
    stdin.write_all(b"1\n2\n3\n4\n5\n6\n7\n8\n9\n10\n11\n12\n13\n14\n15\nn\n").expect("Failed to write to stdin");
    drop(stdin);

    let output = child.wait_with_output().expect("Failed to wait for program");
    
    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout).to_lowercase();
        
        // Should ask about playing again or show some indication of game completion
        let supports_replay = stdout.contains("play again") || 
                             stdout.contains("again") ||
                             stdout.contains("another") ||
                             stdout.contains("congratulations") ||
                             stdout.contains("correct");
        
        assert!(
            supports_replay,
            "Program should support playing again or show victory message. Output:\n{}",
            String::from_utf8_lossy(&output.stdout)
        );
    }
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
    stdin.write_all(b"quit\n").expect("Failed to write to stdin");
    drop(stdin);

    let output = child.wait_with_output().expect("Failed to wait for program");
    
    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        
        // Should mention the valid range (1 to 100)
        let mentions_range = (stdout.contains("1") && stdout.contains("100")) ||
                            stdout.contains("between") ||
                            stdout.contains("range");
        
        assert!(
            mentions_range,
            "Program should indicate the valid range of numbers. Output:\n{}",
            stdout
        );
    }
}

#[test]
fn test_source_code_uses_key_concepts() {
    let source_code = std::fs::read_to_string("src/main.rs")
        .expect("Should be able to read source code");
    
    // Should use key Chapter 2 concepts
    assert!(
        source_code.contains("rand"),
        "Source code should use rand for random number generation"
    );
    
    assert!(
        source_code.contains("loop"),
        "Source code should use loop for game control"
    );
    
    assert!(
        source_code.contains("match"),
        "Source code should use match expressions"
    );
    
    // Should minimize or eliminate expect() calls for proper error handling
    let expect_count = source_code.matches(".expect(").count();
    assert!(
        expect_count <= 2, // Allow minimal expect calls for flush or similar
        "Source code should minimize expect() calls for proper error handling. Found {} expect() calls",
        expect_count
    );
}

#[test]
fn test_program_provides_encouraging_feedback() {
    let mut child = Command::new("cargo")
        .args(&["run"])
        .current_dir(".")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Failed to start program");

    let stdin = child.stdin.as_mut().expect("Failed to open stdin");
    stdin.write_all(b"50\nquit\n").expect("Failed to write to stdin");
    drop(stdin);

    let output = child.wait_with_output().expect("Failed to wait for program");
    
    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout).to_lowercase();
        
        // Should provide encouraging or helpful feedback
        let has_encouraging_feedback = stdout.contains("try") || 
                                      stdout.contains("good") ||
                                      stdout.contains("can you") ||
                                      stdout.contains("think") ||
                                      stdout.contains("guess");
        
        assert!(
            has_encouraging_feedback,
            "Program should provide encouraging or helpful feedback. Output:\n{}",
            String::from_utf8_lossy(&output.stdout)
        );
    }
}

#[test]
fn test_program_handles_edge_cases() {
    let mut child = Command::new("cargo")
        .args(&["run"])
        .current_dir(".")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Failed to start program");

    let stdin = child.stdin.as_mut().expect("Failed to open stdin");
    // Test edge cases: empty input, out of range, zero, negative
    stdin.write_all(b"\n0\n-1\n101\n999999\nquit\n").expect("Failed to write to stdin");
    drop(stdin);

    let output = child.wait_with_output().expect("Failed to wait for program");
    
    // Program should handle all edge cases without crashing
    assert!(
        output.status.success(),
        "Program should handle edge cases gracefully. Error output:\n{}",
        String::from_utf8_lossy(&output.stderr)
    );
}