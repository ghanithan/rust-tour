use std::process::{Command, Stdio};
use std::io::Write;
use std::fs;

#[test]
fn test_program_compiles() {
    let output = Command::new("cargo")
        .args(&["check"])
        .current_dir(".")
        .output()
        .expect("Failed to execute cargo check");

    assert!(
        output.status.success(),
        "Program should compile successfully. Compiler output:\n{}",
        String::from_utf8_lossy(&output.stderr)
    );
}

#[test]
fn test_cargo_toml_has_rand_dependency() {
    let cargo_content = std::fs::read_to_string("Cargo.toml")
        .expect("Should be able to read Cargo.toml");
    
    assert!(
        cargo_content.contains("rand"),
        "Cargo.toml should contain rand as a dependency"
    );
}

#[test]
fn test_program_runs_without_crashing() {
    let mut child = Command::new("cargo")
        .args(&["run"])
        .current_dir(".")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Failed to start program");

    let stdin = child.stdin.as_mut().expect("Failed to open stdin");
    // Quick test run
    stdin.write_all(b"50\nquit\n").expect("Failed to write to stdin");
    drop(stdin);

    let output = child.wait_with_output().expect("Failed to wait for program");
    
    assert!(
        output.status.success(),
        "Enhanced program should run without crashing. Error output:\n{}",
        String::from_utf8_lossy(&output.stderr)
    );
}

#[test]
fn test_program_shows_enhanced_welcome() {
    let mut child = Command::new("cargo")
        .args(&["run"])
        .current_dir(".")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Failed to start program");

    let stdin = child.stdin.as_mut().expect("Failed to open stdin");
    stdin.write_all(b"1\n50\nn\n").expect("Failed to write to stdin");
    drop(stdin);

    let output = child.wait_with_output().expect("Failed to wait for program");
    
    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout).to_lowercase();
        
        // Should show enhanced/advanced features in welcome
        let shows_enhancements = stdout.contains("enhanced") || 
                                stdout.contains("difficulty") ||
                                stdout.contains("stats") ||
                                stdout.contains("choose") ||
                                stdout.contains("select");
        
        assert!(
            shows_enhancements,
            "Program should show enhanced features in interface. Output:\n{}",
            String::from_utf8_lossy(&output.stdout)
        );
    }
}

#[test]
fn test_program_supports_difficulty_selection() {
    let mut child = Command::new("cargo")
        .args(&["run"])
        .current_dir(".")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Failed to start program");

    let stdin = child.stdin.as_mut().expect("Failed to open stdin");
    // Try to select different difficulties
    stdin.write_all(b"1\n25\nn\n").expect("Failed to write to stdin");
    drop(stdin);

    let output = child.wait_with_output().expect("Failed to wait for program");
    
    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout).to_lowercase();
        
        // Should mention difficulty levels
        let has_difficulty_system = stdout.contains("easy") || 
                                   stdout.contains("medium") ||
                                   stdout.contains("hard") ||
                                   stdout.contains("difficulty") ||
                                   stdout.contains("level");
        
        assert!(
            has_difficulty_system,
            "Program should support difficulty selection. Output:\n{}",
            String::from_utf8_lossy(&output.stdout)
        );
    }
}

#[test]
fn test_program_mentions_statistics() {
    let mut child = Command::new("cargo")
        .args(&["run"])
        .current_dir(".")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Failed to start program");

    let stdin = child.stdin.as_mut().expect("Failed to open stdin");
    stdin.write_all(b"2\n50\nn\n").expect("Failed to write to stdin");
    drop(stdin);

    let output = child.wait_with_output().expect("Failed to wait for program");
    
    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout).to_lowercase();
        
        // Should show or mention statistics tracking
        let has_stats = stdout.contains("stats") || 
                       stdout.contains("games played") ||
                       stdout.contains("win rate") ||
                       stdout.contains("best") ||
                       stdout.contains("streak") ||
                       stdout.contains("performance");
        
        assert!(
            has_stats,
            "Program should mention or show statistics tracking. Output:\n{}",
            String::from_utf8_lossy(&output.stdout)
        );
    }
}

