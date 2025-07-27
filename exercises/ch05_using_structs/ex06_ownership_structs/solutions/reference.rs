// Fixed version: Ownership Patterns with Structs
// This demonstrates correct ownership patterns, borrowing, and cloning

#[derive(Debug, Clone)] // Added Clone trait for cloning when needed
struct Book {
    title: String,
    author: String,
    pages: u32,
    available: bool,
}

#[derive(Debug)]
struct Library {
    name: String,
    books: Vec<Book>,
}

fn main() {
    let mut library = Library {
        name: String::from("City Library"),
        books: vec![
            Book {
                title: String::from("The Rust Programming Language"),
                author: String::from("Steve Klabnik"),
                pages: 560,
                available: true,
            },
            Book {
                title: String::from("Programming Rust"),
                author: String::from("Jim Blandy"),
                pages: 624,
                available: true,
            },
        ],
    };

    println!("Welcome to {}", library.name);
    
    // FIX 1: Clone instead of moving to avoid partial move
    let first_book = library.books[0].clone();
    println!("First book: {:?}", first_book);
    
    // FIX 2: Pass reference instead of moving the entire library
    display_library_info(&library);
    
    // FIX 3: Can modify after borrowing (not moving)
    library.books[1].available = false;
    
    // FIX 4: Clone the title or use reference to avoid move within moved value
    let book_title = first_book.title.clone();
    println!("Title: {}", first_book.title); // Now works because first_book is cloned
    
    // FIX 5: Use references or clone to avoid double move
    let another_book = &library.books[1]; // Borrow instead of move
    let yet_another = library.books[1].clone(); // Clone if you need ownership
    
    println!("Books processed successfully!");
    println!("Another book (borrowed): {:?}", another_book);
    println!("Yet another book (cloned): {:?}", yet_another);
}

// FIX: Function now takes a reference instead of taking ownership
fn display_library_info(lib: &Library) {
    println!("Library: {}", lib.name);
    println!("Total books: {}", lib.books.len());
    for book in &lib.books { // Borrow each book instead of moving
        println!("- {} by {}", book.title, book.author);
    }
}

// FIX 6: Return a reference instead of owned value
fn find_book_by_title(library: &Library, title: &str) -> Option<&Book> {
    for book in &library.books {
        if book.title == title {
            return Some(book); // Return reference to book
        }
    }
    None // Return None if not found instead of panicking
}

// Alternative version that returns a cloned book if ownership is needed
fn find_book_by_title_owned(library: &Library, title: &str) -> Option<Book> {
    for book in &library.books {
        if book.title == title {
            return Some(book.clone()); // Clone the book to return owned value
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_book_creation() {
        let book = Book {
            title: String::from("Test Book"),
            author: String::from("Test Author"),
            pages: 100,
            available: true,
        };
        
        assert_eq!(book.title, "Test Book");
        assert_eq!(book.available, true);
    }

    #[test]
    fn test_library_operations() {
        let library = Library {
            name: String::from("Test Library"),
            books: vec![
                Book {
                    title: String::from("Book 1"),
                    author: String::from("Author 1"),
                    pages: 200,
                    available: true,
                },
            ],
        };
        
        // Test passes with proper borrowing
        assert_eq!(library.books.len(), 1);
        assert_eq!(library.books[0].title, "Book 1");
    }

    #[test]
    fn test_borrowing_patterns() {
        let mut library = Library {
            name: String::from("Borrow Library"),
            books: vec![
                Book {
                    title: String::from("Borrowable Book"),
                    author: String::from("Borrow Author"),
                    pages: 150,
                    available: true,
                },
            ],
        };
        
        // Test that we can borrow without moving
        let book_ref = &library.books[0];
        assert_eq!(book_ref.title, "Borrowable Book");
        
        // Should still be able to access library after borrowing
        assert_eq!(library.name, "Borrow Library");
        
        // Should be able to modify availability
        library.books[0].available = false;
        assert_eq!(library.books[0].available, false);
    }

    #[test]
    fn test_find_book_function() {
        let library = Library {
            name: String::from("Search Library"),
            books: vec![
                Book {
                    title: String::from("Findable Book"),
                    author: String::from("Search Author"),
                    pages: 300,
                    available: true,
                },
            ],
        };
        
        // Test finding existing book
        let found_book = find_book_by_title(&library, "Findable Book");
        assert!(found_book.is_some());
        assert_eq!(found_book.unwrap().author, "Search Author");
        
        // Test not finding book
        let not_found = find_book_by_title(&library, "Nonexistent Book");
        assert!(not_found.is_none());
    }
}