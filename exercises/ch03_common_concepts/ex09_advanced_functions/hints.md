# Hints for Advanced Function Concepts

## Level 1: Conceptual Hint

### Understanding Statements vs Expressions

In Rust, functions can return values in two ways:
- **Statements**: Use the `return` keyword followed by a semicolon
- **Expressions**: The last line without a semicolon becomes the return value

**Key Concept**: Expressions are Rust's preferred way because they're more concise and functional in style.

### Early Returns Pattern

Early returns help avoid deeply nested `if-else` chains by handling special cases first and returning immediately. This makes code more readable and follows the "fail fast" principle.

### Option<T> Return Type

The `Option<T>` enum represents a value that might or might not exist:
- `Some(value)` when the operation succeeds
- `None` when the operation cannot produce a valid result

### Mutable References

When a function needs to modify a parameter, use `&mut T` instead of `T`. Inside the function, use `*parameter` to dereference and modify the value.

### Recursion Fundamentals

Recursive functions must have:
1. **Base case**: A condition that stops the recursion
2. **Recursive case**: The function calling itself with modified parameters
3. **Progress**: Each recursive call should move closer to the base case

### Function Composition

Function composition means using the output of one function as the input to another. This creates a pipeline of transformations: `input → function1 → function2 → function3 → output`.

**Related Reading**: 
- [Rust Book Chapter 3.3 - Functions](https://doc.rust-lang.org/book/ch03-03-how-functions-work.html)
- [Rust Book Chapter 3.5 - Control Flow](https://doc.rust-lang.org/book/ch03-05-control-flow.html)

## Level 2: Strategic Hint

### Implementation Strategies

**Expression-based Function**:
```rust
fn add_with_expression(a: i32, b: i32) -> i32 {
    a + b  // No semicolon = expression that returns the value
}
```

**Early Returns Pattern**:
```rust
fn check_status(num: i32) -> &'static str {
    if condition1 {
        return "result1";  // Early return
    }
    if condition2 {
        return "result2";  // Early return
    }
    "default_result"  // Final return as expression
}
```

**Option Returns**:
```rust
fn safe_operation(input: i32) -> Option<ReturnType> {
    if input_is_invalid {
        None
    } else {
        Some(calculated_result)
    }
}
```

**Mutable References**:
```rust
fn modify_value(param: &mut i32) {
    *param = new_value;  // Dereference with * to modify
}
```

**Recursion Pattern**:
```rust
fn recursive_function(n: u32) -> u32 {
    if n <= base_case_value {
        base_case_result
    } else {
        n * recursive_function(n - 1)  // Recursive call
    }
}
```

**Working with Slices**:
```rust
// Use iterator methods for slice operations
let min = *slice.iter().min().unwrap();
let max = *slice.iter().max().unwrap();
let sum: i32 = slice.iter().sum();
```

### Function Composition Approach

Instead of storing intermediate values, you can chain function calls:
```rust
let result = function3(function2(function1(input)));
```

This is equivalent to:
```rust
let step1 = function1(input);
let step2 = function2(step1);
let result = function3(step2);
```

## Level 3: Implementation Hint

### Complete Solutions with Explanations

**Expression-based Addition**:
```rust
fn add_with_expression(a: i32, b: i32) -> i32 {
    a + b  // Expression: no semicolon, value automatically returned
}
```

**Early Returns for Number Classification**:
```rust
fn check_number_status(num: i32) -> &'static str {
    if num < 0 {
        return "negative";  // Handle negative numbers first
    }
    
    if num == 0 {
        return "zero";      // Handle zero case
    }
    
    if num <= 10 {
        return "small positive";  // Handle small positives
    }
    
    "large positive"  // Default case for remaining numbers
}
```

**Option Returns with Validation**:
```rust
fn find_square_root(num: i32) -> Option<f64> {
    if num > 100 {
        None  // Return None for numbers too large
    } else {
        Some((num as f64).sqrt())  // Cast to f64 and calculate sqrt
    }
}
```

**Mutable Reference Operations**:
```rust
fn increment_counter(counter: &mut i32) {
    *counter += 1;  // Dereference and increment
}

fn double_counter(counter: &mut i32) {
    *counter *= 2;  // Dereference and multiply
}
```

**Complex Return Types (Tuples)**:
```rust
fn analyze_grades(grades: &[i32]) -> (i32, i32, f64) {
    let min = *grades.iter().min().unwrap();  // Find minimum, dereference
    let max = *grades.iter().max().unwrap();  // Find maximum, dereference
    let sum: i32 = grades.iter().sum();       // Sum all grades
    let avg = sum as f64 / grades.len() as f64;  // Calculate average
    
    (min, max, avg)  // Return tuple
}
```

**Recursive Power Function**:
```rust
fn power(base: i32, exponent: u32) -> i32 {
    if exponent == 0 {
        1  // Base case: anything to power 0 is 1
    } else {
        base * power(base, exponent - 1)  // Recursive case
    }
}
```

**Recursive Factorial Function**:
```rust
fn factorial(n: u32) -> u32 {
    if n <= 1 {
        1  // Base case: 0! = 1! = 1
    } else {
        n * factorial(n - 1)  // Recursive case: n! = n * (n-1)!
    }
}
```

**Result Combination**:
```rust
fn combine_results(power_result: i32, factorial_result: u32) -> i32 {
    (power_result + factorial_result as i32) / 2  // Cast u32 to i32 and calculate
}
```

**Simple Composition Functions**:
```rust
fn double_number(n: i32) -> i32 {
    n * 2  // Simple multiplication
}

fn add_ten(n: i32) -> i32 {
    n + 10  // Simple addition
}

fn square_number(n: i32) -> i32 {
    n * n  // Multiply by itself
}
```

### Key Points to Remember

1. **Expressions vs Statements**: Expressions don't end with semicolons and return values
2. **Early Returns**: Use `return` keyword for early exits, expression for final return
3. **Option Handling**: Use `Some(value)` for success, `None` for failure cases
4. **Mutable References**: Always dereference with `*` when modifying values
5. **Recursion**: Always include a base case and ensure progress toward that case
6. **Type Casting**: Use `as` keyword to convert between numeric types
7. **Iterator Methods**: Use `.min()`, `.max()`, `.sum()` for slice operations
8. **Composition**: Chain functions by passing output of one as input to another

### Common Mistakes to Avoid

- Adding semicolons to expressions you want to return
- Forgetting to dereference mutable references with `*`
- Missing base cases in recursive functions
- Forgetting to handle the `None` case when working with Options
- Type mismatches when combining different numeric types