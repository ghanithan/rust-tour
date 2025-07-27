// Solution: Using references to borrow data without taking ownership

fn main() {
    // Part 1: Basic reference usage
    let s = String::from("Hello, Rust!");
    
    let len = calculate_length(&s);  // Pass a reference to s
    
    println!("Original string: {}", s);  // s is still usable!
    println!("String length: {}", len);
    
    // Part 2: Multiple references
    let text = String::from("Hello, Rust!");
    
    let word = first_word(&text);  // Pass a reference to text
    
    println!("First word: {}", word);
    println!("Still available: {}", text);  // text is still usable
    
    // Part 3: Struct with references
    let title = String::from("The Rust Programming Language");
    let author = String::from("Steve Klabnik");
    
    let book = Book {
        title: &title,   // Store references, not owned strings
        author: &author,
    };
    
    print_book_info(&book);  // Pass reference to book
    
    let analysis = analyze_book(&title, &author);  // Pass references to strings
    println!("Analysis: {}", analysis);
    
    println!("Book title is still available: {}", title);
}

// Take a reference to String instead of taking ownership
fn calculate_length(s: &String) -> usize {
    s.len()  // s is a reference, but Rust auto-dereferences it
}

// Take a reference and return a string slice (also a reference)
fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    
    &s[..]
}

// Struct that holds references (needs lifetime parameter)
struct Book<'a> {
    title: &'a str,   // Reference to string data
    author: &'a str,  // Reference to string data
}

// Take a reference to Book instead of taking ownership
fn print_book_info(book: &Book) {
    println!("Book info: {} by {}", book.title, book.author);
}

// Take references to string slices
fn analyze_book(title: &str, author: &str) -> String {
    format!("Title '{}' has {} characters", title, title.len())
}