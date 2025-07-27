# Hints for References and Borrowing

## Level 1: Conceptual Hint

**What are References?**
Think of references as "read-only access cards" to data. When you have a reference, you can:
- Look at the data
- Read its contents
- Pass it to functions for inspection
- BUT you cannot take ownership or modify it (unless it's a mutable reference)

**The Borrowing Metaphor**:
- Someone owns a book (the owner variable)
- You borrow the book to read it (create a reference)
- You can read the book but must give it back
- The owner still has their book when you're done

**Key Syntax**:
- `&value` creates a reference to value
- `&Type` is the type of an immutable reference
- Functions that take references don't take ownership

**Why Use References?**
References solve the ownership transfer problem. Instead of moving values to functions (and losing access), you can lend temporary access while keeping ownership.

Read Rust Book section 4.2 to understand how references prevent the need for ownership transfers.

## Level 2: Strategic Hint

**Step-by-step approach**:

1. **Function Parameters**: Change from `String` to `&String` or `&str`
   ```rust
   // Instead of:
   fn my_function(s: String) -> usize
   
   // Use:
   fn my_function(s: &String) -> usize
   // or even better:
   fn my_function(s: &str) -> usize
   ```

2. **Function Calls**: Add `&` when calling
   ```rust
   // Instead of:
   let result = my_function(my_string);
   
   // Use:
   let result = my_function(&my_string);
   ```

3. **Struct Fields**: References in structs need lifetime parameters
   ```rust
   struct Book<'a> {
       title: &'a str,
       author: &'a str,
   }
   ```

4. **Reference Usage**: References are automatically dereferenced
   ```rust
   fn len_function(s: &String) -> usize {
       s.len()  // Works! Rust auto-dereferences
   }
   ```

**Common Patterns**:
- Function parameter: `&String` or `&str`
- Function call: `function(&my_variable)`
- Struct with references: needs `<'a>` lifetime parameter

## Level 3: Implementation Hint

Here's the complete solution:

```rust
fn main() {
    // Part 1: Basic reference usage
    let s = String::from("Hello, Rust!");
    
    let len = calculate_length(&s);  // Pass reference
    
    println!("Original string: {}", s);
    println!("String length: {}", len);
    
    // Part 2: Multiple references
    let text = String::from("Hello, Rust!");
    
    let word = first_word(&text);  // Pass reference
    
    println!("First word: {}", word);
    println!("Still available: {}", text);
    
    // Part 3: Struct with references
    let title = String::from("The Rust Programming Language");
    let author = String::from("Steve Klabnik");
    
    let book = Book {
        title: &title,   // Reference to title
        author: &author, // Reference to author
    };
    
    print_book_info(&book);  // Pass reference to book
    
    let analysis = analyze_book(&title, &author);  // Pass references
    println!("Analysis: {}", analysis);
    
    println!("Book title is still available: {}", title);
}

fn calculate_length(s: &String) -> usize {  // Take reference
    s.len()  // Use reference (auto-dereferenced)
}

fn first_word(s: &String) -> &str {  // Take reference, return string slice
    let bytes = s.as_bytes();
    
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    
    &s[..]
}

struct Book<'a> {  // Lifetime parameter needed for references
    title: &'a str,
    author: &'a str,
}

fn print_book_info(book: &Book) {  // Take reference to Book
    println!("Book info: {} by {}", book.title, book.author);
}

fn analyze_book(title: &str, author: &str) -> String {  // Take references
    format!("Title '{}' has {} characters", title, title.len())
}
```

**Key Points**:
1. `&` creates references when calling functions
2. Function parameters use `&Type` to accept references
3. References are automatically dereferenced when used
4. Structs with references need lifetime parameters (`<'a>`)
5. `&str` is preferred over `&String` for function parameters

The lifetime `'a` tells Rust that the references in the Book struct live as long as the data they point to. Don't worry too much about lifetimes yet - they'll be covered in detail later!