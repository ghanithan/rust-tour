# Hints for Loop Constructs

## Level 1: Conceptual Hint

**Understanding Rust's Loop Types**

Rust has three types of loops, each with specific use cases:

1. **`loop`** - Infinite loops that run until explicitly broken
   - Use when you don't know how many iterations you need
   - Can return values with `break value`

2. **`while`** - Conditional loops that run while a condition is true
   - Use when you have a clear condition to check each iteration
   - Automatically stops when condition becomes false

3. **`for`** - Iterator-based loops for traversing collections/ranges
   - Use when iterating over known collections or ranges
   - Most common and idiomatic in Rust

**Loop Control:**
- `break` - Exit the loop immediately
- `continue` - Skip to next iteration
- `break value` - Exit loop and return a value

**Rust Book References:**
- [Repetition with Loops](https://doc.rust-lang.org/book/ch03-05-control-flow.html#repetition-with-loops)
- [Looping Through Collections](https://doc.rust-lang.org/book/ch03-05-control-flow.html#looping-through-a-collection-with-for)

## Level 2: Strategic Hint

**Implementation Strategies**

**For `loop` with break:**
```rust
loop {
    // do something
    if condition {
        break;
    }
}
```

**For `loop` returning value:**
```rust
let result = loop {
    // do something
    if condition {
        break computed_value;
    }
};
```

**For `while` loops:**
```rust
while condition {
    // modify the condition variable
}
```

**For `for` loops with ranges:**
```rust
for i in 1..=5 {  // inclusive range 1 to 5
    println!("{}", i);
}
```

**For `for` loops with arrays:**
```rust
for item in array.iter() {
    println!("{}", item);
}

for (index, item) in array.iter().enumerate() {
    println!("{}: {}", index, item);
}
```

**For loop control:**
```rust
for num in 1..=10 {
    if num % 2 == 0 {
        continue;  // skip even numbers
    }
    println!("{}", num);
}
```

## Level 3: Implementation Hint

**Complete Solutions**

**Infinite Loop with Break:**
```rust
loop {
    println!("Counter: {}", counter);
    counter += 1;
    
    if counter >= 3 {
        break;
    }
}
```

**Loop with Return Value:**
```rust
let result = loop {
    x += 1;
    if x == 5 {
        break x * 2;  // return 10
    }
};
```

**While Loop Countdown:**
```rust
while number > 0 {
    println!("Countdown: {}", number);
    number -= 1;
}
```

**For Loop with Range:**
```rust
for i in 1..=5 {
    println!("Number: {}", i);
}
```

**For Loop with Array:**
```rust
for fruit in fruits.iter() {
    println!("Fruit: {}", fruit);
}
```

**Nested Loops for Multiplication:**
```rust
for i in 1..=3 {
    for j in 1..=3 {
        print!("{} ", i * j);
    }
    println!(); // New line after each row
}
```

**Loop with Continue:**
```rust
for num in 1..=10 {
    if num % 2 == 0 {  // check if even
        continue;
    }
    println!("Odd number: {}", num);
}
```

**While Let Pattern:**
```rust
while let Some(value) = stack.pop() {
    println!("Popped: {}", value);
}
```

**Sum Calculation:**
```rust
let total: i32 = grades.iter().sum();
```

**Key Techniques:**
- Use `counter >= 3` for break condition
- Use `break value` to return from loops
- Use `number -= 1` to decrement in while loops
- Use `1..=5` for inclusive ranges
- Use `.iter()` to iterate over arrays
- Use `num % 2 == 0` to check for even numbers
- Use `.pop()` with while let for stack operations