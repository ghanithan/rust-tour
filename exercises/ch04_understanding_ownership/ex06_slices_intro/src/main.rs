// Learn to work with slices - views into data without ownership

fn main() {
    println!("=== String Slices ===");
    string_slice_examples();
    
    println!("\n=== Array Slices ===");
    array_slice_examples();
    
    println!("\n=== Practical Examples ===");
    practical_examples();
}

fn string_slice_examples() {
    let text = "The quick brown fox jumps over the lazy dog";
    println!("Original: {}", text);
    
    // TODO: Complete these functions to work with string slices
    let first = first_word(/* TODO: pass appropriate slice or reference */);
    println!("First word: {}", first);
    
    let last = last_word(/* TODO: pass appropriate slice or reference */);
    println!("Last word: {}", last);
    
    let middle = middle_section(/* TODO: pass appropriate slice or reference */);
    println!("Middle section: {}", middle);
    
    let words_with_o = find_words_starting_with(/* TODO: pass appropriate slice or reference */, 'o');
    println!("Words starting with 'o': {}", words_with_o);
}

fn array_slice_examples() {
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    println!("Numbers: {:?}", numbers);
    
    // TODO: Create slices using range syntax
    let first_half = /* TODO: slice first half of numbers */;
    println!("First half: {:?}", first_half);
    
    let last_half = /* TODO: slice last half of numbers */;
    println!("Last half: {:?}", last_half);
    
    let middle = /* TODO: slice middle section (indices 2-7) */;
    println!("Middle section: {:?}", middle);
    
    let evens = find_even_numbers(/* TODO: pass slice of numbers */);
    println!("Even numbers found: {:?}", evens);
}

fn practical_examples() {
    // File extension extraction
    let filename = "document.txt";
    let extension = get_file_extension(/* TODO: pass filename */);
    println!("File extension: {}", extension);
    
    // Domain extraction from email
    let email = "user@example.com";
    let domain = get_domain(/* TODO: pass email */);
    println!("Domain: {}", domain);
    
    // Safe substring that won't panic
    let text = "Hello, World!";
    let substr = safe_substring(/* TODO: pass text */, 0, 5);
    println!("Safe substring: {}", substr);
    
    // Array chunking
    let data = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    process_chunks(/* TODO: pass slice of data */, 3);
}

// TODO: Implement these functions using slices

// Return the first word from a string slice
fn first_word(/* TODO: parameter type */) -> &str {
    // TODO: Find the first space and return slice up to that point
    // If no space found, return the entire string
    unimplemented!("Find and return the first word")
}

// Return the last word from a string slice
fn last_word(/* TODO: parameter type */) -> &str {
    // TODO: Find the last space and return slice after that point
    // If no space found, return the entire string
    unimplemented!("Find and return the last word")
}

// Return the middle section, skipping first and last words
fn middle_section(/* TODO: parameter type */) -> &str {
    // TODO: Find first and last word boundaries and return what's between
    unimplemented!("Return middle section between first and last words")
}

// Find the first word that starts with the given character
fn find_words_starting_with(/* TODO: parameter type */, ch: char) -> &str {
    // TODO: Split into words and find first word starting with ch
    unimplemented!("Find word starting with character")
}

// Return a vector of even numbers from the slice
fn find_even_numbers(/* TODO: parameter type */) -> Vec<i32> {
    // TODO: Filter slice for even numbers and collect into Vec
    unimplemented!("Find even numbers")
}

// Extract file extension from filename
fn get_file_extension(/* TODO: parameter type */) -> &str {
    // TODO: Find last '.' and return slice after it
    // Return empty string if no extension
    unimplemented!("Extract file extension")
}

// Extract domain from email address
fn get_domain(/* TODO: parameter type */) -> &str {
    // TODO: Find '@' and return slice after it
    unimplemented!("Extract domain from email")
}

// Safe substring that returns slice or empty string if out of bounds
fn safe_substring(/* TODO: parameter type */, start: usize, end: usize) -> &str {
    // TODO: Check bounds and return appropriate slice
    // Return empty string if indices are invalid
    unimplemented!("Create safe substring")
}

// Process array in chunks of given size
fn process_chunks(/* TODO: parameter type */, chunk_size: usize) {
    // TODO: Iterate through slice in chunks and print each chunk
    // Print any remaining elements as final chunk
    unimplemented!("Process array in chunks")
}