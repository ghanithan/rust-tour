//! This program demonstrates different types of comments in Rust.
//! 
//! It calculates the areas of basic geometric shapes (circle, rectangle, triangle)
//! and shows how to properly document Rust code with various comment styles.

// Import PI constant from the standard library for circle calculations
use std::f64::consts::PI;

/// Calculates the area of a circle using the formula: π × r²
/// 
/// # Examples
/// ```
/// let area = calculate_circle_area(5.0);
/// assert!((area - 78.54).abs() < 0.01);
/// ```
/// 
/// # Arguments
/// * `radius` - The radius of the circle
/// 
/// # Returns
/// The area of the circle in square units
fn calculate_circle_area(radius: f64) -> f64 {
    // Using the mathematical formula: Area = π × radius²
    PI * radius * radius
}

/// Calculates the area of a rectangle using the formula: width × height
/// 
/// # Examples
/// ```
/// let area = calculate_rectangle_area(4.0, 6.0);
/// assert_eq!(area, 24.0);
/// ```
fn calculate_rectangle_area(width: f64, height: f64) -> f64 {
    // Simple multiplication: length times width gives area
    width * height
}

/// Calculates the area of a triangle using the formula: (base × height) ÷ 2
/// 
/// # Examples
/// ```
/// let area = calculate_triangle_area(8.0, 3.0);
/// assert_eq!(area, 12.0);
/// ```
fn calculate_triangle_area(base: f64, height: f64) -> f64 {
    /*
    Triangle area formula: (base × height) ÷ 2
    We divide by 2 because a triangle is half of a rectangle
    with the same base and height dimensions.
    */
    base * height / 2.0
}

fn main() {
    // This program demonstrates various Rust comment styles while calculating geometric areas
    
    println!("=== Geometry Area Calculator ===");
    
    // Calculate circle area using radius of 5.0 units
    let circle_area = calculate_circle_area(5.0);
    println!("Circle (radius 5.0): {:.2} square units", circle_area);
    
    // Calculate rectangle area with width=4.0 and height=6.0
    let rectangle_area = calculate_rectangle_area(4.0, 6.0);
    println!("Rectangle (4.0 x 6.0): {:.2} square units", rectangle_area);
    
    // Calculate triangle area using base=8.0 and height=3.0
    let triangle_area = calculate_triangle_area(8.0, 3.0);
    println!("Triangle (base 8.0, height 3.0): {:.2} square units", triangle_area);
    
    /*
    This program successfully demonstrated:
    - Module-level documentation with //!
    - Function documentation with ///
    - Inline comments with //
    - Multi-line comments with /* */
    
    To extend this program, you could add:
    - More geometric shapes (pentagon, hexagon, etc.)
    - Input validation for negative numbers
    - Interactive user input for dimensions
    - Unit conversion between different measurement systems
    */
}