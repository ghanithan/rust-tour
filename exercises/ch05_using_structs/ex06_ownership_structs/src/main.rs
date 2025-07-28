// This exercise demonstrates common ownership issues with structs
// Fix the compilation errors and ownership issues in this code

#[derive(Debug)]
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
    
    // BUG 1: This will cause a partial move - fix it
    let first_book = library.books[0];
    println!("First book: {:?}", first_book);
    
    // BUG 2: Cannot use library.books after partial move
    display_library_info(&library);
    
    // BUG 3: Trying to modify after move
    library.books[1].available = false;
    
    // BUG 4: Using moved value
    let book_title = first_book.title;
    println!("Title: {}", first_book.title); // This should fail
    
    // BUG 5: Double move issue
    let another_book = library.books[1];
    let yet_another = library.books[1];
    
    println!("Books processed successfully!");
}

// This function should display library information without taking ownership
fn display_library_info(lib: Library) {
    println!("Library: {}", lib.name);
    println!("Total books: {}", lib.books.len());
    for book in lib.books {
        println!("- {} by {}", book.title, book.author);
    }
}

// Helper function to borrow a book (should return a reference)
fn find_book_by_title(library: &Library, title: &str) -> Book {
    for book in &library.books {
        if book.title == title {
            return book; // BUG 6: Cannot return owned value from borrowed data
        }
    }
    panic!("Book not found");
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
        
        // This test should pass when ownership issues are fixed
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
}