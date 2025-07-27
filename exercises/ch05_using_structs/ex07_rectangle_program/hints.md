# Hints for Rectangle Program - Practical Struct Example

## Level 1: Conceptual Hint

This exercise demonstrates the core concepts of structs in Rust - custom data types that group related data together and provide methods to operate on that data.

**Key Concepts to Understand:**

- **Structs** are custom data types that let you package together related values
- **Methods** are functions defined within the context of a struct using `impl` blocks
- **The Debug trait** allows structs to be printed using `{:?}` formatting
- **References** (`&self`) let methods borrow data without taking ownership

**Rust Book Connection:**
This follows the exact pattern from [Chapter 5.2](https://doc.rust-lang.org/book/ch05-02-example-structs.html), where the Rust Book walks through building a rectangle program step by step.

**What you're building:**
A `Rectangle` struct that can calculate its area and determine if it can contain another rectangle. This teaches the fundamental pattern of defining data structures and their associated behavior.

## Level 2: Strategic Hint

**Approach and Implementation Strategy:**

1. **Define the struct:**
```rust
#[derive(Debug)]
struct Rectangle {
    // Two u32 fields for dimensions
}
```

2. **Implement methods in an impl block:**
```rust
impl Rectangle {
    fn area(&self) -> u32 {
        // Multiply width by height
    }
    
    fn can_hold(&self, other: &Rectangle) -> bool {
        // Check if self can contain other
        // Both width AND height must be larger
    }
}
```

3. **In main function:**
   - Create rectangles using struct instantiation syntax: `Rectangle { width: 30, height: 50 }`
   - Use `println!("{:?}", rect)` for debug printing
   - Call methods using dot notation: `rect.area()`

**Common Patterns:**
- Methods that don't modify the struct use `&self`
- Methods that calculate values typically return the calculated type
- Comparison methods typically return `bool`

**Testing Strategy:**
The provided tests expect specific method signatures and behavior - make sure your methods match exactly.

## Level 3: Implementation Hint

**Complete Implementation Guide:**

```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("rect1 is {:?}", rect1);
    println!("The area of the rectangle is {} square pixels.", rect1.area());

    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };

    let rect3 = Rectangle {
        width: 60,
        height: 25,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}
```

**Explanation of Each Part:**

- **`#[derive(Debug)]`**: This attribute automatically implements the Debug trait, allowing the struct to be printed with `{:?}`
- **Struct fields**: Both `width` and `height` are `u32` (unsigned 32-bit integers) suitable for dimensions
- **`area` method**: Takes `&self` (immutable reference) and returns the product of width and height
- **`can_hold` method**: Takes `&self` and `&Rectangle`, returns true only if both dimensions are strictly larger
- **Method calls**: Use dot notation (`rect1.area()`) and pass references (`&rect2`) when needed

**Why This Design:**
This follows Rust's ownership principles - methods borrow the struct rather than taking ownership, allowing the struct to be used after method calls. The `can_hold` method takes a reference to avoid moving the parameter.