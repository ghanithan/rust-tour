# Exercise 4: Data Types - Integers and Floats

## Learning Objectives

Master Rust's numeric data types:
- Understanding integer types: i8, i16, i32, i64, u8, u16, u32, u64
- Working with floating point types: f32, f64  
- Handling type conversions and arithmetic
- Preventing integer overflow
- Parsing strings to numbers

## Background

Rust provides explicit numeric types to give you control over memory usage and prevent common bugs:

**Integer Types:**
- **Signed**: `i8`, `i16`, `i32`, `i64` (can be negative)
- **Unsigned**: `u8`, `u16`, `u32`, `u64` (positive only, larger max values)
- **Architecture-dependent**: `isize`, `usize` (match pointer size)

**Float Types:**
- `f32`: 32-bit floating point (less precision, smaller memory)
- `f64`: 64-bit floating point (more precision, default for literals)

**Key Principles:**
- Choose the right size for your data
- Understand overflow behavior
- Handle type conversions explicitly

## Rust Book References

Essential reading:
- [Chapter 3.2: Data Types](https://doc.rust-lang.org/book/ch03-02-data-types.html) - Core concepts
- [Appendix B: Operators and Symbols](https://doc.rust-lang.org/book/appendix-02-operators.html) - Type casting

## Your Task

Fix the compilation errors in `src/main.rs` by:

1. **Choosing appropriate integer types** for values that don't fit
2. **Using signed vs unsigned** types correctly
3. **Handling type conversions** for arithmetic operations
4. **Using safe arithmetic** to prevent overflow
5. **Parsing strings** to the correct numeric types

## Common Issues You'll Fix

- **Integer overflow**: Values too large for the type
- **Negative values in unsigned types**: Can't store negative in `u32`
- **Mixed type arithmetic**: Can't add `i32` + `f64` directly
- **Precision loss**: Using `f32` for high-precision values
- **String parsing**: Converting text to numbers

## Testing

```bash
cargo run        # Should work after fixing
cargo test       # Run automated tests
```

## Expected Behavior

After fixing:
- All numbers should fit in their chosen types
- No overflow or underflow errors
- Type conversions should be explicit and safe
- String parsing should work correctly

## Success Criteria

- Code compiles without errors or warnings
- All numeric values are stored in appropriate types
- Arithmetic operations use compatible types
- No integer overflow in release mode