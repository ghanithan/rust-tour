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
fn test_program_output() {
    let output = Command::new("cargo")
        .args(&["run"])
        .current_dir(".")
        .output()
        .expect("Failed to execute cargo run");

    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        
        // Check that temperature changes from 32 to 212
        assert!(
            stdout.contains("Initial temperature: 32°F") && stdout.contains("Updated temperature: 212°F"),
            "Part 1: Should show temperature changing from 32°F to 212°F.\nActual output:\n{}",
            stdout
        );
        
        // Check that score becomes 150 (100 + 50)
        assert!(
            stdout.contains("Final score: 150"),
            "Part 2: Should show final score as 150 (100 + 50).\nActual output:\n{}",
            stdout
        );
        
        // Check that counter becomes 1 (0 + 1)
        assert!(
            stdout.contains("Counter after increment: 1"),
            "Part 3: Should show counter as 1 after increment.\nActual output:\n{}",
            stdout
        );
    } else {
        panic!(
            "Program should run successfully. Error output:\n{}",
            String::from_utf8_lossy(&output.stderr)
        );
    }
}

#[test]
fn test_mutability_concepts_demonstrated() {
    // This test ensures the code actually demonstrates the concepts
    let source = std::fs::read_to_string("src/main.rs").unwrap();
    
    // Should use mutable variables
    assert!(
        source.contains("let mut"),
        "Code should demonstrate mutable variables with 'let mut'"
    );
    
    // Should use mutable references  
    assert!(
        source.contains("&mut"),
        "Code should demonstrate mutable references with '&mut'"
    );
    
    // Should dereference mutable references
    assert!(
        source.contains("*") && source.contains("+="),
        "Code should demonstrate dereferencing mutable references"
    );
}