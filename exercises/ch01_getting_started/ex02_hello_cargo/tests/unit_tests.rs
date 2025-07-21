use std::process::Command;

#[test]
fn test_cargo_toml_has_colored_dependency() {
    let cargo_content = std::fs::read_to_string("Cargo.toml")
        .expect("Failed to read Cargo.toml");

    assert!(
        cargo_content.contains("colored"),
        "Cargo.toml should include the 'colored' crate as a dependency"
    );

    assert!(
        cargo_content.contains("2") || cargo_content.contains("\"2\""),
        "The colored crate should be version 2"
    );
}

#[test]
fn test_project_compiles_with_dependencies() {
    let output = Command::new("cargo")
        .args(&["check"])
        .current_dir(".")
        .output()
        .expect("Failed to execute cargo check");

    assert!(
        output.status.success(),
        "Project should compile successfully with dependencies. Error:\n{}",
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

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        !stdout.trim().is_empty(),
        "Program should produce some output"
    );
}

#[test]
fn test_uses_colored_crate() {
    let main_content = std::fs::read_to_string("src/main.rs")
        .expect("Failed to read src/main.rs");

    assert!(
        main_content.contains("use colored") || main_content.contains("colored::"),
        "main.rs should use the colored crate (import it with 'use colored::*' or similar)"
    );

    // Check that TODO comments have been removed
    assert!(
        !main_content.contains("TODO"),
        "All TODO comments should be replaced with actual implementation"
    );
}

#[test]
fn test_proper_rust_project_structure() {
    // Check that we have the expected files
    assert!(
        std::path::Path::new("Cargo.toml").exists(),
        "Cargo.toml should exist in the project root"
    );

    assert!(
        std::path::Path::new("src/main.rs").exists(),
        "src/main.rs should exist"
    );
}

#[test]
fn test_cargo_build_works() {
    let output = Command::new("cargo")
        .args(&["build"])
        .current_dir(".")
        .output()
        .expect("Failed to execute cargo build");

    assert!(
        output.status.success(),
        "cargo build should succeed. Error:\n{}",
        String::from_utf8_lossy(&output.stderr)
    );
}

#[test]
fn test_demonstrates_cargo_concepts() {
    let main_content = std::fs::read_to_string("src/main.rs")
        .expect("Failed to read src/main.rs");

    // The exercise should demonstrate understanding of Cargo
    let has_cargo_mention = main_content.to_lowercase().contains("cargo") ||
                           main_content.to_lowercase().contains("build") ||
                           main_content.to_lowercase().contains("dependency") ||
                           main_content.to_lowercase().contains("crate");

    assert!(
        has_cargo_mention,
        "The program should mention or demonstrate Cargo concepts"
    );
}