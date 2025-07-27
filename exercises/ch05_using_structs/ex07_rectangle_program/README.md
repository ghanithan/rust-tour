# Rectangle Program - Practical Struct Example

## Learning Objectives

In this exercise, you'll build a complete program using structs, following the classic Rust Book rectangle example. You'll learn to:

- Define structs with appropriate field types
- Derive the `Debug` trait for convenient printing
- Implement methods using `impl` blocks
- Work with method parameters and references
- Build a practical program that demonstrates struct usage

## Your Task

Complete the rectangle program by implementing:

1. **Rectangle struct**: Define a struct with `width` and `height` fields (both `u32`)
2. **Debug trait**: Use `#[derive(Debug)]` to enable debug printing
3. **Area calculation**: Implement an `area` method that returns `u32`
4. **Comparison method**: Implement a `can_hold` method that checks if one rectangle can fit inside another
5. **Main function**: Create rectangles and test the methods

## Expected Program Output

When completed, your program should output something like:

```
rect1 is Rectangle { width: 30, height: 50 }
The area of the rectangle is 1500 square pixels.
Can rect1 hold rect2? true
Can rect1 hold rect3? false
```

## Key Concepts

- **Struct definition**: Creating custom data types
- **Method syntax**: Using `self` and `&self` parameters
- **Debug formatting**: Using `{:?}` for struct printing
- **References**: Passing structs without taking ownership

## Success Criteria

Your implementation should:
- ✅ Define a `Rectangle` struct with `width` and `height` fields
- ✅ Derive the `Debug` trait for the struct
- ✅ Implement an `area` method that calculates width × height
- ✅ Implement a `can_hold` method for rectangle comparison
- ✅ Create and test rectangles in the main function
- ✅ Pass all unit tests
- ✅ Compile without warnings

## Tips

- Remember that methods take `&self` as the first parameter
- The `can_hold` method should check if both width and height of the other rectangle are smaller
- Use `#[derive(Debug)]` above your struct definition
- In `main()`, use `{:?}` to print the rectangle with debug formatting

## Rust Book References

- [Chapter 5.2: An Example Program Using Structs](https://doc.rust-lang.org/book/ch05-02-example-structs.html)
- [Chapter 5.1: Defining and Instantiating Structs](https://doc.rust-lang.org/book/ch05-01-defining-structs.html)
- [Chapter 5.3: Method Syntax](https://doc.rust-lang.org/book/ch05-03-method-syntax.html)

**Estimated time**: 30 minutes