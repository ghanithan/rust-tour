use ex07_rectangle_program::*;

fn main() {
    // TODO: Create a rectangle with width 30 and height 50
    
    // TODO: Print the rectangle using debug formatting {:?}
    
    // TODO: Print the area using the area method
    
    // TODO: Create a second rectangle with width 10 and height 40
    
    // TODO: Test if the first rectangle can hold the second
    
    // TODO: Create a third rectangle with width 60 and height 25
    
    // TODO: Test if the first rectangle can hold the third
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
}