#[test]
fn test_program_supports_hints() {
    let mut child = Command::new("cargo")
        .args(&["run"])
        .current_dir(".")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Failed to start program");

    let stdin = child.stdin.as_mut().expect("Failed to open stdin");
    // Try to use hint system
    stdin.write_all(b"1\nhint\n25\nn\n").expect("Failed to write to stdin");
    drop(stdin);

    let output = child.wait_with_output().expect("Failed to wait for program");
    
    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout).to_lowercase();
        
        // Should support or mention hints
        let has_hints = stdout.contains("hint") || 
                       stdout.contains("help") ||
                       stdout.contains("💡") ||
                       stdout.contains("available") ||
                       stdout.contains("remaining");
        
        assert!(
            has_hints,
            "Program should support hint system. Output:\n{}",
            String::from_utf8_lossy(&output.stdout)
        );
    }
}

#[test]
fn test_program_handles_multiple_games() {
    let mut child = Command::new("cargo")
        .args(&["run"])
        .current_dir(".")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Failed to start program");

    let stdin = child.stdin.as_mut().expect("Failed to open stdin");
    // Play multiple games
    stdin.write_all(b"2\n50\ny\n2\n25\nn\n").expect("Failed to write to stdin");
    drop(stdin);

    let output = child.wait_with_output().expect("Failed to wait for program");
    
    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout).to_lowercase();
        
        // Should handle multiple games
        let supports_multiple_games = stdout.contains("play again") || 
                                     stdout.contains("another") ||
                                     stdout.contains("new game") ||
                                     stdout.matches("game").count() > 1;
        
        assert!(
            supports_multiple_games,
            "Program should support playing multiple games. Output:\n{}",
            String::from_utf8_lossy(&output.stdout)
        );
    }
}

#[test]
fn test_source_code_shows_enhancements() {
    let source_code = std::fs::read_to_string("src/main.rs")
        .expect("Should be able to read source code");
    
    // Should have some indication of enhanced features in code
    let has_enhancement_code = source_code.contains("difficulty") || 
                              source_code.contains("stats") ||
                              source_code.contains("hint") ||
                              source_code.contains("config") ||
                              source_code.contains("enhanced");
    
    assert!(
        has_enhancement_code,
        "Source code should show implementation of enhanced features"
    );
}

#[test]
fn test_program_creates_persistent_data() {
    // Run program to potentially create data files
    let mut child = Command::new("cargo")
        .args(&["run"])
        .current_dir(".")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Failed to start program");

    let stdin = child.stdin.as_mut().expect("Failed to open stdin");
    stdin.write_all(b"2\n50\nn\n").expect("Failed to write to stdin");
    drop(stdin);

    let _output = child.wait_with_output().expect("Failed to wait for program");
    
    // Check if any data files were created (stats, config, etc.)
    let has_data_files = fs::metadata("game_stats.txt").is_ok() ||
                        fs::metadata("config.txt").is_ok() ||
                        fs::metadata("stats.json").is_ok() ||
                        fs::metadata("game_data.txt").is_ok();
    
    // This is optional - not all implementations may create files immediately
    if has_data_files {
        println!("✅ Program creates persistent data files for enhanced features");
    }
}

#[test]
fn test_program_shows_professional_ui() {
    let mut child = Command::new("cargo")
        .args(&["run"])
        .current_dir(".")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Failed to start program");

    let stdin = child.stdin.as_mut().expect("Failed to open stdin");
    stdin.write_all(b"1\n25\nn\n").expect("Failed to write to stdin");
    drop(stdin);

    let output = child.wait_with_output().expect("Failed to wait for program");
    
    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        
        // Should have professional UI elements
        let has_professional_ui = stdout.contains("🎲") || 
                                 stdout.contains("📊") ||
                                 stdout.contains("🎮") ||
                                 stdout.contains("💡") ||
                                 stdout.contains("🎉") ||
                                 stdout.contains("Welcome") ||
                                 stdout.contains("Stats") ||
                                 stdout.len() > 200; // Reasonably rich output
        
        assert!(
            has_professional_ui,
            "Program should have professional, engaging user interface. Output:\n{}",
            stdout
        );
    }
}

#[test]
fn test_program_handles_edge_cases_gracefully() {
    let mut child = Command::new("cargo")
        .args(&["run"])
        .current_dir(".")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Failed to start program");

    let stdin = child.stdin.as_mut().expect("Failed to open stdin");
    // Test various edge cases
    stdin.write_all(b"invalid\n0\n4\n999\n2\n50\nn\n").expect("Failed to write to stdin");
    drop(stdin);

    let output = child.wait_with_output().expect("Failed to wait for program");
    
    // Enhanced program should handle all edge cases gracefully
    assert!(
        output.status.success(),
        "Enhanced program should handle all edge cases without crashing. Error output:\n{}",
        String::from_utf8_lossy(&output.stderr)
    );
}