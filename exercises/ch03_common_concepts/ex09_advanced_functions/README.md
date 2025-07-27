# Exercise 9: Advanced Function Concepts

## Learning Objectives

By completing this exercise, you will:

- Master the difference between statements and expressions in Rust functions
- Implement early returns for cleaner conditional logic
- Work with mutable references to modify function parameters
- Use `Option<T>` as a return type for fallible operations
- Build recursive functions with proper base cases
- Practice function composition and chaining
- Understand how Rust's expression-based nature enables elegant code

## Prerequisites

Before starting this exercise, ensure you understand:
- Basic function syntax and parameters (Chapter 3.3)
- Control flow with `if` statements (Chapter 3.5)
- The concept of ownership and borrowing (Chapter 4.2)

## Exercise Description

This intermediate-level exercise explores advanced function concepts in Rust. You'll implement various functions that demonstrate different programming patterns:

1. **Statements vs Expressions**: Compare explicit `return` statements with expression-based returns
2. **Early Returns**: Use early returns to handle different conditions efficiently
3. **Option Returns**: Return `Some(value)` or `None` based on input validation
4. **Mutable References**: Modify values through mutable references
5. **Complex Return Types**: Return tuples containing multiple calculated values
6. **Recursion**: Implement recursive algorithms with proper base cases
7. **Function Composition**: Chain function calls for data transformation

## Key Concepts Covered

- **Statements vs Expressions**: Understanding when to use `return` statements versus expression-based returns
- **Early Returns**: Simplifying complex conditional logic with early returns
- **Mutable References**: Using `&mut` parameters to modify caller's data
- **Option Returns**: Handling cases where functions might not produce valid results
- **Recursion**: Building functions that call themselves with proper termination conditions
- **Function Composition**: Combining simple functions to create complex transformations

## Instructions

1. **Read the Code**: Examine `src/main.rs` to understand the structure and expected behavior
2. **Complete the TODOs**: Implement each function following the comments and guidelines
3. **Test Frequently**: Run `cargo test` to verify your implementations
4. **Run the Program**: Use `cargo run` to see the complete output
5. **Experiment**: Try different approaches and understand why they work

## Expected Output

When completed correctly, your program should output:

```
=== Advanced Function Concepts ===

--- Expression vs Statement ---
Statement-based addition: 8
Expression-based addition: 8

--- Early Returns ---
Number 10: small positive
Number -5: negative
Number 15: large positive
Number 0: zero
Number -3: negative
Number 20: large positive

--- Functions with Option Returns ---
√10 = 3.16
√25 = 5.00
√100 = 10.00
150 is too large for this function

--- Parameter Modification ---
Initial counter: 0
After increment: 1
After doubling: 2

--- Complex Return Types ---
Grades: [85, 92, 78, 96, 88]
Min: 78, Max: 96, Average: 87.8

--- Nested Function Calls ---
3^4 = 81
5! = 120
Combined calculation: 100

--- Function Composition ---
Input: 5 → Double: 10 → Add 10: 20 → Square: 400
Composed result: 400
```

## Functions to Implement

### 1. Expression-based Functions
- `add_with_expression`: Return sum without using `return` keyword

### 2. Early Returns
- `check_number_status`: Classify numbers using early returns

### 3. Option Returns
- `find_square_root`: Return square root for numbers ≤ 100, None otherwise

### 4. Mutable References
- `increment_counter`: Add 1 to a mutable reference
- `double_counter`: Multiply a mutable reference by 2

### 5. Complex Returns
- `analyze_grades`: Return tuple of (min, max, average) from a slice

### 6. Recursive Functions
- `power`: Calculate base^exponent recursively
- `factorial`: Calculate n! recursively

### 7. Composition Functions
- `double_number`: Multiply by 2
- `add_ten`: Add 10
- `square_number`: Calculate n²
- `combine_results`: Combine power and factorial results

## Testing

Run the tests to verify your implementation:

```bash
cargo test
```

The tests verify:
- Program compiles and runs successfully
- Expressions vs statements work correctly
- Early returns classify numbers properly
- Option returns handle valid/invalid cases
- Mutable references modify values correctly
- Recursive functions produce correct results
- Function composition works as expected

## Hints Available

This exercise includes a progressive hint system:
- **Level 1**: Conceptual understanding and theory
- **Level 2**: Implementation strategies and approaches
- **Level 3**: Complete solutions with explanations

Use hints if you get stuck, but try to solve each part independently first!

## Time Estimate

**30 minutes** - This exercise covers multiple advanced concepts and may take time to fully understand and implement correctly.

## Related Reading

- [The Rust Programming Language - Chapter 3.3: Functions](https://doc.rust-lang.org/book/ch03-03-how-functions-work.html)
- [The Rust Programming Language - Chapter 3.5: Control Flow](https://doc.rust-lang.org/book/ch03-05-control-flow.html)
- [The Rust Programming Language - Chapter 4.2: References and Borrowing](https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html)