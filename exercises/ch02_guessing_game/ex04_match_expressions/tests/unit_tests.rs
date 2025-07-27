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
fn test_program_handles_first_number_smaller() {
    let mut child = Command::new("cargo")
        .args(&["run"])
        .current_dir(".")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Failed to start program");

    let stdin = child.stdin.as_mut().expect("Failed to open stdin");
    stdin.write_all(b"15\n25\n").expect("Failed to write to stdin");
    drop(stdin);

    let output = child.wait_with_output().expect("Failed to wait for program");
    
    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout).to_lowercase();
        
        // Should indicate first number is smaller/less
        let indicates_smaller = stdout.contains("smaller") || 
                               stdout.contains("less") ||
                               stdout.contains("first") && (stdout.contains("smaller") || stdout.contains("less"));
        
        assert!(
            indicates_smaller,
            "Program should indicate that 15 is smaller than 25. Output:\n{}",
            String::from_utf8_lossy(&output.stdout)
        );
    }
}

#[test]
fn test_program_handles_first_number_greater() {
    let mut child = Command::new("cargo")
        .args(&["run"])
        .current_dir(".")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Failed to start program");

    let stdin = child.stdin.as_mut().expect("Failed to open stdin");
    stdin.write_all(b"40\n20\n").expect("Failed to write to stdin");
    drop(stdin);

    let output = child.wait_with_output().expect("Failed to wait for program");
    
    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout).to_lowercase();
        
        // Should indicate first number is greater/larger
        let indicates_greater = stdout.contains("greater") || 
                               stdout.contains("larger") ||
                               stdout.contains("bigger") ||
                               stdout.contains("first") && (stdout.contains("greater") || stdout.contains("larger"));
        
        assert!(
            indicates_greater,
            "Program should indicate that 40 is greater than 20. Output:\n{}",
            String::from_utf8_lossy(&output.stdout)
        );
    }
}

#[test]
fn test_program_handles_equal_numbers() {
    let mut child = Command::new("cargo")
        .args(&["run"])
        .current_dir(".")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Failed to start program");

    let stdin = child.stdin.as_mut().expect("Failed to open stdin");
    stdin.write_all(b"30\n30\n").expect("Failed to write to stdin");
    drop(stdin);

    let output = child.wait_with_output().expect("Failed to wait for program");
    
    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout).to_lowercase();
        
        // Should indicate numbers are equal
        let indicates_equal = stdout.contains("equal") || 
                             stdout.contains("same") ||
                             stdout.contains("identical");
        
        assert!(
            indicates_equal,
            "Program should indicate that 30 equals 30. Output:\n{}",
            String::from_utf8_lossy(&output.stdout)
        );
    }
}

#[test]
fn test_program_shows_both_numbers() {
    let mut child = Command::new("cargo")
        .args(&["run"])
        .current_dir(".")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Failed to start program");

    let stdin = child.stdin.as_mut().expect("Failed to open stdin");
    stdin.write_all(b"7\n13\n").expect("Failed to write to stdin");
    drop(stdin);

    let output = child.wait_with_output().expect("Failed to wait for program");
    
    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        
        // Should show both input numbers in the output
        assert!(
            stdout.contains("7") && stdout.contains("13"),
            "Program should display both input numbers (7 and 13). Output:\n{}",
            stdout
        );
    }
}

#[test]
fn test_program_uses_comparison_language() {
    let mut child = Command::new("cargo")
        .args(&["run"])
        .current_dir(".")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Failed to start program");

    let stdin = child.stdin.as_mut().expect("Failed to open stdin");
    stdin.write_all(b"5\n10\n").expect("Failed to write to stdin");
    drop(stdin);

    let output = child.wait_with_output().expect("Failed to wait for program");
    
    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout).to_lowercase();
        
        // Should use comparison-related language
        let has_comparison_language = stdout.contains("compar") || 
                                     stdout.contains("first") ||
                                     stdout.contains("second") ||
                                     stdout.contains("number");
        
        assert!(
            has_comparison_language,
            "Program should use comparison-related language. Output:\n{}",
            String::from_utf8_lossy(&output.stdout)
        );
    }
}

#[test]
fn test_source_code_uses_match() {
    let source_code = std::fs::read_to_string("src/main.rs")
        .expect("Should be able to read source code");
    
    assert!(
        source_code.contains("match"),
        "Source code should use a match expression. Current source:\n{}",
        source_code
    );
}

#[test]
fn test_source_code_imports_ordering() {
    let source_code = std::fs::read_to_string("src/main.rs")
        .expect("Should be able to read source code");
    
    assert!(
        source_code.contains("Ordering") && (source_code.contains("use") || source_code.contains("std::cmp")),
        "Source code should import or use the Ordering enum. Current source:\n{}",
        source_code
    );
}

#[test]
fn test_multiple_comparison_scenarios() {
    let test_cases = [
        ("1", "100", "smaller"),    // clearly smaller
        ("100", "1", "greater"),    // clearly greater  
        ("42", "42", "equal"),      // exactly equal
        ("0", "1", "smaller"),      // edge case with 0
        ("99", "100", "smaller"),   // close but smaller
    ];
    
    for (num1, num2, expected_type) in &test_cases {
        let mut child = Command::new("cargo")
            .args(&["run"])
            .current_dir(".")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .expect("Failed to start program");

        let stdin = child.stdin.as_mut().expect("Failed to open stdin");
        stdin.write_all(format!("{}\n{}\n", num1, num2).as_bytes()).expect("Failed to write to stdin");
        drop(stdin);

        let output = child.wait_with_output().expect("Failed to wait for program");
        
        if output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout).to_lowercase();
            
            let matches_expected = match *expected_type {
                "smaller" => stdout.contains("smaller") || stdout.contains("less"),
                "greater" => stdout.contains("greater") || stdout.contains("larger") || stdout.contains("bigger"),
                "equal" => stdout.contains("equal") || stdout.contains("same"),
                _ => false,
            };
            
            assert!(
                matches_expected,
                "For inputs {} and {}, expected '{}' comparison result. Output:\n{}",
                num1, num2, expected_type, String::from_utf8_lossy(&output.stdout)
            );
        }
    }
}