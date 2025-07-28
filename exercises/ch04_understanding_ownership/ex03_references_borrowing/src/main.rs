// Learn to use references to avoid unnecessary moves

fn main() {
    // Part 1: Basic reference usage
    let s = String::from("Hello, Rust!");
    
    // TODO: Pass a reference to calculate_length instead of moving s
    let len = calculate_length(/* TODO: pass reference here */);
    
    println!("Original string: {}", s);  // s should still be usable!
    println!("String length: {}", len);
    
    // Part 2: Multiple references
    let text = String::from("Hello, Rust!");
    
    // TODO: Pass a reference to first_word
    let word = first_word(/* TODO: pass reference here */);
    
    println!("First word: {}", word);
    println!("Still available: {}", text);  // text is still usable
    
    // Part 3: Struct with references
    let title = String::from("The Rust Programming Language");
    let author = String::from("Steve Klabnik");
    
    // TODO: Create a Book using references
    let book = Book {
        title: /* TODO: use reference */,
        author: /* TODO: use reference */,
    };
    
    print_book_info(/* TODO: pass reference to book */);
    
    // TODO: Pass references to analyze_book
    let analysis = analyze_book(/* TODO: pass reference to title */, /* TODO: pass reference to author */);
    println!("Analysis: {}", analysis);
    
    println!("Book title is still available: {}", title);
}

// TODO: Change parameter to take a reference instead of owned String
fn calculate_length(/* TODO: parameter should be &String */) -> usize {
    s.len()  // TODO: s is now a reference, use it appropriately
}

// TODO: Change parameter to take a reference
fn first_word(/* TODO: parameter should be &String */) -> &str {
    let bytes = s.as_bytes();
    
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    
    &s[..]
}

// TODO: Define a struct that holds references to strings
struct Book /* TODO: add lifetime parameter */ {
    title: /* TODO: should be &str */,
    author: /* TODO: should be &str */,
}

// TODO: Change parameter to take a reference to Book
fn print_book_info(/* TODO: should take &Book */) {
    println!("Book info: {} by {}", book.title, book.author);
}

// TODO: Change parameters to take references
fn analyze_book(/* TODO: &str */, /* TODO: &str */) -> String {
    format!("Title '{}' has {} characters", title, title.len())
}