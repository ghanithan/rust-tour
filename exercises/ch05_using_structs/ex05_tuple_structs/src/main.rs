// BUG: Define Color as a tuple struct with three u8 values for RGB
// struct Color = (u8, u8, u8);  // This is incorrect syntax

// BUG: Define Point as a tuple struct with two f64 values for coordinates  
// struct Point = (f64, f64);  // This is incorrect syntax

// BUG: Define a unit-like struct for a marker type
// struct AlwaysEqual = ();  // This is incorrect syntax

// BUG: Define UserId as a tuple struct wrapping u32 (newtype pattern)
// struct UserId = u32;  // This is incorrect syntax

fn main() {
    println!("Creating tuple structs...");
    
    // BUG: Create a red color - fix the constructor syntax
    let red = Color(255, 0, 0);
    println!("Red color: RGB({}, {}, {})", red.0, red.1, red.2);
    
    // BUG: Create a point - fix the constructor syntax  
    let origin = Point(0.0, 0.0);
    println!("Origin point: ({}, {})", origin.0, origin.1);
    
    // BUG: Create a unit-like struct instance
    let subject = AlwaysEqual;
    println!("Unit-like struct created: {:?}", subject);
    
    // BUG: Create user IDs with the newtype pattern
    let user1 = UserId(42);
    let user2 = UserId(100);
    
    println!("User 1 ID: {}", user1.0);
    println!("User 2 ID: {}", user2.0);
    
    // BUG: Test comparison - this should work for the same types
    // but fail for different types (demonstrating type safety)
    println!("Testing type safety...");
    
    let point1 = Point(1.0, 2.0);
    let point2 = Point(1.0, 2.0);
    
    // BUG: This comparison should work (same types)
    // if point1 == point2 {
    //     println!("Points are equal");
    // }
    
    // BUG: This should NOT compile (different types)
    // Uncomment to see the type safety in action:
    // let color = Color(1, 2, 3);
    // let point = Point(1.0, 2.0);
    // if color == point {  // This should cause a compile error
    //     println!("This shouldn't work");
    // }
    
    println!("Testing methods on tuple structs...");
    
    // Test methods that should be implemented
    println!("Red color brightness: {}", red.brightness());
    println!("Distance from origin to point: {}", point1.distance_from_origin());
    println!("User 1 is valid: {}", user1.is_valid());
}

// BUG: Implement methods for the tuple structs
impl Color {
    // BUG: Method to calculate brightness (sum of RGB values)
    fn brightness(&self) -> u32 {
        // TODO: Return sum of all three color components
        unimplemented!()
    }
    
    // BUG: Method to check if color is grayscale
    fn is_grayscale(&self) -> bool {
        // TODO: Return true if all RGB values are equal
        unimplemented!()
    }
}

impl Point {
    // BUG: Method to calculate distance from origin
    fn distance_from_origin(&self) -> f64 {
        // TODO: Use Pythagorean theorem: sqrt(x² + y²)
        unimplemented!()
    }
    
    // BUG: Method to translate point by given offsets
    fn translate(&mut self, dx: f64, dy: f64) {
        // TODO: Add dx to x coordinate and dy to y coordinate
        unimplemented!()
    }
}

impl UserId {
    // BUG: Method to validate user ID
    fn is_valid(&self) -> bool {
        // TODO: Return true if ID is greater than 0
        unimplemented!()
    }
}

// BUG: Implement Debug for AlwaysEqual
// #[derive(Debug)]
// Add the derive attribute above the struct definition

// BUG: Implement PartialEq for Point to enable comparison
// #[derive(PartialEq)]
// Add the derive attribute above the struct definition

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color_creation() {
        let color = Color(255, 128, 64);
        assert_eq!(color.0, 255);
        assert_eq!(color.1, 128);
        assert_eq!(color.2, 64);
    }

    #[test]
    fn test_point_creation() {
        let point = Point(3.0, 4.0);
        assert_eq!(point.0, 3.0);
        assert_eq!(point.1, 4.0);
    }

    #[test]
    fn test_user_id_newtype() {
        let user_id = UserId(123);
        assert_eq!(user_id.0, 123);
    }

    #[test]
    fn test_color_brightness() {
        let color = Color(100, 150, 200);
        assert_eq!(color.brightness(), 450);
    }

    #[test]
    fn test_point_distance() {
        let point = Point(3.0, 4.0);
        assert_eq!(point.distance_from_origin(), 5.0);
    }

    #[test]
    fn test_user_id_validation() {
        let valid_id = UserId(1);
        let invalid_id = UserId(0);
        
        assert!(valid_id.is_valid());
        assert!(!invalid_id.is_valid());
    }
}