// Reference solution for Rectangle Program exercise
// This demonstrates the complete implementation following Rust Book Chapter 5.2

#[derive(Debug)]
pub struct Rectangle {
    pub width: u32,
    pub height: u32,
}

impl Rectangle {
    /// Calculates the area of the rectangle
    /// 
    /// # Returns
    /// The area as a u32 (width * height)
    pub fn area(&self) -> u32 {
        self.width * self.height
    }

    /// Determines if this rectangle can hold another rectangle
    /// 
    /// # Arguments
    /// * `other` - A reference to another Rectangle
    /// 
    /// # Returns
    /// true if both width and height are greater than or equal to the other rectangle's dimensions
    pub fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    // Create a rectangle with width 30 and height 50
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    // Print the rectangle using debug formatting {:?}
    println!("rect1 is {:?}", rect1);

    // Print the area using the area method
    println!("The area of the rectangle is {} square pixels.", rect1.area());

    // Create a second rectangle with width 10 and height 40
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };

    // Test if the first rectangle can hold the second
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));

    // Create a third rectangle with width 60 and height 25
    let rect3 = Rectangle {
        width: 60,
        height: 25,
    };

    // Test if the first rectangle can hold the third
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_area_calculation() {
        let rect = Rectangle {
            width: 10,
            height: 20,
        };
        assert_eq!(rect.area(), 200);
    }

    #[test]
    fn test_can_hold() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };
        let same_size = Rectangle {
            width: 8,
            height: 7,
        };

        assert!(larger.can_hold(&smaller));
        assert!(!smaller.can_hold(&larger));
        assert!(larger.can_hold(&same_size));
    }

    #[test]
    fn test_rectangle_instantiation() {
        let rect = Rectangle {
            width: 42,
            height: 24,
        };
        assert_eq!(rect.width, 42);
        assert_eq!(rect.height, 24);
    }

    #[test]
    fn test_debug_formatting() {
        let rect = Rectangle {
            width: 30,
            height: 50,
        };
        let debug_str = format!("{:?}", rect);
        assert!(debug_str.contains("Rectangle"));
        assert!(debug_str.contains("30"));
        assert!(debug_str.contains("50"));
    }
}