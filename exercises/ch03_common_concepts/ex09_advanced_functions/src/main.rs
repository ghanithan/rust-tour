// Exercise: Advanced Function Concepts
//
// Master advanced function concepts in Rust:
// - Statements vs expressions
// - Return values and early returns
// - Function parameters and ownership
// - Expression-based programming

fn main() {
    println!("=== Advanced Function Concepts ===");
    
    // Part 1: Expression-based functions
    println!("\n--- Expression vs Statement ---");
    
    let result1 = add_with_statement(5, 3);
    let result2 = add_with_expression(5, 3);
    
    println!("Statement-based addition: {}", result1);
    println!("Expression-based addition: {}", result2);
    
    // Part 2: Early returns and conditional logic
    println!("\n--- Early Returns ---");
    
    let numbers = [10, -5, 15, 0, -3, 20];
    
    for num in numbers.iter() {
        let result = check_number_status(*num);
        println!("Number {}: {}", num, result);
    }
    
    // Part 3: Multiple return types with Option
    println!("\n--- Functions with Option Returns ---");
    
    let test_values = [10, 25, 100, 150];
    
    for value in test_values.iter() {
        match find_square_root(*value) {
            Some(sqrt) => println!("√{} = {:.2}", value, sqrt),
            None => println!("{} is too large for this function", value),
        }
    }
    
    // Part 4: Functions that modify parameters
    println!("\n--- Parameter Modification ---");
    
    let mut counter = 0;
    println!("Initial counter: {}", counter);
    
    increment_counter(&mut counter);
    println!("After increment: {}", counter);
    
    double_counter(&mut counter);
    println!("After doubling: {}", counter);
    
    // Part 5: Complex return types
    println!("\n--- Complex Return Types ---");
    
    let grades = [85, 92, 78, 96, 88];
    let (min, max, avg) = analyze_grades(&grades);
    
    println!("Grades: {:?}", grades);
    println!("Min: {}, Max: {}, Average: {:.1}", min, max, avg);
    
    // Part 6: Nested function calls
    println!("\n--- Nested Function Calls ---");
    
    let base = 3;
    let exponent = 4;
    let power_result = power(base, exponent);
    let factorial_result = factorial(5);
    
    println!("{}^{} = {}", base, exponent, power_result);
    println!("5! = {}", factorial_result);
    
    // Combine results in a calculation
    let combined = combine_results(power_result, factorial_result);
    println!("Combined calculation: {}", combined);
    
    // Part 7: Function composition
    println!("\n--- Function Composition ---");
    
    let input = 5;
    let step1 = double_number(input);
    let step2 = add_ten(step1);
    let step3 = square_number(step2);
    
    println!("Input: {} → Double: {} → Add 10: {} → Square: {}", 
             input, step1, step2, step3);
    
    // Same thing in one line
    let composed = square_number(add_ten(double_number(input)));
    println!("Composed result: {}", composed);
}

// TODO: Implement function using explicit return statement
fn add_with_statement(a: i32, b: i32) -> i32 {
    let sum = a + b;
    return sum; // Explicit return - this is a statement
}

// TODO: Implement function using expression (no return keyword)
fn add_with_expression(a: i32, b: i32) -> i32 {
    /* Return a + b without using return keyword */
}

// TODO: Implement function with early returns for different conditions
fn check_number_status(num: i32) -> &'static str {
    // Return "negative" if num < 0
    if /* condition for negative */ {
        /* early return for negative */
    }
    
    // Return "zero" if num == 0
    if /* condition for zero */ {
        /* early return for zero */
    }
    
    // Return "small positive" if num <= 10
    if /* condition for small positive */ {
        /* early return for small positive */
    }
    
    // Return "large positive" for everything else
    /* final return for large positive */
}

// TODO: Implement function that returns Some(sqrt) for numbers <= 100, None otherwise
fn find_square_root(num: i32) -> Option<f64> {
    if /* condition for numbers too large */ {
        /* return None */
    } else {
        /* return Some with square root */
    }
}

// TODO: Implement function that takes mutable reference and increments by 1
fn increment_counter(counter: &mut i32) {
    /* increment the counter value */
}

// TODO: Implement function that takes mutable reference and doubles the value
fn double_counter(counter: &mut i32) {
    /* double the counter value */
}

// TODO: Implement function that returns tuple of (min, max, average)
fn analyze_grades(grades: &[i32]) -> (i32, i32, f64) {
    let min = /* find minimum value in grades */;
    let max = /* find maximum value in grades */;
    let sum: i32 = /* sum all grades */;
    let avg = sum as f64 / grades.len() as f64;
    
    /* return tuple of (min, max, avg) */
}

// TODO: Implement recursive function to calculate power (base^exponent)
fn power(base: i32, exponent: u32) -> i32 {
    if exponent == 0 {
        /* base case: anything to power 0 is 1 */
    } else {
        /* recursive case: base * power(base, exponent - 1) */
    }
}

// TODO: Implement recursive function to calculate factorial
fn factorial(n: u32) -> u32 {
    if /* base case condition */ {
        /* return 1 for factorial(0) and factorial(1) */
    } else {
        /* recursive case: n * factorial(n - 1) */
    }
}

// TODO: Implement function that combines two results with a formula
fn combine_results(power_result: i32, factorial_result: u32) -> i32 {
    // Calculate: (power_result + factorial_result) / 2
    /* implement the calculation */
}

// TODO: Implement helper functions for composition

// Double a number
fn double_number(n: i32) -> i32 {
    /* return n * 2 */
}

// Add 10 to a number
fn add_ten(n: i32) -> i32 {
    /* return n + 10 */
}

// Square a number
fn square_number(n: i32) -> i32 {
    /* return n * n */
}