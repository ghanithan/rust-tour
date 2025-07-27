# Hints for If/Else Control Flow

## Level 1: Conceptual Hint

**Understanding If Expressions in Rust**

In Rust, `if` is an expression, not just a statement. This means:
- Every `if` expression evaluates to a value
- You can use `if` in `let` statements for conditional assignment
- All branches must return the same type

**Basic Syntax:**
```rust
if condition {
    // code
} else if other_condition {
    // code  
} else {
    // code
}
```

**Key Points:**
- Conditions must be `bool` type (no truthiness like other languages)
- Use comparison operators: `==`, `!=`, `<`, `>`, `<=`, `>=`
- Combine with logical operators: `&&` (AND), `||` (OR), `!` (NOT)
- Braces are required even for single statements

**Rust Book References:**
- [If Expressions](https://doc.rust-lang.org/book/ch03-05-control-flow.html#if-expressions)
- [Using if in a let Statement](https://doc.rust-lang.org/book/ch03-05-control-flow.html#using-if-in-a-let-statement)

## Level 2: Strategic Hint

**Implementation Strategies**

**For Boolean Conditions:**
```rust
// Fix: condition must be boolean
if temperature > 70 { // not just `if temperature`
    
// Age comparison
if age >= 18 {

// Range checking
if score >= 80 && score < 90 {

// Complex logic
if has_license && has_car && has_gas {
```

**For Conditional Assignment:**
```rust
let activity = if weather == "sunny" {
    "go to park"
} else if weather == "rainy" {
    "read indoors"
} else {
    "watch movies"
};
```

**For Nested Conditionals:**
```rust
if time_condition {
    if day_condition {
        // nested logic
    }
}
```

**Common Patterns:**
- Use `&&` when ALL conditions must be true
- Use `||` when ANY condition can be true
- Use `!` to negate conditions
- Use `else if` for multiple exclusive conditions

## Level 3: Implementation Hint

**Complete Solutions**

**Fix Temperature Condition:**
```rust
if temperature > 70 {  // Must be boolean comparison
    println!("It's warm outside!");
} else {
    println!("It's cool outside!");
}
```

**Voting Eligibility:**
```rust
if age >= 18 {
    println!("You can vote!");
} else {
    println!("You cannot vote yet.");
}
```

**Grade Assignment:**
```rust
if score >= 90 {
    println!("Grade: A");
} else if score >= 80 {  // 80-89 range
    println!("Grade: B");
} else if score >= 70 {  // 70-79 range
    println!("Grade: C");
} else {
    println!("Grade: F");
}
```

**Conditional Assignment:**
```rust
let activity = if weather == "sunny" {
    "go to park"
} else if weather == "rainy" {
    "read indoors"
} else {
    "watch movies"
};
```

**Complex Boolean Logic:**
```rust
let can_drive = has_license && has_car && has_gas;

if !has_license {
    println!("Reason: No license");
}
if !has_car {
    println!("Reason: No car");
}
if !has_gas {
    println!("Reason: No gas");
}
```

**Nested Time Logic:**
```rust
} else if time_of_day >= 11 && time_of_day < 15 {  // lunch time
    println!("Lunch time!");
    if is_weekend {
        println!("Weekend lunch has extended hours!");
    }
} else if time_of_day >= 17 && time_of_day <= 22 {  // dinner time
    println!("Dinner time!");
```

**Key Techniques:**
- Always use boolean expressions in conditions
- Use `>=` and `<=` for inclusive ranges
- Use `&&` for multiple required conditions
- Use `!` to check for absence of conditions
- Structure else-if chains from most specific to most general