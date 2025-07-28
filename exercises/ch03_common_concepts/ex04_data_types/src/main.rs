// Exercise: Data Types - Integers and Floats
//
// Rust has various numeric types with different sizes and capabilities.
// This exercise helps you understand when to use different numeric types
// and how to work with them safely.
//
// Fix the compilation errors by using appropriate data types.

fn main() {
    println!("=== Integer Types ===");
    
    // Part 1: Basic integer types
    // Different integer types have different ranges and sizes
    
    // This works fine - small number fits in i32
    let small_number: i32 = 42;
    println!("Small number (i32): {}", small_number);
    
    // TODO: Fix this - the number is too big for i32
    // Use a larger integer type (try i64)
    let big_number: i32 = 9_223_372_036_854_775_807;
    println!("Big number: {}", big_number);
    
    // Part 2: Unsigned vs signed integers
    // Unsigned integers (u32, u64) can't store negative numbers
    // but can store larger positive numbers
    
    // TODO: Fix this - can't store negative number in unsigned type
    let population: u32 = -50_000;
    println!("Population: {}", population);
    
    // TODO: Fix this - number too big for u8 (max 255)
    let age: u8 = 300;
    println!("Age: {}", age);
    
    // Part 3: Floating point types
    println!("\n=== Float Types ===");
    
    // f32 vs f64 precision differences
    let precise_pi: f64 = 3.141592653589793;
    println!("Precise π (f64): {}", precise_pi);
    
    // TODO: Fix this precision loss by using appropriate type
    let less_precise_pi: f32 = 3.141592653589793;
    println!("Less precise π (f32): {}", less_precise_pi);
    
    // Part 4: Type conversion and arithmetic
    println!("\n=== Type Conversion ===");
    
    let integer_value = 42;
    let float_value = 3.14;
    
    // TODO: Fix this - can't directly add different numeric types
    let result = integer_value + float_value;
    println!("Result: {}", result);
    
    // Part 5: Integer overflow handling
    println!("\n=== Overflow Handling ===");
    
    let max_u8 = 255u8;
    println!("Max u8: {}", max_u8);
    
    // TODO: This will panic in debug mode - use checked arithmetic
    let overflow_result = max_u8 + 1;
    println!("Overflow result: {}", overflow_result);
    
    // Part 6: Parsing numbers from strings
    println!("\n=== String to Number Conversion ===");
    
    let number_string = "42";
    
    // TODO: Parse this string to the appropriate integer type
    // Use .parse::<type>().unwrap()
    let parsed_number: i32 = number_string;
    println!("Parsed number: {}", parsed_number);
    
    let float_string = "3.14159";
    
    // TODO: Parse this string to the appropriate float type
    let parsed_float: f64 = float_string;
    println!("Parsed float: {}", parsed_float);
}