// Exercise: Comments in Rust
//
// Learn different types of comments in Rust and when to use them.
// This exercise demonstrates line comments, block comments, and documentation comments.

/* 
   This is a block comment that can span multiple lines.
   Block comments are useful for temporarily disabling code
   or for longer explanations.
*/

// TODO: Add appropriate comments throughout this program

fn main() {
    println!("=== Comments Exercise ===");
    
    // This is a line comment - it explains the next line of code
    let temperature = 72;
    
    /* TODO: Add a line comment explaining what this variable represents */
    let is_comfortable = temperature >= 68 && temperature <= 78;
    
    println!("Temperature: {}°F", temperature);
    println!("Comfortable: {}", is_comfortable);
    
    // TODO: Add a block comment explaining this calculation
    let fahrenheit = 100.0;
    let celsius = (fahrenheit - 32.0) * 5.0 / 9.0;
    
    println!("{}°F = {:.1}°C", fahrenheit, celsius);
    
    // TODO: Comment out the next line using a line comment
    println!("This line should be commented out!");
    
    /* TODO: Comment out this entire block using block comments
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
    
    // TODO: Add a comment explaining why we're calling this function
    demonstrate_comment_styles();
    
    // TODO: Add inline comments explaining each step
    let numbers = [1, 2, 3, 4, 5]; // TODO: Explain what this array contains
    let sum: i32 = numbers.iter().sum(); // TODO: Explain what this line does
    let average = sum as f64 / numbers.len() as f64; // TODO: Explain this calculation
    
    println!("Numbers: {:?}", numbers);
    println!("Sum: {}, Average: {:.2}", sum, average);
}

// TODO: Add a documentation comment (///) for this function
fn add_numbers(a: i32, b: i32) -> i32 {
    // TODO: Add a comment explaining the operation
    a + b
}

/* TODO: Convert this block comment to a documentation comment
   This function calculates the area of a rectangle.
   It takes width and height as parameters and returns the area.
*/
fn calculate_area(width: f64, height: f64) -> f64 {
    /* TODO: Add a line comment explaining this calculation */
    width * height
}

// TODO: Add proper documentation comments for this function
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
    
    // TODO: Add more comment examples here
}

// TODO: Add a documentation comment explaining what this struct represents
struct Person {
    name: String,    // TODO: Add inline comment about the name field
    age: u8,         // TODO: Add inline comment about the age field
    email: String,   // TODO: Add inline comment about the email field
}

// TODO: Add documentation comment for this implementation block
impl Person {
    // TODO: Add documentation comment for this method
    fn new(name: String, age: u8, email: String) -> Person {
        /* TODO: Add comment explaining why we create Person this way */
        Person { name, age, email }
    }
    
    // TODO: Add documentation comment explaining what this method does
    fn greet(&self) -> String {
        // TODO: Add comment explaining the string formatting
        format!("Hello, my name is {} and I'm {} years old", self.name, self.age)
    }
}