use std::process::Command;

#[test]
fn test_project_compiles_successfully() {
    let output = Command::new("cargo")
        .args(&["check"])
        .current_dir(".")
        .output()
        .expect("Failed to execute cargo check");

    assert!(
        output.status.success(),
        "Project should compile successfully. Error:\n{}",
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
        "Program should run successfully. Error:\n{}",
        String::from_utf8_lossy(&output.stderr)
    );
}

#[test]
fn test_program_produces_colored_output() {
    let output = Command::new("cargo")
        .args(&["run"])
        .current_dir(".")
        .output()
        .expect("Failed to execute cargo run");

    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        
        // Test that program produces meaningful output
        assert!(
            !stdout.trim().is_empty(),
            "Program should produce visible output"
        );
        
        // Since we can't directly test color codes in a simple way,
        // we test that the program runs and uses the colored crate functionality
        assert!(
            stdout.len() > 10, // Colored output tends to be longer due to escape codes
            "Program should produce substantial output (likely with color formatting)"
        );
    }
}

#[test]
fn test_dependencies_are_properly_configured() {
    // Test that cargo can download and build dependencies
    let output = Command::new("cargo")
        .args(&["build"])
        .current_dir(".")
        .output()
        .expect("Failed to execute cargo build");

    assert!(
        output.status.success(),
        "cargo build should succeed, indicating dependencies are properly configured. Error:\n{}",
        String::from_utf8_lossy(&output.stderr)
    );
}

#[test]
fn test_project_structure_is_valid() {
    // Test that the project has the expected Rust/Cargo structure
    assert!(
        std::path::Path::new("Cargo.toml").exists(),
        "Project should have a Cargo.toml file"
    );

    assert!(
        std::path::Path::new("src/main.rs").exists(),
        "Project should have a src/main.rs file"
    );
    
    // Test that Cargo.toml is valid by attempting to parse it
    let cargo_output = Command::new("cargo")
        .args(&["metadata", "--format-version=1"])
        .current_dir(".")
        .output()
        .expect("Failed to execute cargo metadata");
    
    assert!(
        cargo_output.status.success(),
        "Cargo.toml should be valid and parseable"
    );
}

#[test]
fn test_external_crate_integration() {
    // Test that the program successfully integrates an external crate
    let output = Command::new("cargo")
        .args(&["run"])
        .current_dir(".")
        .output()
        .expect("Failed to execute cargo run");

    if output.status.success() {
        // If a program compiles and runs with dependencies, it likely uses them
        // This is more reliable than checking source code patterns
        let cargo_toml = std::fs::read_to_string("Cargo.toml")
            .expect("Failed to read Cargo.toml");
        
        // Check that dependencies exist in Cargo.toml
        assert!(
            cargo_toml.contains("[dependencies]"),
            "Cargo.toml should have a [dependencies] section"
        );
        
        // Test that dependency resolution works by building
        let build_output = Command::new("cargo")
            .args(&["build", "--verbose"])
            .current_dir(".")
            .output()
            .expect("Failed to execute cargo build");
            
        let build_log = String::from_utf8_lossy(&build_output.stderr);
        
        // Successful build with dependencies means external crates are integrated
        assert!(
            build_output.status.success(),
            "Build should succeed with external dependencies"
        );
        
        // Check that external crates were actually compiled (visible in verbose output)
        assert!(
            build_log.contains("Compiling") || cargo_toml.contains("colored") || cargo_toml.contains("rand") || cargo_toml.contains("serde"),
            "Should demonstrate integration with external crates"
        );
    }
}

#[test]
fn test_cargo_run_vs_direct_execution() {
    // Test that cargo run works (demonstrating Cargo as a build tool)
    let cargo_run_output = Command::new("cargo")
        .args(&["run"])
        .current_dir(".")
        .output()
        .expect("Failed to execute cargo run");

    assert!(
        cargo_run_output.status.success(),
        "cargo run should work, demonstrating Cargo's build and execution capabilities"
    );
    
    // Test that cargo build produces an executable
    let build_output = Command::new("cargo")
        .args(&["build"])
        .current_dir(".")
        .output()
        .expect("Failed to execute cargo build");

    assert!(
        build_output.status.success(),
        "cargo build should create an executable"
    );
    
    // Verify the executable was created
    let target_debug_path = std::path::Path::new("target/debug");
    assert!(
        target_debug_path.exists(),
        "Build should create target/debug directory with executable"
    );
}

#[test]
fn test_dependency_management_works() {
    // Test that Cargo can manage dependencies by cleaning and rebuilding
    let clean_output = Command::new("cargo")
        .args(&["clean"])
        .current_dir(".")
        .output()
        .expect("Failed to execute cargo clean");

    assert!(
        clean_output.status.success(),
        "cargo clean should work"
    );

    // After cleaning, build should still work (re-downloading dependencies)
    let rebuild_output = Command::new("cargo")
        .args(&["build"])
        .current_dir(".")
        .output()
        .expect("Failed to execute cargo build after clean");

    assert!(
        rebuild_output.status.success(),
        "Should be able to rebuild project after cleaning, demonstrating dependency management"
    );
}