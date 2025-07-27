use ex07_rectangle_program::*;
use std::process::Command;

#[test]
fn test_rectangle_creation() {
    let rect = Rectangle {
        width: 30,
        height: 50,
    };
    assert_eq!(rect.width, 30);
    assert_eq!(rect.height, 50);
}

#[test]
fn test_area_calculation_basic() {
    let rect = Rectangle {
        width: 10,
        height: 20,
    };
    assert_eq!(rect.area(), 200);
}

#[test]
fn test_area_calculation_square() {
    let rect = Rectangle {
        width: 15,
        height: 15,
    };
    assert_eq!(rect.area(), 225);
}

#[test]
fn test_area_calculation_edge_cases() {
    let rect1 = Rectangle {
        width: 1,
        height: 1,
    };
    assert_eq!(rect1.area(), 1);

    let rect2 = Rectangle {
        width: 100,
        height: 1,
    };
    assert_eq!(rect2.area(), 100);
}

#[test]
fn test_can_hold_smaller_rectangle() {
    let larger = Rectangle {
        width: 8,
        height: 7,
    };
    let smaller = Rectangle {
        width: 5,
        height: 1,
    };
    assert!(larger.can_hold(&smaller));
}

#[test]
fn test_cannot_hold_larger_rectangle() {
    let smaller = Rectangle {
        width: 5,
        height: 1,
    };
    let larger = Rectangle {
        width: 8,
        height: 7,
    };
    assert!(!smaller.can_hold(&larger));
}

#[test]
fn test_can_hold_same_size() {
    let rect1 = Rectangle {
        width: 8,
        height: 7,
    };
    let rect2 = Rectangle {
        width: 8,
        height: 7,
    };
    // Same size should return true (>= behavior)
    assert!(rect1.can_hold(&rect2));
}

#[test]
fn test_can_hold_partial_overlap() {
    let rect1 = Rectangle {
        width: 10,
        height: 5,
    };
    let rect2 = Rectangle {
        width: 5,
        height: 10,
    };
    // rect1 is wider but shorter, rect2 is narrower but taller
    assert!(!rect1.can_hold(&rect2));
    assert!(!rect2.can_hold(&rect1));
}

#[test]
fn test_debug_trait_is_implemented() {
    let rect = Rectangle {
        width: 30,
        height: 50,
    };
    let debug_output = format!("{:?}", rect);
    assert!(debug_output.contains("Rectangle"));
    assert!(debug_output.contains("30"));
    assert!(debug_output.contains("50"));
}

#[test]
fn test_program_compiles_and_runs() {
    let output = Command::new("cargo")
        .args(&["run", "--quiet"])
        .current_dir("/workspaces/rust-tour/exercises/ch05_using_structs/ex07_rectangle_program")
        .output()
        .expect("Failed to execute program");

    assert!(output.status.success(), "Program should compile and run successfully");
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    
    // Check that the program produces expected output
    assert!(stdout.contains("rect1 is"), "Should print rect1 debug output");
    assert!(stdout.contains("area"), "Should print area information");
    assert!(stdout.contains("1500"), "Should calculate correct area (30 * 50 = 1500)");
    assert!(stdout.contains("Can rect1 hold"), "Should test can_hold functionality");
}

#[test]
fn test_main_function_behavior() {
    let output = Command::new("cargo")
        .args(&["run", "--quiet"])
        .current_dir("/workspaces/rust-tour/exercises/ch05_using_structs/ex07_rectangle_program")
        .output()
        .expect("Failed to execute program");

    let stdout = String::from_utf8_lossy(&output.stdout);
    
    // Test specific expected outputs based on the TODO comments in main.rs
    assert!(stdout.contains("Rectangle { width: 30, height: 50 }") || 
           stdout.contains("width: 30") && stdout.contains("height: 50"), 
           "Should print rectangle with width 30 and height 50");
           
    assert!(stdout.contains("1500"), "Should calculate area as 1500 (30 * 50)");
    
    // Test can_hold results: rect1(30x50) should hold rect2(10x40) but not rect3(60x25)
    let lines: Vec<&str> = stdout.lines().collect();
    let can_hold_lines: Vec<&str> = lines.iter()
        .filter(|line| line.to_lowercase().contains("can") && line.to_lowercase().contains("hold"))
        .cloned()
        .collect();
    
    assert!(can_hold_lines.len() >= 2, "Should have at least 2 can_hold test outputs");
}