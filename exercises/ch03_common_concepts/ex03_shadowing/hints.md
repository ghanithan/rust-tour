# Hints for Variable Shadowing

## Level 1: Conceptual Hint

Variable shadowing is about creating **new variables** with the same name:

**What Shadowing Does:**
- Creates a completely new variable
- Can change the type of the data
- Previous variable becomes inaccessible
- Each `let` statement creates a new binding

**Shadowing vs Mutation:**
```rust
// Shadowing - new variables
let x = 5;
let x = x + 1;     // New variable x shadows the old one

// Mutation - same variable  
let mut y = 5;
y = y + 1;         // Modify existing variable y
```

**Type Conversion through Shadowing:**
```rust
let data = "42";                              // &str
let data = data.parse::<i32>().unwrap();      // i32 
let data = data as f64 * 1.5;                 // f64
let data = data.to_string();                  // String
```

**Scope-based Shadowing:**
- Inner scopes can shadow outer scope variables
- When inner scope ends, outer variable becomes accessible again

📖 **Essential Reading:** [Rust Book 3.1 - Variables and Mutability (Shadowing section)](https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html#shadowing)

## Level 2: Strategic Hint

For each TODO, understand what transformation you need:

**Part 1: Area Calculation**
- Calculate π × radius²
- Use `3.14159 * (radius as f64) * (radius as f64)`
- Need to cast radius to f64 for floating point math
- Shadow `radius` with the result

**Part 2: String Processing**
- First: Use `.trim()` to remove whitespace
- Second: Use `.parse::<i32>().unwrap()` to convert to number
- Each step shadows the previous variable

**Part 3: Multiple Type Changes**
- String → i32: `.parse::<i32>().unwrap()`
- i32 → f64: `(data as f64) * 1.5`
- f64 → String: `.to_string()`

**Part 4: Scope Shadowing**
- Inside `{}` block, create new variable with same name
- Use `let message = "inner scope";`

**Function Parameter Shadowing**
- Inside function, shadow parameter with `let value = value * 2;`

**Remember:** Each `let` creates a new variable, even with the same name!

## Level 3: Implementation Hint

Here are the exact solutions for each TODO:

**Part 1: Area Calculation**
```rust
let radius = 3.14159 * (radius as f64) * (radius as f64);
```

**Part 2: String Processing**
```rust
// First TODO:
let user_input = user_input.trim();

// Second TODO:  
let user_input = user_input.parse::<i32>().unwrap();
```

**Part 3: Type Conversions**
```rust
// Convert to integer:
let data = data.parse::<i32>().unwrap();

// Convert to float:
let data = (data as f64) * 1.5;

// Convert to string:
let data = data.to_string();
```

**Part 4: Scope Shadowing**
```rust
{
    let message = "inner scope";  // Shadows outer message
    println!("Inner message: {}", message);
}
```

**Function Parameter Shadowing**
```rust
fn demonstrate_parameter_shadowing(value: i32) {
    println!("Parameter value: {}", value);
    
    let value = value * 2;  // Shadow the parameter
    println!("Shadowed value: {}", value);
}
```

**Complete Expected Output:**
```
The value of x is: 5
The value of x is: 6
Circle with radius: 3
Circle area: 28.27
Raw input: '  42  '
Trimmed input: '42'
Parsed number: 42
Original data: 123 (type: &str)
As integer: 123 (type: i32)
As float: 184.5 (type: f64)
Back to string: 184.5 (type: String)
Outer message: outer scope
Inner message: inner scope
Back to outer message: outer scope
```

**Key Insight:** Shadowing is perfect for data transformation pipelines where you want to keep the same logical name but change the type or value!