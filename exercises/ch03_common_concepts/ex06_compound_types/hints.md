# Hints for Compound Data Types

## Level 1: Conceptual Hint

**Understanding Compound Types**

Rust has two main compound types that let you group multiple values:

1. **Tuples** - Can hold different types together: `(i32, f64, char)`
2. **Arrays** - Hold multiple values of the same type: `[i32; 5]`

**Key Differences:**
- Tuples: Different types, access by index (`tuple.0`, `tuple.1`)
- Arrays: Same type, access by index (`array[0]`, `array[1]`)
- Both are fixed size at compile time

**Destructuring** lets you extract values using patterns:
```rust
let tuple = (1, 2.0, 'a');
let (x, y, z) = tuple;  // Extract all values
let (first, ..) = tuple;  // Extract only first, ignore rest
```

**Rust Book References:**
- [Compound Types](https://doc.rust-lang.org/book/ch03-02-data-types.html#compound-types)
- [Array Type](https://doc.rust-lang.org/book/ch03-02-data-types.html#the-array-type)

## Level 2: Strategic Hint

**Implementation Approaches**

**For Tuple Operations:**
- Access elements: `tuple.0`, `tuple.1`, `tuple.2`, etc.
- Destructuring syntax: `let (a, b, c) = tuple;`
- Partial destructuring: `let (first, ..) = tuple;`
- Nested destructuring: `let ((a, b), (c, d)) = nested_tuple;`

**For Array Operations:**
- Array creation: `[1, 2, 3]` or `[0; 5]` (5 zeros)
- Element access: `array[index]`
- Length: `array.len()`
- Iteration: `array.iter().enumerate()`
- Slicing: `&array[start..end]`

**For Function Returns:**
- Single value: `fn func() -> i32`
- Tuple return: `fn func() -> (i32, i32) { (val1, val2) }`

**Common Patterns:**
```rust
// Array initialization
let zeros = [0; 5];           // [0, 0, 0, 0, 0]
let values = [1, 2, 3, 4, 5]; // Explicit values

// Iteration with index
for (i, value) in array.iter().enumerate() {
    println!("Index {}: {}", i, value);
}

// Sum calculation
let sum: i32 = array.iter().sum();
```

## Level 3: Implementation Hint

**Complete Solutions**

**Tuple Access and Destructuring:**
```rust
// Tuple element access
let second_element = person.1;
let third_element = person.2;

// Partial destructuring (ignore rest)
let (number, pi, ..) = mixed_tuple;

// Nested tuple access
let first_tuple = nested.0;
```

**Array Operations:**
```rust
// Array element access
let last = numbers[numbers.len() - 1];

// Array initialization
let zeros: [i32; 5] = [0; 5];
let threes = [3; 4];

// Array length
let length = fruits.len();

// Array modification
scores[1] = 92;
scores[2] = 78;

// Array slicing
let middle_slice = &slice_array[1..4];

// Sum calculation
let total: i32 = grades.iter().sum();
```

**Function Implementations:**
```rust
fn calculate_rectangle_area(width: i32, height: i32) -> i32 {
    width * height
}

fn calculate_rectangle_stats(width: i32, height: i32) -> (i32, i32) {
    let area = width * height;
    let perimeter = 2 * (width + height);
    (area, perimeter)
}
```

**Key Techniques:**
- Use `.iter().sum()` for summing array elements
- Use `&array[start..end]` for slicing
- Use tuple return `(val1, val2)` for multiple return values
- Use `.len()` for array length
- Use destructuring to extract tuple values cleanly