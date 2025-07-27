// Reference solution for Data Types - Integers and Floats
//
// This solution demonstrates proper use of Rust's numeric types,
// including appropriate sizing, type conversions, and overflow handling.

fn main() {
    println!("=== Integer Types ===");
    
    // Part 1: Basic integer types
    let small_number: i32 = 42;
    println!("Small number (i32): {}", small_number);
    
    // Fixed: Use i64 for large numbers that don't fit in i32
    let big_number: i64 = 9_223_372_036_854_775_807;
    println!("Big number: {}", big_number);
    
    // Part 2: Unsigned vs signed integers
    // Fixed: Use signed type for negative numbers
    let population: i32 = -50_000;
    println!("Population: {}", population);
    
    // Fixed: Use larger type for values > 255
    let age: u16 = 300;
    println!("Age: {}", age);
    
    // Part 3: Floating point types
    println!("\n=== Float Types ===");
    
    let precise_pi: f64 = 3.141592653589793;
    println!("Precise π (f64): {}", precise_pi);
    
    // Keep f32 to show precision difference (this is educational)
    let less_precise_pi: f32 = 3.141592653589793;
    println!("Less precise π (f32): {}", less_precise_pi);
    
    // Part 4: Type conversion and arithmetic
    println!("\n=== Type Conversion ===");
    
    let integer_value = 42;
    let float_value = 3.14;
    
    // Fixed: Cast integer to float for arithmetic
    let result = (integer_value as f64) + float_value;
    println!("Result: {}", result);
    
    // Part 5: Integer overflow handling
    println!("\n=== Overflow Handling ===");
    
    let max_u8 = 255u8;
    println!("Max u8: {}", max_u8);
    
    // Fixed: Use checked arithmetic to prevent panic
    let overflow_result = max_u8.checked_add(1).unwrap_or(0);
    println!("Overflow result: {}", overflow_result);
    
    // Part 6: Parsing numbers from strings
    println!("\n=== String to Number Conversion ===");
    
    let number_string = "42";
    
    // Fixed: Parse string to integer
    let parsed_number: i32 = number_string.parse::<i32>().unwrap();
    println!("Parsed number: {}", parsed_number);
    
    let float_string = "3.14159";
    
    // Fixed: Parse string to float
    let parsed_float: f64 = float_string.parse::<f64>().unwrap();
    println!("Parsed float: {}", parsed_float);
}