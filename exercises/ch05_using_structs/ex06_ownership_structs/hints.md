# Hints for Ownership Patterns with Structs

## Level 1: Conceptual Hint

Understanding ownership with structs is crucial in Rust. When you access struct fields, you need to consider whether you're moving, borrowing, or cloning data. Key concepts:

- **Partial moves**: When you move a field out of a struct, the entire struct becomes partially moved
- **Borrowing vs. moving**: Use references (`&`) to borrow data without taking ownership
- **Clone trait**: Sometimes you need to explicitly clone data to avoid ownership conflicts
- **Function parameters**: Consider whether functions should take ownership or borrow references

**Rust Book References:**
- Chapter 4.1: [Ownership and Functions](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html#ownership-and-functions)
- Chapter 4.2: [References and Borrowing](https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html)
- Chapter 5.1: [Defining and Instantiating Structs](https://doc.rust-lang.org/book/ch05-01-defining-structs.html)

## Level 2: Strategic Hint

The bugs in this exercise follow common ownership patterns:

1. **Partial move issue**: `let first_book = library.books[0];` moves the book out of the vector, making the entire vector unusable. Solution approaches:
   - Use `.clone()` to create a copy
   - Use `&library.books[0]` to borrow a reference

2. **Function ownership**: `display_library_info(lib: Library)` takes ownership when it should borrow:
   - Change parameter to `lib: &Library`
   - Update function body to work with references

3. **Moving within moved values**: Once you move a value, you can't access its fields:
   - Clone before moving if you need the original
   - Use references instead of moving

4. **Return type mismatches**: Returning owned data from borrowed context:
   - Return `Option<&Book>` instead of `Book`
   - Or implement a cloning version that returns `Option<Book>`

**Pattern to remember**: When in doubt, try borrowing first (`&`), then clone (`.clone()`) if ownership is needed.

## Level 3: Implementation Hint

Here are the specific fixes needed:

**Fix 1 - Partial move:**
```rust
// Instead of: let first_book = library.books[0];
let first_book = library.books[0].clone();
// Or: let first_book = &library.books[0];
```

**Fix 2 - Function parameter:**
```rust
// Change from: fn display_library_info(lib: Library)
fn display_library_info(lib: &Library) {
    println!("Library: {}", lib.name);
    println!("Total books: {}", lib.books.len());
    for book in &lib.books { // Borrow each book
        println!("- {} by {}", book.title, book.author);
    }
}
```

**Fix 3 - Call with reference:**
```rust
// Change from: display_library_info(library);
display_library_info(&library);
```

**Fix 4 - Clone or use existing cloned value:**
```rust
// If first_book is cloned, this works:
println!("Title: {}", first_book.title);
// Or clone the field: let book_title = first_book.title.clone();
```

**Fix 5 - Use references or clone:**
```rust
// Instead of moving twice:
let another_book = &library.books[1]; // Borrow
let yet_another = library.books[1].clone(); // Clone
```

**Fix 6 - Return reference:**
```rust
fn find_book_by_title(library: &Library, title: &str) -> Option<&Book> {
    for book in &library.books {
        if book.title == title {
            return Some(book); // Return reference
        }
    }
    None
}
```

**Don't forget**: Add `#[derive(Clone)]` to the `Book` struct to enable cloning!

**Complete working pattern:**
```rust
#[derive(Debug, Clone)]
struct Book { /* fields */ }

// Use references for borrowing, .clone() when ownership is needed
// Functions should generally take references unless they need ownership
// Return references when possible, Option<T> for safer error handling
```