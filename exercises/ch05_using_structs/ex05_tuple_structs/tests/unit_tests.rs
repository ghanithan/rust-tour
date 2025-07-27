use std::process::Command;

/// Test that the program compiles and runs without errors
#[test]
fn test_program_compiles_and_runs() {
    let output = Command::new("cargo")
        .args(&["run", "--quiet"])
        .current_dir(".")
        .output()
        .expect("Failed to execute cargo run");

    assert!(
        output.status.success(),
        "Program should compile and run successfully. Stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
}

/// Test that the program produces expected output for tuple struct creation
#[test]
fn test_color_output() {
    let output = Command::new("cargo")
        .args(&["run", "--quiet"])
        .current_dir(".")
        .output()
        .expect("Failed to execute cargo run");

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("Red color: RGB(255, 0, 0)"),
        "Expected red color output with RGB(255, 0, 0). Got: {}",
        stdout
    );
}

/// Test that the program outputs point creation correctly
#[test]
fn test_point_output() {
    let output = Command::new("cargo")
        .args(&["run", "--quiet"])
        .current_dir(".")
        .output()
        .expect("Failed to execute cargo run");

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("Origin point: (0, 0)"),
        "Expected origin point output with (0, 0). Got: {}",
        stdout
    );
}

/// Test that unit-like struct creation works
#[test]
fn test_unit_struct_output() {
    let output = Command::new("cargo")
        .args(&["run", "--quiet"])
        .current_dir(".")
        .output()
        .expect("Failed to execute cargo run");

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("Unit-like struct created: AlwaysEqual"),
        "Expected unit-like struct debug output. Got: {}",
        stdout
    );
}

/// Test that UserID newtype pattern works
#[test]
fn test_user_id_output() {
    let output = Command::new("cargo")
        .args(&["run", "--quiet"])
        .current_dir(".")
        .output()
        .expect("Failed to execute cargo run");

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("User 1 ID: 42") && stdout.contains("User 2 ID: 100"),
        "Expected user ID outputs with values 42 and 100. Got: {}",
        stdout
    );
}

/// Test that type safety demonstration works (points comparison)
#[test]
fn test_type_safety_comparison() {
    let output = Command::new("cargo")
        .args(&["run", "--quiet"])
        .current_dir(".")
        .output()
        .expect("Failed to execute cargo run");

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("Points are equal"),
        "Expected points comparison to work and show 'Points are equal'. Got: {}",
        stdout
    );
}

/// Test that color brightness method works correctly
#[test]
fn test_color_brightness_output() {
    let output = Command::new("cargo")
        .args(&["run", "--quiet"])
        .current_dir(".")
        .output()
        .expect("Failed to execute cargo run");

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("Red color brightness: 255"),
        "Expected red color brightness of 255. Got: {}",
        stdout
    );
}

/// Test that point distance calculation works correctly
#[test]
fn test_point_distance_output() {
    let output = Command::new("cargo")
        .args(&["run", "--quiet"])
        .current_dir(".")
        .output()
        .expect("Failed to execute cargo run");

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("Distance from origin to point: 2.23606797749979") 
        || stdout.contains("Distance from origin to point: 2.236067977499"),
        "Expected point distance calculation for Point(1.0, 2.0). Got: {}",
        stdout
    );
}

/// Test that user ID validation works correctly
#[test]
fn test_user_id_validation_output() {
    let output = Command::new("cargo")
        .args(&["run", "--quiet"])
        .current_dir(".")
        .output()
        .expect("Failed to execute cargo run");

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("User 1 is valid: true"),
        "Expected user ID validation to show 'User 1 is valid: true'. Got: {}",
        stdout
    );
}

/// Test that all embedded unit tests pass
#[test]
fn test_embedded_unit_tests_pass() {
    let output = Command::new("cargo")
        .args(&["test", "--", "--nocapture"])
        .current_dir(".")
        .output()
        .expect("Failed to execute cargo test");

    assert!(
        output.status.success(),
        "All embedded unit tests should pass. Stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    
    // Check that specific tests are mentioned and passed
    let expected_tests = [
        "test_color_creation",
        "test_point_creation", 
        "test_user_id_newtype",
        "test_color_brightness",
        "test_point_distance",
        "test_user_id_validation"
    ];

    for test_name in &expected_tests {
        assert!(
            stdout.contains(&format!("test tests::{} ... ok", test_name)),
            "Expected test '{}' to pass. Got: {}",
            test_name,
            stdout
        );
    }
}

/// Test that the program demonstrates type safety concepts
#[test]
fn test_type_safety_demonstration() {
    let output = Command::new("cargo")
        .args(&["run", "--quiet"])
        .current_dir(".")
        .output()
        .expect("Failed to execute cargo run");

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("Testing type safety..."),
        "Expected type safety demonstration section. Got: {}",
        stdout
    );
}

/// Test that method testing section is present
#[test]
fn test_methods_section() {
    let output = Command::new("cargo")
        .args(&["run", "--quiet"])
        .current_dir(".")
        .output()
        .expect("Failed to execute cargo run");

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("Testing methods on tuple structs..."),
        "Expected methods testing section. Got: {}",
        stdout
    );
}

/// Performance test: ensure compilation is reasonably fast
#[test]
fn test_compilation_performance() {
    use std::time::Instant;
    
    let start = Instant::now();
    let output = Command::new("cargo")
        .args(&["check"])
        .current_dir(".")
        .output()
        .expect("Failed to execute cargo check");
    let duration = start.elapsed();

    assert!(
        output.status.success(),
        "Code should compile successfully"
    );
    
    assert!(
        duration.as_secs() < 30,
        "Compilation should complete within 30 seconds, took: {:?}",
        duration
    );
}

/// Test that clippy passes without warnings on the solution
#[test] 
fn test_clippy_compliance() {
    let output = Command::new("cargo")
        .args(&["clippy", "--", "-D", "warnings"])
        .current_dir(".")
        .output()
        .expect("Failed to execute cargo clippy");

    if !output.status.success() {
        // Only fail if there are actual clippy warnings, not if clippy itself fails
        let stderr = String::from_utf8_lossy(&output.stderr);
        if stderr.contains("warning:") {
            panic!("Clippy warnings found: {}", stderr);
        }
    }
}