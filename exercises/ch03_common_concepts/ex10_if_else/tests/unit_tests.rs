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
fn test_temperature_condition() {
    let output = Command::new("cargo")
        .args(&["run", "--quiet"])
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output()
        .expect("Failed to execute program");

    let stdout = String::from_utf8_lossy(&output.stdout);
    
    // Temperature 75 should be warm
    assert!(
        stdout.contains("It's warm outside!"),
        "Should correctly identify warm temperature (75°F)"
    );
}

#[test]
fn test_voting_eligibility() {
    let output = Command::new("cargo")
        .args(&["run", "--quiet"])
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output()
        .expect("Failed to execute program");

    let stdout = String::from_utf8_lossy(&output.stdout);
    
    // Age 25 should be eligible to vote
    assert!(
        stdout.contains("You can vote!"),
        "Should allow voting for age 25"
    );
}

#[test]
fn test_grade_assignment() {
    let output = Command::new("cargo")
        .args(&["run", "--quiet"])
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output()
        .expect("Failed to execute program");

    let stdout = String::from_utf8_lossy(&output.stdout);
    
    // Score 85 should get grade B
    assert!(
        stdout.contains("Grade: B"),
        "Score 85 should result in grade B (80-89 range)"
    );
}

#[test]
fn test_conditional_assignment() {
    let output = Command::new("cargo")
        .args(&["run", "--quiet"])
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output()
        .expect("Failed to execute program");

    let stdout = String::from_utf8_lossy(&output.stdout);
    
    // Should show activity assignment based on weather
    assert!(
        stdout.contains("Today's activity:"),
        "Should demonstrate conditional assignment for activity"
    );
}

#[test]
fn test_complex_boolean_logic() {
    let output = Command::new("cargo")
        .args(&["run", "--quiet"])
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output()
        .expect("Failed to execute program");

    let stdout = String::from_utf8_lossy(&output.stdout);
    
    // With license=true, car=true, gas=false, should not be able to drive
    assert!(
        stdout.contains("You cannot drive."),
        "Should correctly evaluate complex driving conditions"
    );
    
    // Should provide specific reason for not being able to drive
    assert!(
        stdout.contains("Reason: No gas"),
        "Should identify specific reason for not driving (no gas)"
    );
}

#[test]
fn test_time_based_recommendations() {
    let output = Command::new("cargo")
        .args(&["run", "--quiet"])
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output()
        .expect("Failed to execute program");

    let stdout = String::from_utf8_lossy(&output.stdout);
    
    // Time 14 (2 PM) should be lunch time
    assert!(
        stdout.contains("Lunch time!"),
        "Time 14 (2 PM) should be identified as lunch time"
    );
}