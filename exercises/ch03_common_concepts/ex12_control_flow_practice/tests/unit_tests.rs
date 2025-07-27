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
fn test_game_starts_correctly() {
    let output = Command::new("cargo")
        .args(&["run", "--quiet"])
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output()
        .expect("Failed to execute program");

    let stdout = String::from_utf8_lossy(&output.stdout);
    
    assert!(
        stdout.contains("Number Guessing Game"),
        "Should display game title"
    );
    
    assert!(
        stdout.contains("I'm thinking of a number between 1 and 100"),
        "Should display game instructions"
    );
    
    assert!(
        stdout.contains("You have 5 attempts"),
        "Should display attempt limit"
    );
}

#[test]
fn test_guess_feedback() {
    let output = Command::new("cargo")
        .args(&["run", "--quiet"])
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output()
        .expect("Failed to execute program");

    let stdout = String::from_utf8_lossy(&output.stdout);
    
    // First guess (25) should be too low (secret is 42)
    assert!(
        stdout.contains("Attempt 1: Your guess is 25"),
        "Should show first guess attempt"
    );
    
    assert!(
        stdout.contains("Too low! Try a higher number") ||
        stdout.contains("📈 Too low!"),
        "Should provide 'too low' feedback for guess 25"
    );
    
    // Second guess (60) should be too high
    assert!(
        stdout.contains("Too high! Try a lower number") ||
        stdout.contains("📉 Too high!"),
        "Should provide 'too high' feedback for guess 60"
    );
}

#[test]
fn test_correct_guess_wins() {
    let output = Command::new("cargo")
        .args(&["run", "--quiet"])
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output()
        .expect("Failed to execute program");

    let stdout = String::from_utf8_lossy(&output.stdout);
    
    // Last guess (42) should be correct
    assert!(
        stdout.contains("Congratulations! You guessed it!") ||
        stdout.contains("🎉 Congratulations!"),
        "Should congratulate on correct guess"
    );
    
    assert!(
        stdout.contains("You won in 4 attempts") ||
        stdout.contains("won in 4"),
        "Should show number of attempts taken to win"
    );
}

#[test]
fn test_attempt_tracking() {
    let output = Command::new("cargo")
        .args(&["run", "--quiet"])
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output()
        .expect("Failed to execute program");

    let stdout = String::from_utf8_lossy(&output.stdout);
    
    // Should show attempt numbers
    assert!(
        stdout.contains("Attempt 1:") &&
        stdout.contains("Attempt 2:") &&
        stdout.contains("Attempt 3:") &&
        stdout.contains("Attempt 4:"),
        "Should track and display attempt numbers"
    );
}

#[test]
fn test_remaining_attempts() {
    let output = Command::new("cargo")
        .args(&["run", "--quiet"])
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output()
        .expect("Failed to execute program");

    let stdout = String::from_utf8_lossy(&output.stdout);
    
    // Should show remaining attempts after guesses
    assert!(
        stdout.contains("You have") && stdout.contains("attempts left"),
        "Should display remaining attempts after each guess"
    );
}

#[test]
fn test_game_flow_logic() {
    let output = Command::new("cargo")
        .args(&["run", "--quiet"])
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output()
        .expect("Failed to execute program");

    let stdout = String::from_utf8_lossy(&output.stdout);
    
    // Game should process all guesses in order
    let lines: Vec<&str> = stdout.lines().collect();
    let attempt1_line = lines.iter().position(|&line| line.contains("Attempt 1: Your guess is 25"));
    let attempt2_line = lines.iter().position(|&line| line.contains("Attempt 2: Your guess is 60"));
    let attempt4_line = lines.iter().position(|&line| line.contains("Attempt 4: Your guess is 42"));
    
    assert!(
        attempt1_line.is_some() && attempt2_line.is_some() && attempt4_line.is_some(),
        "Should process attempts in correct order"
    );
    
    if let (Some(a1), Some(a2), Some(a4)) = (attempt1_line, attempt2_line, attempt4_line) {
        assert!(
            a1 < a2 && a2 < a4,
            "Attempts should appear in chronological order"
        );
    }
}