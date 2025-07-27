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
    
    // Check for tuple-related output
    assert!(
        stdout.contains("Name: Alice"),
        "Should demonstrate tuple destructuring with Alice's name"
    );
    
    assert!(
        stdout.contains("Age: 30"),
        "Should demonstrate tuple access with age"
    );
    
    // Check for array-related output
    assert!(
        stdout.contains("Array length:"),
        "Should demonstrate array length calculation"
    );
    
    assert!(
        stdout.contains("Zeros array:"),
        "Should demonstrate array initialization"
    );
    
    // Check for function output
    assert!(
        stdout.contains("Rectangle area:"),
        "Should demonstrate function returning value"
    );
    
    assert!(
        stdout.contains("Area:") && stdout.contains("Perimeter:"),
        "Should demonstrate function returning tuple"
    );
}

#[test]
fn test_demonstrates_tuple_destructuring() {
    let output = Command::new("cargo")
        .args(&["run", "--quiet"])
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output()
        .expect("Failed to execute program");

    let stdout = String::from_utf8_lossy(&output.stdout);
    
    // Should show tuple values are properly extracted
    assert!(
        stdout.contains("First: Alice") || stdout.contains("Name: Alice"),
        "Should demonstrate tuple element access"
    );
    
    assert!(
        stdout.contains("Number:") && stdout.contains("Pi:"),
        "Should demonstrate partial tuple destructuring"
    );
}

#[test]
fn test_demonstrates_array_operations() {
    let output = Command::new("cargo")
        .args(&["run", "--quiet"])
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output()
        .expect("Failed to execute program");

    let stdout = String::from_utf8_lossy(&output.stdout);
    
    // Should show array indexing and length
    assert!(
        stdout.contains("First:") && stdout.contains("Last:"),
        "Should demonstrate array indexing"
    );
    
    assert!(
        stdout.contains("Array length:"),
        "Should demonstrate array length property"
    );
    
    // Should show array iteration
    assert!(
        stdout.contains("Index") && stdout.contains("apple"),
        "Should demonstrate array iteration with enumerate"
    );
}

#[test]
fn test_average_calculation() {
    let output = Command::new("cargo")
        .args(&["run", "--quiet"])
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output()
        .expect("Failed to execute program");

    let stdout = String::from_utf8_lossy(&output.stdout);
    
    // Should calculate averages for students
    assert!(
        stdout.contains("average:"),
        "Should calculate and display student averages"
    );
}