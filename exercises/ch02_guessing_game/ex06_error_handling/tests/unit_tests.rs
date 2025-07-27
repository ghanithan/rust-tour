use std::process::{Command, Stdio};
use std::io::Write;
use std::fs;

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
fn test_program_handles_missing_config_file() {
    // Remove config file if it exists
    let _ = fs::remove_file("config.txt");
    
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
    
    // Program should not panic when config file is missing
    assert!(
        output.status.success(),
        "Program should handle missing config file gracefully. Error output:\n{}",
        String::from_utf8_lossy(&output.stderr)
    );
    
    // Should indicate it's using defaults or warn about missing file
    let stdout = String::from_utf8_lossy(&output.stdout).to_lowercase();
    let handles_missing_file = stdout.contains("default") || 
                              stdout.contains("not found") ||
                              stdout.contains("warning") ||
                              stdout.contains("fallback");
    
    assert!(
        handles_missing_file,
        "Program should indicate it's handling missing config file. Output:\n{}",
        String::from_utf8_lossy(&output.stdout)
    );
}

#[test]
fn test_program_handles_invalid_number_input() {
    // Create a valid config file for this test
    fs::write("config.txt", "default_value=30").expect("Failed to create test config");
    
    let mut child = Command::new("cargo")
        .args(&["run"])
        .current_dir(".")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Failed to start program");

    let stdin = child.stdin.as_mut().expect("Failed to open stdin");
    // Send invalid input followed by quit
    stdin.write_all(b"abc\nxyz\n123.45\nquit\n").expect("Failed to write to stdin");
    drop(stdin);

    let output = child.wait_with_output().expect("Failed to wait for program");
    
    // Program should not panic on invalid input
    assert!(
        output.status.success(),
        "Program should handle invalid number input gracefully. Error output:\n{}",
        String::from_utf8_lossy(&output.stderr)
    );
    
    // Should show error messages for invalid input
    let stdout = String::from_utf8_lossy(&output.stdout).to_lowercase();
    let handles_invalid_input = stdout.contains("error") || 
                               stdout.contains("invalid") ||
                               stdout.contains("not a valid number") ||
                               stdout.contains("please");
    
    assert!(
        handles_invalid_input,
        "Program should show error messages for invalid input. Output:\n{}",
        String::from_utf8_lossy(&output.stdout)
    );
    
    // Clean up
    let _ = fs::remove_file("config.txt");
}

#[test]
fn test_program_handles_valid_input_with_config() {
    // Create a valid config file
    fs::write("config.txt", "default_value=25").expect("Failed to create test config");
    
    let mut child = Command::new("cargo")
        .args(&["run"])
        .current_dir(".")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Failed to start program");

    let stdin = child.stdin.as_mut().expect("Failed to open stdin");
    stdin.write_all(b"42\n").expect("Failed to write to stdin");
    drop(stdin);

    let output = child.wait_with_output().expect("Failed to wait for program");
    
    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        
        // Should process the valid number
        assert!(
            stdout.contains("42"),
            "Program should process valid number input. Output:\n{}",
            stdout
        );
        
        // Should show doubled number
        assert!(
            stdout.contains("84"), // 42 * 2
            "Program should show doubled number (84). Output:\n{}",
            stdout
        );
    }
    
    // Clean up
    let _ = fs::remove_file("config.txt");
}

#[test]
fn test_program_handles_invalid_config_file() {
    // Create an invalid config file
    fs::write("config.txt", "invalid_config_format\ngarbage_data").expect("Failed to create test config");
    
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
    
    // Program should not panic on invalid config
    assert!(
        output.status.success(),
        "Program should handle invalid config file gracefully. Error output:\n{}",
        String::from_utf8_lossy(&output.stderr)
    );
    
    // Clean up
    let _ = fs::remove_file("config.txt");
}

#[test]
fn test_source_code_no_expect_calls() {
    let source_code = fs::read_to_string("src/main.rs")
        .expect("Should be able to read source code");
    
    // Count expect calls - there should be very few or none in the final solution
    let expect_count = source_code.matches(".expect(").count();
    
    assert!(
        expect_count <= 1, // Allow maybe one expect for test file creation or similar
        "Source code should minimize or eliminate expect() calls for proper error handling. Found {} expect() calls",
        expect_count
    );
}

#[test]
fn test_source_code_uses_match_for_results() {
    let source_code = fs::read_to_string("src/main.rs")
        .expect("Should be able to read source code");
    
    // Should use match statements for error handling
    assert!(
        source_code.contains("match") && source_code.contains("Ok") && source_code.contains("Err"),
        "Source code should use match statements with Ok/Err for error handling. Current source:\n{}",
        source_code
    );
}

#[test]
fn test_program_allows_quit_command() {
    // Test that quit command works properly
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
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.to_lowercase().contains("goodbye") || stdout.to_lowercase().contains("quit"),
        "Program should acknowledge quit command. Output:\n{}",
        stdout
    );
}

#[test]
fn test_program_provides_helpful_error_messages() {
    let mut child = Command::new("cargo")
        .args(&["run"])
        .current_dir(".")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Failed to start program");

    let stdin = child.stdin.as_mut().expect("Failed to open stdin");
    stdin.write_all(b"not_a_number\nquit\n").expect("Failed to write to stdin");
    drop(stdin);

    let output = child.wait_with_output().expect("Failed to wait for program");
    
    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout).to_lowercase();
        
        // Should provide helpful error messages
        let has_helpful_messages = stdout.contains("error") || 
                                  stdout.contains("invalid") ||
                                  stdout.contains("please") ||
                                  stdout.contains("try");
        
        assert!(
            has_helpful_messages,
            "Program should provide helpful error messages. Output:\n{}",
            String::from_utf8_lossy(&output.stdout)
        );
    }
}

#[test]
fn test_program_continues_after_errors() {
    let mut child = Command::new("cargo")
        .args(&["run"])
        .current_dir(".")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Failed to start program");

    let stdin = child.stdin.as_mut().expect("Failed to open stdin");
    // Send invalid input followed by valid input
    stdin.write_all(b"invalid\n42\n").expect("Failed to write to stdin");
    drop(stdin);

    let output = child.wait_with_output().expect("Failed to wait for program");
    
    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        
        // Should process the valid input after handling the invalid input
        assert!(
            stdout.contains("42"),
            "Program should continue and process valid input after handling errors. Output:\n{}",
            stdout
        );
    }
}