use std::process::Command;

#[test]
fn test_program_compiles_and_runs() {
    let output = Command::new("cargo")
        .args(&["run", "--quiet"])
        .current_dir(".")
        .output()
        .expect("Failed to execute cargo run");

    assert!(
        output.status.success(),
        "Program should compile and run successfully. Make sure all methods are implemented. Stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
}

#[test]
fn test_area_method_output() {
    let output = Command::new("cargo")
        .args(&["run", "--quiet"])
        .current_dir(".")
        .output()
        .expect("Failed to execute cargo run");

    let stdout = String::from_utf8_lossy(&output.stdout);
    
    // Test that area calculations work
    assert!(stdout.contains("area: 1500"), "Should calculate rect1 area as 1500");
    assert!(stdout.contains("area: 400"), "Should calculate rect2 area as 400");
}

#[test]
fn test_comparison_methods_output() {
    let output = Command::new("cargo")
        .args(&["run", "--quiet"])
        .current_dir(".")
        .output()
        .expect("Failed to execute cargo run");

    let stdout = String::from_utf8_lossy(&output.stdout);
    
    // Test comparison method outputs
    assert!(
        stdout.contains("rect1 can hold rect2: true"),
        "rect1 should be able to hold rect2"
    );
    assert!(
        stdout.contains("rect2 can hold rect1: false"),
        "rect2 should not be able to hold rect1"
    );
    assert!(
        stdout.contains("rect1 has same area as rect3: true"),
        "rect1 and rect3 should have same area"
    );
}

#[test]
fn test_scale_method_output() {
    let output = Command::new("cargo")
        .args(&["run", "--quiet"])
        .current_dir(".")
        .output()
        .expect("Failed to execute cargo run");

    let stdout = String::from_utf8_lossy(&output.stdout);
    
    // Test scaling functionality
    assert!(
        stdout.contains("Before scaling: Rectangle: 30x50"),
        "Should show original dimensions before scaling"
    );
    assert!(
        stdout.contains("After scaling by 2: Rectangle: 60x100"),
        "Should show doubled dimensions after scaling"
    );
    assert!(
        stdout.contains("area: 6000"),
        "Should show correct area after scaling"
    );
}

#[test]
fn test_to_square_method_output() {
    let output = Command::new("cargo")
        .args(&["run", "--quiet"])
        .current_dir(".")
        .output()
        .expect("Failed to execute cargo run");

    let stdout = String::from_utf8_lossy(&output.stdout);
    
    // Test square creation
    assert!(
        stdout.contains("square: 25x25"),
        "Should create a square with minimum dimension (10) from rect2"
    );
}

#[test]
fn test_individual_unit_tests() {
    // Test that the area calculation unit test passes
    let output = Command::new("cargo")
        .args(&["test", "--quiet", "test_area_calculation"])
        .current_dir(".")
        .output()
        .expect("Failed to execute cargo test");

    assert!(
        output.status.success(),
        "Area calculation test should pass. Stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    // Test that the can_hold unit test passes
    let output = Command::new("cargo")
        .args(&["test", "--quiet", "test_can_hold"])
        .current_dir(".")
        .output()
        .expect("Failed to execute cargo test");

    assert!(
        output.status.success(),
        "can_hold test should pass. Stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    // Test that the scale unit test passes
    let output = Command::new("cargo")
        .args(&["test", "--quiet", "test_scale"])
        .current_dir(".")
        .output()
        .expect("Failed to execute cargo test");

    assert!(
        output.status.success(),
        "scale test should pass. Stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
}