#[derive(Debug, Clone)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // TODO: Implement the `area` method
    // Should return the area of the rectangle (width * height)
    // Takes an immutable reference to self since we only need to read
    
    // TODO: Implement the `perimeter` method
    // Should return the perimeter of the rectangle (2 * (width + height))
    // Takes an immutable reference to self since we only need to read
    
    // TODO: Implement the `can_hold` method
    // Should return true if this rectangle can completely hold another rectangle
    // Takes an immutable reference to self and another Rectangle
    // Returns true if both width and height are greater than or equal to the other rectangle
    
    // TODO: Implement the `has_same_area` method
    // Should return true if this rectangle has the same area as another rectangle
    // Takes an immutable reference to self and another Rectangle
    
    // TODO: Implement the `scale` method
    // Should multiply both width and height by the given factor
    // Takes a mutable reference to self since it modifies the rectangle
    // Parameter: factor (u32)
    
    // TODO: Implement the `to_square` method
    // Should consume the rectangle and return a new Rectangle that is a square
    // The square should have sides equal to the minimum of width and height
    // Takes ownership of self (moves the rectangle)
}

fn main() {
    println!("Creating rectangles...");
    
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    
    let rect3 = Rectangle {
        width: 60,
        height: 25,
    };
    
    // Display rectangles using the debug trait and area method
    println!("Rectangle: {}x{} (area: {})", rect1.width, rect1.height, rect1.area());
    println!("Rectangle: {}x{} (area: {})", rect2.width, rect2.height, rect2.area());
    println!("Rectangle: {}x{} (area: {})", rect3.width, rect3.height, rect3.area());
    
    println!("\nTesting area calculations...");
    println!("rect1 area: {}", rect1.area());
    println!("rect2 area: {}", rect2.area());
    
    println!("\nTesting comparisons...");
    println!("rect1 can hold rect2: {}", rect1.can_hold(&rect2));
    println!("rect2 can hold rect1: {}", rect2.can_hold(&rect1));
    println!("rect1 has same area as rect3: {}", rect1.has_same_area(&rect3));
    
    println!("\nTesting mutations...");
    let mut rect_for_scaling = rect1.clone();
    println!("Before scaling: Rectangle: {}x{} (area: {})", 
             rect_for_scaling.width, rect_for_scaling.height, rect_for_scaling.area());
    
    rect_for_scaling.scale(2);
    println!("After scaling by 2: Rectangle: {}x{} (area: {})",
             rect_for_scaling.width, rect_for_scaling.height, rect_for_scaling.area());
    
    println!("\nMaking square from rectangle...");
    // Use rect2 to create a square (this will consume rect2)
    let square = rect2.to_square();
    println!("Original rectangle was consumed to create square: {}x{} (area: {})",
             square.width, square.height, square.area());
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
    fn test_perimeter_calculation() {
        let rect = Rectangle {
            width: 10,
            height: 20,
        };
        assert_eq!(rect.perimeter(), 60);
    }

    #[test]
    fn test_can_hold() {
        let larger = Rectangle {
            width: 30,
            height: 50,
        };
        let smaller = Rectangle {
            width: 10,
            height: 40,
        };
        
        assert!(larger.can_hold(&smaller));
        assert!(!smaller.can_hold(&larger));
    }

    #[test]
    fn test_same_area() {
        let rect1 = Rectangle {
            width: 10,
            height: 20,
        };
        let rect2 = Rectangle {
            width: 20,
            height: 10,
        };
        let rect3 = Rectangle {
            width: 5,
            height: 30,
        };
        
        assert!(rect1.has_same_area(&rect2));
        assert!(!rect1.has_same_area(&rect3));
    }

    #[test]
    fn test_scale() {
        let mut rect = Rectangle {
            width: 10,
            height: 20,
        };
        
        rect.scale(3);
        
        assert_eq!(rect.width, 30);
        assert_eq!(rect.height, 60);
    }

    #[test]
    fn test_to_square() {
        let rect = Rectangle {
            width: 10,
            height: 20,
        };
        
        let square = rect.to_square();
        
        assert_eq!(square.width, 10);
        assert_eq!(square.height, 10);
    }
}