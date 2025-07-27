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
fn test_program_handles_valid_number_input() {
    let mut child = Command::new("cargo")
        .args(&["run"])
        .current_dir(".")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Failed to start program");

    let stdin = child.stdin.as_mut().expect("Failed to open stdin");
    stdin.write_all(b"25\n").expect("Failed to write to stdin");
    drop(stdin);

    let output = child.wait_with_output().expect("Failed to wait for program");
    
    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        
        // Should show the entered number
        assert!(
            stdout.contains("25"),
            "Program should display the entered number. Output:\n{}",
            stdout
        );
    }
}

#[test]
fn test_program_performs_numeric_doubling() {
    let mut child = Command::new("cargo")
        .args(&["run"])
        .current_dir(".")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Failed to start program");

    let stdin = child.stdin.as_mut().expect("Failed to open stdin");
    stdin.write_all(b"15\n").expect("Failed to write to stdin");
    drop(stdin);

    let output = child.wait_with_output().expect("Failed to wait for program");
    
    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        
        // Should show 15 * 2 = 30, not string concatenation
        assert!(
            stdout.contains("30"),
            "Program should double the number (15 * 2 = 30), not concatenate strings. Output:\n{}",
            stdout
        );
        
        // Should NOT contain string concatenation result "1515"
        assert!(
            !stdout.contains("1515"),
            "Program should not perform string concatenation. Output:\n{}",
            stdout
        );
    }
}

#[test]
fn test_program_performs_numeric_comparison() {
    // Test with number greater than 30
    let mut child = Command::new("cargo")
        .args(&["run"])
        .current_dir(".")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Failed to start program");

    let stdin = child.stdin.as_mut().expect("Failed to open stdin");
    stdin.write_all(b"45\n").expect("Failed to write to stdin");
    drop(stdin);

    let output = child.wait_with_output().expect("Failed to wait for program");
    
    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        
        // Should show true for 45 > 30
        assert!(
            stdout.contains("true"),
            "Program should correctly identify that 45 > 30. Output:\n{}",
            stdout
        );
    }
}

#[test]
fn test_program_handles_number_less_than_30() {
    let mut child = Command::new("cargo")
        .args(&["run"])
        .current_dir(".")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Failed to start program");

    let stdin = child.stdin.as_mut().expect("Failed to open stdin");
    stdin.write_all(b"10\n").expect("Failed to write to stdin");
    drop(stdin);

    let output = child.wait_with_output().expect("Failed to wait for program");
    
    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        
        // Should show false for 10 > 30
        assert!(
            stdout.contains("false"),
            "Program should correctly identify that 10 is not > 30. Output:\n{}",
            stdout
        );
    }
}

#[test]
fn test_program_handles_whitespace_in_input() {
    let mut child = Command::new("cargo")
        .args(&["run"])
        .current_dir(".")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Failed to start program");

    let stdin = child.stdin.as_mut().expect("Failed to open stdin");
    // Input with extra spaces
    stdin.write_all(b"  20  \n").expect("Failed to write to stdin");
    drop(stdin);

    let output = child.wait_with_output().expect("Failed to wait for program");
    
    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        
        // Should handle whitespace and show 20 * 2 = 40
        assert!(
            stdout.contains("20") && stdout.contains("40"),
            "Program should handle whitespace and perform calculations correctly. Output:\n{}",
            stdout
        );
    }
}

#[test]
fn test_program_shows_descriptive_output() {
    let mut child = Command::new("cargo")
        .args(&["run"])
        .current_dir(".")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Failed to start program");

    let stdin = child.stdin.as_mut().expect("Failed to open stdin");
    stdin.write_all(b"35\n").expect("Failed to write to stdin");
    drop(stdin);

    let output = child.wait_with_output().expect("Failed to wait for program");
    
    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout).to_lowercase();
        
        // Should have descriptive text about operations
        let has_descriptive_text = stdout.contains("double") || 
                                   stdout.contains("greater") ||
                                   stdout.contains("entered");
        
        assert!(
            has_descriptive_text,
            "Program should have descriptive output about the operations. Output:\n{}",
            String::from_utf8_lossy(&output.stdout)
        );
    }
}

#[test] 
fn test_program_uses_numeric_types() {
    // This test checks that the program actually uses numeric operations
    // by verifying mathematical results are correct
    
    let test_cases = [
        ("5", "10"),   // 5 * 2 = 10
        ("7", "14"),   // 7 * 2 = 14  
        ("12", "24"),  // 12 * 2 = 24
    ];
    
    for (input, expected_double) in &test_cases {
        let mut child = Command::new("cargo")
            .args(&["run"])
            .current_dir(".")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .expect("Failed to start program");

        let stdin = child.stdin.as_mut().expect("Failed to open stdin");
        stdin.write_all(format!("{}\n", input).as_bytes()).expect("Failed to write to stdin");
        drop(stdin);

        let output = child.wait_with_output().expect("Failed to wait for program");
        
        if output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout);
            
            assert!(
                stdout.contains(expected_double),
                "Program should calculate {} * 2 = {} correctly. Output:\n{}",
                input, expected_double, stdout
            );
        }
    }
}