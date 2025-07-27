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
fn test_infinite_loop_with_break() {
    let output = Command::new("cargo")
        .args(&["run", "--quiet"])
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output()
        .expect("Failed to execute program");

    let stdout = String::from_utf8_lossy(&output.stdout);
    
    // Should show counter incrementing
    assert!(
        stdout.contains("Counter: 0"),
        "Should show initial counter value"
    );
    
    assert!(
        stdout.contains("Counter: 1"),
        "Should show counter incrementing"
    );
    
    assert!(
        stdout.contains("Counter: 2"),
        "Should show counter continuing"
    );
    
    // Should not go beyond 3
    assert!(
        !stdout.contains("Counter: 4"),
        "Should break at counter 3"
    );
}

#[test]
fn test_loop_with_return_value() {
    let output = Command::new("cargo")
        .args(&["run", "--quiet"])
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output()
        .expect("Failed to execute program");

    let stdout = String::from_utf8_lossy(&output.stdout);
    
    // Loop should return 5 * 2 = 10
    assert!(
        stdout.contains("Loop result: 10"),
        "Loop should return value 10 (5 * 2)"
    );
}

#[test]
fn test_while_loop_countdown() {
    let output = Command::new("cargo")
        .args(&["run", "--quiet"])
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output()
        .expect("Failed to execute program");

    let stdout = String::from_utf8_lossy(&output.stdout);
    
    // Should show countdown
    assert!(
        stdout.contains("Countdown: 10"),
        "Should start countdown from 10"
    );
    
    assert!(
        stdout.contains("Countdown: 1"),
        "Should countdown to 1"
    );
    
    assert!(
        stdout.contains("Liftoff!"),
        "Should end with liftoff message"
    );
}

#[test]
fn test_for_loop_with_range() {
    let output = Command::new("cargo")
        .args(&["run", "--quiet"])
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output()
        .expect("Failed to execute program");

    let stdout = String::from_utf8_lossy(&output.stdout);
    
    // Should iterate through range 1-5
    assert!(
        stdout.contains("Number: 1") && 
        stdout.contains("Number: 2") && 
        stdout.contains("Number: 5"),
        "Should iterate through numbers 1 through 5"
    );
}

#[test]
fn test_array_iteration() {
    let output = Command::new("cargo")
        .args(&["run", "--quiet"])
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output()
        .expect("Failed to execute program");

    let stdout = String::from_utf8_lossy(&output.stdout);
    
    // Should iterate over fruits array
    assert!(
        stdout.contains("Fruit: apple") &&
        stdout.contains("Fruit: banana") &&
        stdout.contains("Fruit: cherry"),
        "Should iterate over all fruits in array"
    );
}

#[test]
fn test_nested_loops_multiplication() {
    let output = Command::new("cargo")
        .args(&["run", "--quiet"])
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output()
        .expect("Failed to execute program");

    let stdout = String::from_utf8_lossy(&output.stdout);
    
    // Should show multiplication table results
    assert!(
        stdout.contains("1 2 3") || stdout.contains("1  2  3"),
        "Should show first row of multiplication table (1*1, 1*2, 1*3)"
    );
    
    assert!(
        stdout.contains("4 ") || stdout.contains("4  "),
        "Should show 2*2=4 in multiplication table"
    );
    
    assert!(
        stdout.contains("9 ") || stdout.contains("9  "),
        "Should show 3*3=9 in multiplication table"
    );
}

#[test]
fn test_continue_with_odd_numbers() {
    let output = Command::new("cargo")
        .args(&["run", "--quiet"])
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output()
        .expect("Failed to execute program");

    let stdout = String::from_utf8_lossy(&output.stdout);
    
    // Should show only odd numbers
    assert!(
        stdout.contains("Odd number: 1") &&
        stdout.contains("Odd number: 3") &&
        stdout.contains("Odd number: 5"),
        "Should display odd numbers 1, 3, 5"
    );
    
    // Should not show even numbers
    assert!(
        !stdout.contains("Odd number: 2") &&
        !stdout.contains("Odd number: 4"),
        "Should skip even numbers with continue"
    );
}

#[test]
fn test_while_let_pattern() {
    let output = Command::new("cargo")
        .args(&["run", "--quiet"])
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output()
        .expect("Failed to execute program");

    let stdout = String::from_utf8_lossy(&output.stdout);
    
    // Should pop all values from stack
    assert!(
        stdout.contains("Popped: 5") &&
        stdout.contains("Popped: 1"),
        "Should pop all values from the stack"
    );
}