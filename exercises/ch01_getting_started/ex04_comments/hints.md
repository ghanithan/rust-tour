# Hints for Documentation and Code Comments

## Level 1: Conceptual Hint

Rust has several types of comments, each serving a different purpose:

1. **Regular comments (`//` and `/* */`)** - For explaining code to other programmers (and yourself!)
2. **Documentation comments (`///` and `//!`)** - For generating formal documentation with `cargo doc`

**When to use regular comments:**
- Explain complex logic or algorithms
- Clarify business rules or domain knowledge
- Add context that isn't obvious from the code
- Explain "why" something is done, not just "what"

**When to use documentation comments:**
- Document public functions, structs, modules
- Provide usage examples
- Explain parameters and return values
- Create user-facing documentation

**Good commenting principles:**
- Comments should add value, not repeat obvious code
- Keep comments up to date when code changes
- Use proper grammar and spelling
- Be concise but complete

The Rust community values good documentation highly - well-documented code is considered professional and maintainable!

## Level 2: Strategic Hint

For this exercise, you need to replace TODO markers with appropriate comments:

**Module-level documentation (at the top):**
Use `//!` to describe what the entire program does.

**Function documentation:**
Use `///` before each function to document:
- What the function does
- What parameters it takes
- What it returns
- Include an example with the `# Examples` section

**Inline comments:**
Use `//` to explain:
- Mathematical formulas
- Why specific values were chosen
- What each section of code accomplishes

**Block comments:**
Use `/* */` for longer explanations that span multiple lines.

**Structure your documentation like this:**
```rust
/// Brief one-line description.
/// 
/// Longer description if needed.
/// 
/// # Examples
/// ```
/// let result = function_name(parameter);
/// assert_eq!(result, expected_value);
/// ```
```

Remember to explain the mathematical formulas and why certain calculations are performed!

## Level 3: Implementation Hint

Here are the specific comments to add:

**Module documentation (top of file):**
```rust
//! This program demonstrates different types of comments in Rust.
//! 
//! It calculates the areas of basic geometric shapes (circle, rectangle, triangle)
//! and shows how to properly document Rust code with various comment styles.
```

**Import comment:**
```rust
// Import PI constant from the standard library for circle calculations
use std::f64::consts::PI;
```

**Function documentation examples:**

```rust
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
```

**Inline comments for main function:**
```rust
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
```