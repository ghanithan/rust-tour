//! This program demonstrates different types of comments in Rust.
//! 
//! TODO: Add a brief description of what this program does
//! (Hint: It calculates areas of geometric shapes)

// TODO: Add a regular comment explaining what the std::f64::consts module provides
use std::f64::consts::PI;

/// TODO: Add documentation for this function
/// 
/// This function should:
/// - Explain what it calculates
/// - Show the formula used
/// - Include an example of usage
/// - Document the parameter and return value
fn calculate_circle_area(radius: f64) -> f64 {
    // TODO: Add a comment explaining the formula
    PI * radius * radius
}

/// TODO: Add documentation for this function
/// Include an example showing how to call it
fn calculate_rectangle_area(width: f64, height: f64) -> f64 {
    // TODO: Add inline comment explaining this calculation
    width * height
}

/// TODO: Document this function with proper examples
fn calculate_triangle_area(base: f64, height: f64) -> f64 {
    /*
    TODO: Replace this block comment with proper explanation of 
    the triangle area formula and why we divide by 2.0
    */
    base * height / 2.0
}

fn main() {
    // TODO: Add comment explaining what this program demonstrates
    
    println!("=== Geometry Area Calculator ===");
    
    // TODO: Add comments for each calculation explaining the values chosen
    let circle_area = calculate_circle_area(5.0);
    println!("Circle (radius 5.0): {:.2} square units", circle_area);
    
    let rectangle_area = calculate_rectangle_area(4.0, 6.0);
    println!("Rectangle (4.0 x 6.0): {:.2} square units", rectangle_area);
    
    let triangle_area = calculate_triangle_area(8.0, 3.0);
    println!("Triangle (base 8.0, height 3.0): {:.2} square units", triangle_area);
    
    /*
    TODO: Add a multi-line comment here explaining:
    - What this program accomplished
    - What comment types were demonstrated
    - How someone could extend this program
    */
}