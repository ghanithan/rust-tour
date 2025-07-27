# Hints for Move Semantics Deep Dive

## Level 1: Conceptual Hint

Let's think about what's happening with ownership in this exercise:

**Vec and Ownership**: A `Vec<String>` owns all the strings it contains. When you push a String into a Vec, the Vec takes ownership. When you remove a String from a Vec, you get ownership back.

**Key Concepts**:
- `Vec::push()` takes ownership of the value you're adding
- `Vec::pop()` returns ownership via `Option<T>`
- When iterating to consume a Vec, you need to move values out

**Mental Model**: Think of the StringBuffer as a temporary holding area. Strings check in (lose their ownership to the buffer) and check out (regain ownership when removed).

**Questions to Consider**:
1. What type should the field in StringBuffer be?
2. How does `Vec::pop()` transfer ownership?
3. How can you move all strings out of a Vec to combine them?

The Rust Book section on Vectors (8.1) will help understand how collections handle ownership.

## Level 2: Strategic Hint

Here's the approach for each method:

**Structure**:
```rust
struct StringBuffer {
    strings: Vec<String>  // Vec owns all the strings
}
```

**Method Strategies**:

1. **new()**: Create an empty Vec
   - Pattern: `Vec::new()` or `vec![]`

2. **add()**: Take ownership and store
   - Pattern: `self.strings.push(text)`
   - The parameter `text: String` means we take ownership

3. **remove_last()**: Transfer ownership back
   - Pattern: `self.strings.pop()`
   - Returns `Option<String>` - Some(string) or None

4. **flush()**: Move all strings out and combine
   - Challenge: How to move strings out of the Vec?
   - Consider: `drain()`, `into_iter()`, or swap with empty Vec
   - Pattern: Take ownership of all strings, join them

5. **len()**: Just check the Vec's length
   - Pattern: `self.strings.len()`

For flush(), think about:
- You need to move strings out (not borrow)
- The buffer should be empty afterward
- `String::new()` + iteration, or `join()`

## Level 3: Implementation Hint

Here's the complete solution:

```rust
struct StringBuffer {
    strings: Vec<String>
}

impl StringBuffer {
    fn new() -> Self {
        StringBuffer {
            strings: Vec::new()
        }
    }
    
    fn add(&mut self, text: String) {
        // Take ownership of text and store it
        self.strings.push(text);
    }
    
    fn remove_last(&mut self) -> Option<String> {
        // Pop removes and returns the last element
        self.strings.pop()
    }
    
    fn flush(&mut self) -> String {
        // Method 1: Drain all elements and collect
        let result: String = self.strings.drain(..).collect();
        result
        
        // Method 2: Swap with empty vec and iterate
        // let mut temp = Vec::new();
        // std::mem::swap(&mut self.strings, &mut temp);
        // temp.into_iter().collect()
        
        // Method 3: Manual concatenation
        // let mut result = String::new();
        // while let Some(s) = self.strings.pop() {
        //     result.insert_str(0, &s);
        // }
        // result
    }
    
    fn len(&self) -> usize {
        self.strings.len()
    }
}
```

**Key Insights**:
1. `push()` moves the String into the Vec
2. `pop()` moves the String out of the Vec
3. `drain(..)` moves all elements out, leaving Vec empty
4. `.collect::<String>()` concatenates all strings

The beauty of Rust's ownership system: it's impossible to accidentally have two references to the same String. Each String has exactly one owner at all times!