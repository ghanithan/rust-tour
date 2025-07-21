# Hints for Variables and Mutability

## Level 1: Conceptual Hint

This exercise is about understanding Rust's approach to **variable safety**. Rust prevents many common bugs by making variables immutable by default.

**Key concepts you need to understand:**

### Immutability by Default
- Variables in Rust are **immutable by default**
- This means once you assign a value, you can't change it
- This prevents accidental mutations that cause bugs

### The `mut` Keyword
- To make a variable changeable, use the `mut` keyword
- `let mut x = 5;` creates a mutable variable
- `let x = 5;` creates an immutable variable

### Shadowing vs Mutation
- **Shadowing**: Creating a new variable with the same name (`let x = x + 1;`)
- **Mutation**: Changing the value of a mutable variable (`x = x + 1;`)
- Shadowing can change the type, mutation cannot

### Constants
- Declared with `const` keyword
- Must have type annotations
- Must be ALL_CAPS with underscores
- Always immutable (no `mut` allowed)

**📖 Read:** [Rust Book 3.1 - Variables and Mutability](https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html)

**💭 Think about:** Why would making variables immutable by default help prevent bugs?

## Level 2: Strategic Hint

Look at each compilation error and understand what Rust is telling you:

### Section 1: Basic Mutability Error
```rust
let x = 5;
x = 6;  // Error: cannot assign twice to immutable variable
```
**Fix:** Add `mut` to make the variable mutable.

### Section 2: Shadowing Issue  
```rust
let spaces = "   ";
spaces = spaces.len();  // Error: mismatched types + immutability
```
**Fix:** Use `let` again to shadow the variable with a new type.

### Section 3: Constant Declaration
```rust
let max_points = 100_000;  // Wrong: should be const
```
**Fix:** Use `const` with proper naming and type annotation.

### Section 4: Counter Mutations
```rust
let counter = 0;
counter = counter + 1;  // Error: immutable variable
```
**Fix:** Make `counter` mutable.

### Section 5: Mixed Shadowing and Mutation
Look carefully at when you need shadowing vs when you need mutation.

### Module-Level Constant
Constants belong at the top of the file, not inside functions.

**Strategy:**
1. Read each error message carefully
2. Decide: Do I need mutation or shadowing?
3. Apply the appropriate fix
4. Test with `cargo check`

## Level 3: Implementation Hint

Here are the specific fixes for each section:

### Section 1: Add `mut`
```rust
let mut x = 5;  // Add mut keyword
println!("The value of x is: {}", x);
x = 6;  // Now this works
```

### Section 2: Use Shadowing
```rust
let spaces = "   ";
let spaces = spaces.len();  // Shadow with new type and value
```

### Section 3: Fix Constant
```rust
const MAX_POINTS: u32 = 100_000;  // Use const, proper name, type annotation
```

### Section 4: Make Counter Mutable
```rust
let mut counter = 0;  // Add mut
// Now the mutations will work
counter = counter + 1;
counter = counter + 1;
```

### Section 5: Remove `mut` from Shadowing
```rust
let value = "42";
let value = value.parse::<i32>().expect("Not a number!");  // Remove mut from let
// But keep it mutable for the next operation:
let mut value = value;  // Shadow again to make it mutable
value = value * 2;
```

Or more simply:
```rust
let mut value = value.parse::<i32>().expect("Not a number!");  // Just add mut here
```

### Move Constant to Top
Move this line to the top of the file (outside main):
```rust
const SECONDS_IN_MINUTE: u32 = 60;  // Also fix the type to u32
```

### Complete Fixed Code Structure:
```rust
const SECONDS_IN_MINUTE: u32 = 60;

fn main() {
    // Section 1: let mut x = 5;
    // Section 2: let spaces = spaces.len();
    // Section 3: const MAX_POINTS: u32 = 100_000;
    // Section 4: let mut counter = 0;
    // Section 5: proper shadowing/mutation
}
```

After making these changes, `cargo check` should pass!