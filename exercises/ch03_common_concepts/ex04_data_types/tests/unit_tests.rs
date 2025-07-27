use std::process::Command;

#[test]
fn test_code_compiles() {
    let output = Command::new("cargo")
        .args(&["check"])
        .current_dir(".")
        .output()
        .expect("Failed to execute cargo check");

    assert!(
        output.status.success(),
        "Code should compile without errors. Compiler output:\n{}\nError output:\n{}",
        String::from_utf8_lossy(&output.stdout),
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
        "Program should run without panicking. Error output:\n{}",
        String::from_utf8_lossy(&output.stderr)
    );
}

#[test]
fn test_integer_handling() {
    let output = Command::new("cargo")
        .args(&["run"])
        .current_dir(".")
        .output()
        .expect("Failed to execute cargo run");

    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        
        // Should handle large number (9,223,372,036,854,775,807)
        assert!(
            stdout.contains("Big number: 9223372036854775807"),
            "Should display the large number correctly. Output:\n{}",
            stdout
        );
        
        // Should handle negative population
        assert!(
            stdout.contains("Population: -50000"),
            "Should handle negative population. Output:\n{}",
            stdout
        );
        
        // Should handle age 300
        assert!(
            stdout.contains("Age: 300"),
            "Should handle age 300. Output:\n{}",
            stdout
        );
    }
}

#[test]
fn test_float_handling() {
    let output = Command::new("cargo")
        .args(&["run"])
        .current_dir(".")
        .output()
        .expect("Failed to execute cargo run");

    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        
        // Should show precise pi
        assert!(
            stdout.contains("Precise π (f64): 3.141592653589793"),
            "Should display precise pi value. Output:\n{}",
            stdout
        );
    }
}

#[test]
fn test_arithmetic_operations() {
    let output = Command::new("cargo")
        .args(&["run"])
        .current_dir(".")
        .output()
        .expect("Failed to execute cargo run");

    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        
        // Should show result of mixed arithmetic (42 + 3.14 = 45.14)
        assert!(
            stdout.contains("Result: 45.14"),
            "Should show result of integer + float arithmetic. Output:\n{}",
            stdout
        );
    }
}

#[test]
fn test_string_parsing() {
    let output = Command::new("cargo")
        .args(&["run"])
        .current_dir(".")
        .output()
        .expect("Failed to execute cargo run");

    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        
        // Should parse "42" to integer 42
        assert!(
            stdout.contains("Parsed number: 42"),
            "Should parse string '42' to integer. Output:\n{}",
            stdout
        );
        
        // Should parse "3.14159" to float
        assert!(
            stdout.contains("Parsed float: 3.14159"),
            "Should parse string to float. Output:\n{}",
            stdout
        );
    }
}

#[test]
fn test_overflow_handling() {
    let output = Command::new("cargo")
        .args(&["run"])
        .current_dir(".")
        .output()
        .expect("Failed to execute cargo run");

    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        
        // Should handle overflow safely (not panic)
        assert!(
            stdout.contains("Overflow result:"),
            "Should handle overflow operation safely. Output:\n{}",
            stdout
        );
    }
}