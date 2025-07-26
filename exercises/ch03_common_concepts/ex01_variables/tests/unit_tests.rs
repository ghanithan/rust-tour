use std::process::Command;

#[test]
fn test_code_compiles_successfully() {
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
fn test_program_runs_without_panics() {
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
fn test_exercise_demonstrates_all_variable_concepts() {
    let output = Command::new("cargo")
        .args(&["run"])
        .current_dir(".")
        .output()
        .expect("Failed to execute cargo run");

    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        
        // 1. Test variable shadowing (currently implemented)
        assert!(
            stdout.contains("The value of x is: 5") && stdout.contains("The value of x is: 6"),
            "❌ Variable shadowing: Program should print x=5 and then x=6"
        );
        
        // 2. Test type conversion through shadowing (currently implemented)
        assert!(
            stdout.contains("spaces") && stdout.contains("3"),
            "❌ Type conversion: Program should show spaces string and its length (3)"
        );
        
        // 3. Test mutable variables 
        assert!(
            stdout.contains("count") && stdout.contains("0") && stdout.contains("1"),
            "❌ Mutable variables: Program should demonstrate count incrementing from 0 to 1"
        );
    }
}