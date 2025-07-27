use std::process::Command;

#[test]
fn test_program_compiles_and_runs() {
    let output = Command::new("cargo")
        .args(&["run", "--quiet"])
        .current_dir(".")
        .output()
        .expect("Failed to execute program");
    
    assert!(output.status.success(), "Program should compile and run successfully");
}

#[test]
fn test_program_output_contains_expected_content() {
    let output = Command::new("cargo")
        .args(&["run", "--quiet"])
        .current_dir(".")
        .output()
        .expect("Failed to execute program");
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    
    // Check that the program outputs the expected greeting messages
    assert!(stdout.contains("=== Basic Functions ==="), "Should contain header");
    assert!(stdout.contains("Hello, world!"), "Should contain greeting from greet() function");
    assert!(stdout.contains("Hello, Alice!"), "Should contain personalized greeting");
    assert!(stdout.contains("Sum: 8"), "Should display correct sum (5 + 3 = 8)");
    assert!(stdout.contains("Product: 28"), "Should display correct product (4 * 7 = 28)");
    assert!(stdout.contains("Is 10 even? true"), "Should correctly identify 10 as even");
}

// Include the main.rs file to test functions directly
include!("../src/main.rs");

#[cfg(test)]
mod direct_function_tests {
    use super::*;

    #[test]
    fn test_add_numbers_function() {
        assert_eq!(add_numbers(5, 3), 8, "add_numbers(5, 3) should return 8");
        assert_eq!(add_numbers(0, 0), 0, "add_numbers(0, 0) should return 0");
        assert_eq!(add_numbers(-5, 5), 0, "add_numbers(-5, 5) should return 0");
        assert_eq!(add_numbers(10, -3), 7, "add_numbers(10, -3) should return 7");
    }

    #[test]
    fn test_multiply_function() {
        assert_eq!(multiply(4, 7), 28, "multiply(4, 7) should return 28");
        assert_eq!(multiply(0, 5), 0, "multiply(0, 5) should return 0");
        assert_eq!(multiply(-3, 4), -12, "multiply(-3, 4) should return -12");
        assert_eq!(multiply(1, 1), 1, "multiply(1, 1) should return 1");
    }

    #[test]
    fn test_check_even_function() {
        assert_eq!(check_even(10), true, "10 should be even");
        assert_eq!(check_even(9), false, "9 should be odd");
        assert_eq!(check_even(0), true, "0 should be even");
        assert_eq!(check_even(-2), true, "-2 should be even");
        assert_eq!(check_even(-3), false, "-3 should be odd");
        assert_eq!(check_even(2), true, "2 should be even");
        assert_eq!(check_even(1), false, "1 should be odd");
    }

    #[test]
    fn test_functions_exist_and_compile() {
        // Test that all functions can be called without compilation errors
        // This ensures the function signatures are correct
        
        // Functions that don't return values
        greet();
        say_hello_to("Test");
        
        // Functions that return values
        let _ = add_numbers(1, 2);
        let _ = multiply(2, 3);
        let _ = check_even(4);
        
        // If we reach here, all functions compiled correctly
        assert!(true, "All functions compiled and can be called");
    }

    #[test]
    fn test_function_return_types() {
        // Test that functions return the correct types
        let sum: i32 = add_numbers(1, 2);
        let product: i32 = multiply(2, 3);
        let is_even: bool = check_even(4);
        
        assert_eq!(sum, 3);
        assert_eq!(product, 6);
        assert_eq!(is_even, true);
    }

    #[test]
    fn test_edge_cases() {
        // Test edge cases for mathematical functions
        assert_eq!(add_numbers(i32::MAX, 0), i32::MAX, "Should handle maximum i32 value");
        assert_eq!(add_numbers(i32::MIN, 0), i32::MIN, "Should handle minimum i32 value");
        
        assert_eq!(multiply(0, i32::MAX), 0, "Anything multiplied by 0 should be 0");
        assert_eq!(multiply(1, i32::MIN), i32::MIN, "Multiplying by 1 should preserve value");
        
        assert_eq!(check_even(i32::MAX), false, "MAX i32 value should be odd");
        assert_eq!(check_even(i32::MIN), true, "MIN i32 value should be even");
    }
}