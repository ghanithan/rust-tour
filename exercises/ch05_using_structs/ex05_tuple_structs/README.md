# Exercise 5: Tuple Structs and Unit-Like Structs

## Learning Objectives
By completing this exercise, you will learn:
- How to define and use tuple structs in Rust
- When to use tuple structs vs regular structs
- The newtype pattern for type safety
- Unit-like structs and their use cases
- How to implement methods on tuple structs
- How to derive traits for struct types

## Background
Tuple structs are a hybrid between tuples and structs. They have the added meaning that a struct name provides but don't have names associated with their fields. This makes them useful when you want to give the whole tuple a name and make the tuple a different type from other tuples, and when naming each field would be verbose or redundant.

The newtype pattern is particularly useful for creating type-safe wrappers around primitive types, preventing accidental mixing of values that represent different concepts but have the same underlying type.

## Exercise Type: Bug Fixing
This exercise contains intentional bugs in the code. Your task is to identify and fix all compilation errors and logic issues to make the program work correctly.

## Instructions
1. **Examine the code** in `src/main.rs` - it contains multiple bugs marked with `// BUG:` comments
2. **Fix the struct definitions** - The current syntax for defining tuple structs is incorrect
3. **Add missing derive attributes** - Some structs need `Debug` and `PartialEq` traits
4. **Implement the methods** - Complete the `unimplemented!()` method bodies
5. **Enable comparisons** - Fix the commented-out equality comparison
6. **Run the tests** to verify your implementation works correctly

## Key Concepts to Apply

### Tuple Struct Syntax
```rust
struct StructName(Type1, Type2, Type3);
```

### Accessing Tuple Struct Fields
```rust
let instance = StructName(value1, value2, value3);
let first_field = instance.0;
let second_field = instance.1;
```

### The Newtype Pattern
```rust
struct UserId(u32);  // Wraps u32 but creates a new type
struct OrderId(u32); // Different type, can't be mixed with UserId
```

### Unit-Like Structs
```rust
struct AlwaysEqual;  // No data, useful for traits and markers
```

## Expected Behavior
When the program runs successfully, it should:
1. Create instances of Color, Point, AlwaysEqual, and UserId structs
2. Print information about each instance
3. Demonstrate type safety by preventing mixing of different types
4. Show method calls on tuple structs
5. Pass all the included tests

## Common Pitfalls
- **Incorrect syntax**: Using `=` instead of proper tuple struct syntax
- **Missing derives**: Forgetting to add `#[derive(Debug, PartialEq)]` where needed
- **Method implementation**: Not accessing tuple fields correctly with `.0`, `.1`, etc.
- **Type confusion**: Trying to compare different tuple struct types

## Testing
Run the tests with:
```bash
cargo test
```

All tests should pass when your implementation is correct. The tests verify:
- Struct creation and field access
- Method implementations
- Type safety and validation logic

## Time Estimate
This exercise should take approximately 20 minutes to complete.

## Rust Book References
- **Primary**: [Chapter 5.1 - Using Tuple Structs without Named Fields to Create Different Types](https://doc.rust-lang.org/book/ch05-01-defining-structs.html#using-tuple-structs-without-named-fields-to-create-different-types)
- **Supporting**: [Chapter 5.0 - Using Structs to Structure Related Data](https://doc.rust-lang.org/book/ch05-00-structs.html)

## Next Steps
After completing this exercise, you'll be ready to explore more advanced struct concepts including associated functions and methods in the next exercises.