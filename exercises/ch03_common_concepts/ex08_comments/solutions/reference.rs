// Exercise: Comments in Rust
//
// Learn different types of comments in Rust and when to use them.
// This exercise demonstrates line comments, block comments, and documentation comments.

/* 
   This is a block comment that can span multiple lines.
   Block comments are useful for temporarily disabling code
   or for longer explanations.
*/

fn main() {
    println!("=== Comments Exercise ===");
    
    // This is a line comment - it explains the next line of code
    let temperature = 72;
    
    // Check if temperature is within comfortable range (68-78°F)
    let is_comfortable = temperature >= 68 && temperature <= 78;
    
    println!("Temperature: {}°F", temperature);
    println!("Comfortable: {}", is_comfortable);
    
    /* This calculation converts Fahrenheit to Celsius using the standard formula:
       C = (F - 32) × 5/9
       This is useful for international temperature comparisons */
    let fahrenheit = 100.0;
    let celsius = (fahrenheit - 32.0) * 5.0 / 9.0;
    
    println!("{}°F = {:.1}°C", fahrenheit, celsius);
    
    // This line is commented out because it's not needed for the exercise
    // println!("This line should be commented out!");
    
    /* This entire block is commented out for demonstration purposes
    let unused_variable = 42;
    println!("This should not print: {}", unused_variable);
    let another_unused = "test";
    println!("This should also not print: {}", another_unused);
    */
    
    // Function calls with explanatory comments
    let result = add_numbers(5, 3);
    println!("Addition result: {}", result);
    
    let area = calculate_area(4.5, 6.2);
    println!("Rectangle area: {:.2}", area);
    
    // Demonstrate different comment styles and best practices
    demonstrate_comment_styles();
    
    // Calculate statistics for a sample array of numbers
    let numbers = [1, 2, 3, 4, 5]; // Array of consecutive integers for testing
    let sum: i32 = numbers.iter().sum(); // Sum all elements in the array
    let average = sum as f64 / numbers.len() as f64; // Convert to f64 for decimal precision
    
    println!("Numbers: {:?}", numbers);
    println!("Sum: {}, Average: {:.2}", sum, average);
}

/// Adds two integers and returns the result.
/// 
/// # Arguments
/// * `a` - The first integer
/// * `b` - The second integer
/// 
/// # Examples
/// ```
/// let result = add_numbers(2, 3);
/// assert_eq!(result, 5);
/// ```
fn add_numbers(a: i32, b: i32) -> i32 {
    // Perform simple addition and return the result
    a + b
}

/// Calculates the area of a rectangle.
/// 
/// Takes width and height as parameters and returns the area as a floating-point number.
/// 
/// # Arguments
/// * `width` - The width of the rectangle
/// * `height` - The height of the rectangle
/// 
/// # Returns
/// The area of the rectangle (width × height)
fn calculate_area(width: f64, height: f64) -> f64 {
    // Multiply width by height to get area
    width * height
}

/// Demonstrates different commenting styles and best practices in Rust.
/// 
/// This function shows various ways to write comments effectively,
/// including explanatory comments, TODO markers, and section dividers.
fn demonstrate_comment_styles() {
    println!("\n=== Comment Styles Demo ===");
    
    // Style 1: Explanatory comments
    // These comments explain WHY something is done, not just WHAT
    let max_attempts = 3; // Limit retries to prevent infinite loops
    
    // Style 2: TODO comments
    // TODO: Implement retry logic here
    
    // Style 3: Section dividers
    // ========================
    // Configuration Section
    // ========================
    let config_value = 42;
    println!("Config: {}", config_value);
    
    /* Style 4: Multi-line explanations
       When you need to explain complex logic,
       block comments can be more readable
       than multiple line comments. */
    
    // Style 5: Inline documentation
    let result = max_attempts * 2; // Double the max attempts for special cases
    println!("Max attempts: {}, Result: {}", max_attempts, result);
    
    // Style 6: Algorithm explanation
    // This could be used to explain complex algorithms step by step
}

/// Represents a person with basic contact information.
/// 
/// This struct stores essential data about a person including
/// their name, age, and email address for contact purposes.
struct Person {
    name: String,    // Person's full name
    age: u8,         // Person's age in years (0-255 range is sufficient)
    email: String,   // Person's email address for contact
}

/// Implementation of methods for the Person struct.
impl Person {
    /// Creates a new Person instance.
    /// 
    /// # Arguments
    /// * `name` - The person's full name
    /// * `age` - The person's age
    /// * `email` - The person's email address
    /// 
    /// # Returns
    /// A new Person instance with the provided information
    fn new(name: String, age: u8, email: String) -> Person {
        // Create and return a new Person using struct shorthand syntax
        Person { name, age, email }
    }
    
    /// Returns a greeting message from this person.
    /// 
    /// # Returns
    /// A formatted string containing the person's name and age
    fn greet(&self) -> String {
        // Use format! macro to create a personalized greeting message
        format!("Hello, my name is {} and I'm {} years old", self.name, self.age)
    }
}