use std::process::Command;

#[test]
fn test_program_compiles_and_runs() {
    let output = Command::new("cargo")
        .args(&["run", "--quiet"])
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output()
        .expect("Failed to execute program");

    assert!(
        output.status.success(),
        "Program should compile and run successfully"
    );
}

#[test]
fn test_program_output_contains_expected_content() {
    let output = Command::new("cargo")
        .args(&["run", "--quiet"])
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output()
        .expect("Failed to execute program");

    let stdout = String::from_utf8_lossy(&output.stdout);
    
    // Check for main exercise output
    assert!(
        stdout.contains("Comments Exercise"),
        "Should display comments exercise header"
    );
    
    assert!(
        stdout.contains("Temperature:"),
        "Should display temperature information"
    );
    
    assert!(
        stdout.contains("°F = ") && stdout.contains("°C"),
        "Should demonstrate temperature conversion"
    );
    
    assert!(
        stdout.contains("Addition result:"),
        "Should call and display add_numbers function result"
    );
    
    assert!(
        stdout.contains("Rectangle area:"),
        "Should call and display calculate_area function result"
    );
    
    assert!(
        stdout.contains("Comment Styles Demo"),
        "Should demonstrate different comment styles"
    );
}

#[test]
fn test_functions_work_correctly() {
    let output = Command::new("cargo")
        .args(&["run", "--quiet"])
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output()
        .expect("Failed to execute program");

    let stdout = String::from_utf8_lossy(&output.stdout);
    
    // Test that arithmetic functions produce correct results
    assert!(
        stdout.contains("Addition result: 8"),
        "add_numbers(5, 3) should equal 8"
    );
    
    // Check temperature conversion (100°F = 37.8°C approximately)
    assert!(
        stdout.contains("37.8°C") || stdout.contains("37.7°C") || stdout.contains("37.9°C"),
        "Temperature conversion should be approximately correct"
    );
}

#[test]
fn test_demonstrates_statistics_calculation() {
    let output = Command::new("cargo")
        .args(&["run", "--quiet"])
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output()
        .expect("Failed to execute program");

    let stdout = String::from_utf8_lossy(&output.stdout);
    
    // Should show array processing
    assert!(
        stdout.contains("Numbers: [1, 2, 3, 4, 5]"),
        "Should display the numbers array"
    );
    
    assert!(
        stdout.contains("Sum: 15"),
        "Should calculate correct sum of 1+2+3+4+5=15"
    );
    
    assert!(
        stdout.contains("Average: 3.00"),
        "Should calculate correct average of 15/5=3.00"
    );
}

#[test]
fn test_no_commented_out_lines_print() {
    let output = Command::new("cargo")
        .args(&["run", "--quiet"])
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output()
        .expect("Failed to execute program");

    let stdout = String::from_utf8_lossy(&output.stdout);
    
    // These lines should be commented out and not appear in output
    assert!(
        !stdout.contains("This line should be commented out!"),
        "The line with 'This line should be commented out!' should be commented"
    );
    
    assert!(
        !stdout.contains("This should not print: 42"),
        "The block comment section should be commented out"
    );
    
    assert!(
        !stdout.contains("This should also not print: test"),
        "The block comment section should be commented out"
    );
}