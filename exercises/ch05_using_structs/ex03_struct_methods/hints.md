# Hints for Struct Methods

## Level 1: Conceptual Hint

Methods in Rust are functions that are defined within the context of a struct (or enum or trait object) using `impl` blocks. They provide a way to organize functionality that operates on a particular type.

**Key Concepts:**
- **impl blocks**: Where you define methods for a struct
- **self parameter**: Refers to the instance the method is called on
- **Method call syntax**: `instance.method()` is syntactic sugar for `Type::method(&instance)`
- **Ownership in methods**: Methods can borrow or take ownership of `self`

**Different forms of self:**
- `&self`: Immutable borrow - read-only access to the instance
- `&mut self`: Mutable borrow - can modify the instance
- `self`: Takes ownership - consumes the instance

Rust Book Reference: [Chapter 5.3 - Method Syntax](https://doc.rust-lang.org/book/ch05-03-method-syntax.html)

Methods make code more organized and readable by grouping related functionality with the data it operates on. They also enable method chaining and make APIs more intuitive.

## Level 2: Strategic Hint

Here's how to implement methods for the Rectangle struct:

**Basic Method Structure:**
```rust
impl Rectangle {
    fn method_name(&self) -> ReturnType {
        // Access fields with self.field_name
        // Return calculated value
    }
}
```

**Guidelines for each method:**

1. **area** - Takes `&self`, returns `u32`
   - Calculate: `self.width * self.height`

2. **perimeter** - Takes `&self`, returns `u32`
   - Calculate: `2 * (self.width + self.height)`

3. **can_hold** - Takes `&self` and `&Rectangle`, returns `bool`
   - Check if both dimensions are greater than or equal

4. **has_same_area** - Takes `&self` and `&Rectangle`, returns `bool`
   - Compare areas using the area method

5. **scale** - Takes `&mut self` and `u32`
   - Modify `self.width` and `self.height` directly

6. **to_square** - Takes `self`, returns `Rectangle`
   - Find minimum dimension and create new Rectangle

**Self Parameter Decision Guide:**
- Use `&self` when you only need to read data
- Use `&mut self` when you need to modify the instance
- Use `self` when you want to consume/transform the instance

## Level 3: Implementation Hint

Here are the complete method implementations:

```rust
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    
    fn perimeter(&self) -> u32 {
        2 * (self.width + self.height)
    }
    
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width >= other.width && self.height >= other.height
    }
    
    fn has_same_area(&self, other: &Rectangle) -> bool {
        self.area() == other.area()
    }
    
    fn scale(&mut self, factor: u32) {
        self.width *= factor;
        self.height *= factor;
    }
    
    fn to_square(self) -> Rectangle {
        let side = std::cmp::min(self.width, self.height);
        Rectangle {
            width: side,
            height: side,
        }
    }
}
```

**Explanation of each implementation:**

1. **area**: Simple calculation using field access with `self.width` and `self.height`

2. **perimeter**: Standard perimeter formula, accessing fields through `self`

3. **can_hold**: Compares both dimensions using `&&` (logical AND). Both width and height must be greater than or equal.

4. **has_same_area**: Calls the `area` method on both rectangles and compares. Note how we can call methods on `self` and other instances.

5. **scale**: Takes `&mut self` because it modifies the instance. Uses compound assignment operators (`*=`) to modify fields in place.

6. **to_square**: Takes ownership (`self`) because it consumes the original rectangle to create a new one. Uses `std::cmp::min` to find the smaller dimension.

**Key Points:**
- Methods have access to all fields of the struct through `self`
- You can call other methods from within methods
- The borrow checker ensures safe access patterns
- Method call syntax (`rect.area()`) is more readable than function call syntax (`Rectangle::area(&rect)`)