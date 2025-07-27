# Hints for Data Types - Integers and Floats

## Level 1: Conceptual Hint

Understanding Rust's numeric type system is crucial for writing safe, efficient code:

**Integer Type Ranges:**
```
i8:  -128 to 127
u8:  0 to 255  
i16: -32,768 to 32,767
u16: 0 to 65,535
i32: -2,147,483,648 to 2,147,483,647  
u32: 0 to 4,294,967,295
i64: -9,223,372,036,854,775,808 to 9,223,372,036,854,775,807
u64: 0 to 18,446,744,073,709,551,615
```

**Float Types:**
- `f32`: ~7 decimal digits of precision
- `f64`: ~15 decimal digits of precision (default)

**Type Safety Rules:**
- Rust prevents automatic type conversion
- Must explicitly cast between types: `value as target_type`
- Signed types can be negative, unsigned cannot
- Choose the smallest type that fits your data

**Overflow Behavior:**
- Debug mode: panics on overflow
- Release mode: wraps around (255u8 + 1 = 0u8)
- Use checked arithmetic for safety

📖 **Essential Reading:** [Rust Book 3.2 - Data Types](https://doc.rust-lang.org/book/ch03-02-data-types.html)

## Level 2: Strategic Hint

For each error, identify the root cause:

**Part 1: Integer Size Issues**
- `9_223_372_036_854_775_807` is too big for `i32` (max ~2 billion)
- Solution: Use `i64` for large numbers

**Part 2: Signed vs Unsigned**
- Negative numbers can't go in unsigned types (`u32`)
- `300` is too big for `u8` (max 255)

**Part 3: Float Precision**
- `f32` can't store all digits of high-precision numbers
- Use `f64` for better precision

**Part 4: Mixed Type Arithmetic**
- Can't add `i32` and `f64` directly
- Cast one type to match the other: `integer_value as f64`

**Part 5: Overflow Prevention**
- Use `checked_add()` instead of `+` to handle overflow safely
- Returns `Option<T>` (Some(result) or None if overflow)

**Part 6: String Parsing**
- Use `.parse::<type>().unwrap()` to convert strings
- Example: `"42".parse::<i32>().unwrap()`

**Strategy:** Match each value with the appropriate type based on its range and requirements.

## Level 3: Implementation Hint

Here are the specific fixes for each issue:

**Part 1: Large Integer**
```rust
let big_number: i64 = 9_223_372_036_854_775_807;  // Change i32 to i64
```

**Part 2: Unsigned Type Issues**
```rust
let population: i32 = -50_000;  // Change u32 to i32 (allows negative)
let age: u16 = 300;             // Change u8 to u16 (allows up to 65,535)
```

**Part 3: Float Precision**
```rust
// f32 loses precision - this is expected, but you might want to note it
let less_precise_pi: f64 = 3.141592653589793;  // Or keep f32 and accept loss
```

**Part 4: Type Conversion for Arithmetic**
```rust
let result = (integer_value as f64) + float_value;  // Cast i32 to f64
```

**Part 5: Safe Overflow Handling**
```rust
let overflow_result = max_u8.checked_add(1).unwrap_or(0);  // Safe addition
// Or use saturating arithmetic:
let overflow_result = max_u8.saturating_add(1);  // Caps at max value
```

**Part 6: String Parsing**
```rust
let parsed_number: i32 = number_string.parse::<i32>().unwrap();
let parsed_float: f64 = float_string.parse::<f64>().unwrap();
```

**Complete Working Code Snippet:**
```rust
fn main() {
    // Fixed integer types
    let big_number: i64 = 9_223_372_036_854_775_807;
    let population: i32 = -50_000;  
    let age: u16 = 300;
    
    // Type conversion for arithmetic
    let integer_value = 42;
    let float_value = 3.14;
    let result = (integer_value as f64) + float_value;
    
    // Safe overflow handling
    let max_u8 = 255u8;
    let overflow_result = max_u8.checked_add(1).unwrap_or(0);
    
    // String parsing
    let parsed_number: i32 = "42".parse::<i32>().unwrap();
    let parsed_float: f64 = "3.14159".parse::<f64>().unwrap();
}
```

**Key Takeaway:** Always choose the type that best fits your data's range and precision requirements!