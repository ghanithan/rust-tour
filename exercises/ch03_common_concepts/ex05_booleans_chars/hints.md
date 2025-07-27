# Hints for Boolean and Character Types

## Level 1: Conceptual Hint

**Understanding Boolean and Character Types**

**Boolean Type (`bool`):**
- Only two values: `true` and `false`
- Used for conditional logic and decision making
- Result of comparison operations (`==`, `>`, `<`, etc.)

**Logical Operators:**
- `&&` (AND): Both conditions must be true
- `||` (OR): At least one condition must be true  
- `!` (NOT): Reverses the boolean value

**Character Type (`char`):**
- Represents a Unicode scalar value
- Uses single quotes: `'a'`, `'字'`, `'🦀'`
- 4 bytes in size (not just ASCII)
- Many useful methods for classification

**Common Character Methods:**
- `.is_alphabetic()` - Is it a letter?
- `.is_numeric()` - Is it a digit?
- `.to_lowercase()` - Convert to lowercase
- `.is_alphanumeric()` - Is it letter or digit?

**Rust Book References:**
- [Boolean Type](https://doc.rust-lang.org/book/ch03-02-data-types.html#the-boolean-type)
- [Character Type](https://doc.rust-lang.org/book/ch03-02-data-types.html#the-character-type)

## Level 2: Strategic Hint

**Implementation Approaches**

**For Boolean Operations:**
```rust
let both_true = condition1 && condition2;
let at_least_one = condition1 || condition2;
let opposite = !condition1;
let in_range = value >= min && value <= max;
```

**For Character Operations:**
```rust
let is_alpha = ch.is_alphabetic();
let is_digit = ch.is_numeric();
let lowercase = ch.to_lowercase().collect::<String>();
let is_alphanumeric = ch.is_alphabetic() || ch.is_numeric();
```

**For Comparisons:**
```rust
let can_vote = age >= voting_age;
let comfortable = temperature >= 68 && temperature <= 78;
let is_not_space = ch != ' ';
```

**Pattern Examples:**
- Use `&&` when ALL conditions must be true
- Use `||` when ANY condition can be true
- Use `!` to negate/reverse a boolean
- Chain comparisons for range checks
- Combine character methods with boolean logic

## Level 3: Implementation Hint

**Complete Solutions**

**Boolean Logic Operations:**
```rust
// Logical AND operation
let both_true = is_rust_fun && is_learning;

// Logical OR operation  
let at_least_one = is_rust_fun || is_learning;

// Logical NOT operation
let not_fun = !is_rust_fun;

// Comparison operations
let can_vote = age >= voting_age;
let comfortable = temperature >= 65 && temperature <= 80;
```

**Character Assignments and Methods:**
```rust
// Character assignments
let digit_char = '5';
let space_char = ' ';

// Character method calls
let is_alpha = letter.is_alphabetic();
let is_digit = digit_char.is_numeric();
let lowercase = letter.to_lowercase().collect::<String>();

// Combined character checks
let is_alphanumeric = test_char.is_alphabetic() || test_char.is_numeric();
let is_not_space = test_char != ' ';
```

**Key Techniques:**
- Use `&&` for "both must be true" conditions
- Use `||` for "either can be true" conditions
- Use `!` to reverse a boolean value
- Use `.collect::<String>()` after `.to_lowercase()` to get a String
- Combine character methods with logical operators
- Use comparison operators (`>=`, `<=`, `==`, `!=`) to create booleans