// Reference solution for Tuple Structs and Unit-Like Structs exercise

// Define Color as a tuple struct with three u8 values for RGB
struct Color(u8, u8, u8);

// Define Point as a tuple struct with two f64 values for coordinates
#[derive(PartialEq, Debug)]
struct Point(f64, f64);

// Define a unit-like struct for a marker type
#[derive(Debug)]
struct AlwaysEqual;

// Define UserId as a tuple struct wrapping u32 (newtype pattern)
struct UserId(u32);

fn main() {
    println!("Creating tuple structs...");
    
    // Create a red color
    let red = Color(255, 0, 0);
    println!("Red color: RGB({}, {}, {})", red.0, red.1, red.2);
    
    // Create a point
    let origin = Point(0.0, 0.0);
    println!("Origin point: ({}, {})", origin.0, origin.1);
    
    // Create a unit-like struct instance
    let subject = AlwaysEqual;
    println!("Unit-like struct created: {:?}", subject);
    
    // Create user IDs with the newtype pattern
    let user1 = UserId(42);
    let user2 = UserId(100);
    
    println!("User 1 ID: {}", user1.0);
    println!("User 2 ID: {}", user2.0);
    
    // Test comparison - this works for the same types
    // but would fail for different types (demonstrating type safety)
    println!("Testing type safety...");
    
    let point1 = Point(1.0, 2.0);
    let point2 = Point(1.0, 2.0);
    
    // This comparison works (same types) because we derived PartialEq
    if point1 == point2 {
        println!("Points are equal");
    }
    
    // This would NOT compile (different types) - demonstrating type safety:
    // let color = Color(1, 2, 3);
    // let point = Point(1.0, 2.0);
    // if color == point {  // This would cause a compile error
    //     println!("This shouldn't work");
    // }
    
    println!("Testing methods on tuple structs...");
    
    // Test methods that are implemented
    println!("Red color brightness: {}", red.brightness());
    println!("Distance from origin to point: {}", point1.distance_from_origin());
    println!("User 1 is valid: {}", user1.is_valid());
}

// Implement methods for the tuple structs
impl Color {
    // Method to calculate brightness (sum of RGB values)
    fn brightness(&self) -> u32 {
        // Return sum of all three color components as u32 to prevent overflow
        (self.0 as u32) + (self.1 as u32) + (self.2 as u32)
    }
    
    // Method to check if color is grayscale
    fn is_grayscale(&self) -> bool {
        // Return true if all RGB values are equal
        self.0 == self.1 && self.1 == self.2
    }
}

impl Point {
    // Method to calculate distance from origin
    fn distance_from_origin(&self) -> f64 {
        // Use Pythagorean theorem: sqrt(x² + y²)
        (self.0 * self.0 + self.1 * self.1).sqrt()
    }
    
    // Method to translate point by given offsets
    fn translate(&mut self, dx: f64, dy: f64) {
        // Add dx to x coordinate and dy to y coordinate
        self.0 += dx;
        self.1 += dy;
    }
}

impl UserId {
    // Method to validate user ID
    fn is_valid(&self) -> bool {
        // Return true if ID is greater than 0
        self.0 > 0
    }
}

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

    #[test]
    fn test_color_grayscale() {
        let gray = Color(128, 128, 128);
        let not_gray = Color(128, 64, 192);
        
        assert!(gray.is_grayscale());
        assert!(!not_gray.is_grayscale());
    }

    #[test]
    fn test_point_translate() {
        let mut point = Point(1.0, 1.0);
        point.translate(2.0, 3.0);
        
        assert_eq!(point.0, 3.0);
        assert_eq!(point.1, 4.0);
    }

    #[test]
    fn test_point_equality() {
        let point1 = Point(1.0, 2.0);
        let point2 = Point(1.0, 2.0);
        let point3 = Point(2.0, 1.0);
        
        assert_eq!(point1, point2);
        assert_ne!(point1, point3);
    }

    #[test]
    fn test_unit_struct_creation() {
        let _always_equal1 = AlwaysEqual;
        let _always_equal2 = AlwaysEqual;
        
        // Unit structs can be created multiple times
        // They don't take up any memory
        assert_eq!(std::mem::size_of::<AlwaysEqual>(), 0);
    }

    #[test]
    fn test_type_safety() {
        let user_id1 = UserId(42);
        let user_id2 = UserId(42);
        
        // Even though both wrap the same value, they're different instances
        // But we can compare their inner values
        assert_eq!(user_id1.0, user_id2.0);
        
        // This demonstrates the newtype pattern: we've created a new type
        // that wraps u32 but is distinct from raw u32 values
        let raw_id: u32 = 42;
        assert_eq!(user_id1.0, raw_id);
        
        // However, user_id1 and raw_id are different types and can't be
        // directly compared without accessing the inner value
    }

    #[test]
    fn test_edge_cases() {
        // Test edge case for distance calculation
        let origin = Point(0.0, 0.0);
        assert_eq!(origin.distance_from_origin(), 0.0);
        
        // Test edge case for color brightness
        let black = Color(0, 0, 0);
        assert_eq!(black.brightness(), 0);
        
        let white = Color(255, 255, 255);
        assert_eq!(white.brightness(), 765);
        
        // Test edge case for user ID validation
        let zero_id = UserId(0);
        assert!(!zero_id.is_valid());
        
        let max_id = UserId(u32::MAX);
        assert!(max_id.is_valid());
    }
}