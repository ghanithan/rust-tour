// Solution: Working with slices - views into data

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
    
    let first = first_word(text);  // text is already &str
    println!("First word: {}", first);
    
    let last = last_word(text);
    println!("Last word: {}", last);
    
    let middle = middle_section(text);
    println!("Middle section: {}", middle);
    
    let words_with_o = find_words_starting_with(text, 'o');
    println!("Words starting with 'o': {}", words_with_o);
}

fn array_slice_examples() {
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    println!("Numbers: {:?}", numbers);
    
    let first_half = &numbers[0..5];  // Slice first 5 elements
    println!("First half: {:?}", first_half);
    
    let last_half = &numbers[5..];  // Slice from index 5 to end
    println!("Last half: {:?}", last_half);
    
    let middle = &numbers[2..8];  // Slice from index 2 to 7
    println!("Middle section: {:?}", middle);
    
    let evens = find_even_numbers(&numbers);  // Pass slice of numbers
    println!("Even numbers found: {:?}", evens);
}

fn practical_examples() {
    let filename = "document.txt";
    let extension = get_file_extension(filename);
    println!("File extension: {}", extension);
    
    let email = "user@example.com";
    let domain = get_domain(email);
    println!("Domain: {}", domain);
    
    let text = "Hello, World!";
    let substr = safe_substring(text, 0, 5);
    println!("Safe substring: {}", substr);
    
    let data = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    process_chunks(&data, 3);  // Pass slice of array
}

// Function implementations using slices

fn first_word(text: &str) -> &str {
    if let Some(pos) = text.find(' ') {
        &text[0..pos]  // Return slice from start to first space
    } else {
        text  // No space found, return entire string
    }
}

fn last_word(text: &str) -> &str {
    if let Some(pos) = text.rfind(' ') {
        &text[pos + 1..]  // Return slice from after last space to end
    } else {
        text  // No space found, return entire string
    }
}

fn middle_section(text: &str) -> &str {
    let first_space = text.find(' ').unwrap_or(0);
    let last_space = text.rfind(' ').unwrap_or(text.len());
    
    if first_space < last_space {
        &text[first_space + 1..last_space]  // Between first and last word
    } else {
        ""  // No middle section (less than 3 words)
    }
}

fn find_words_starting_with(text: &str, ch: char) -> &str {
    for word in text.split_whitespace() {
        if word.starts_with(ch) {
            return word;  // Return the word slice
        }
    }
    ""  // No word found starting with character
}

fn find_even_numbers(nums: &[i32]) -> Vec<i32> {
    nums.iter()
        .filter(|&&n| n % 2 == 0)  // Filter for even numbers
        .copied()                   // Copy the values (since we're filtering &i32)
        .collect()                  // Collect into Vec
}

fn get_file_extension(filename: &str) -> &str {
    if let Some(pos) = filename.rfind('.') {
        &filename[pos + 1..]  // Return slice after the last dot
    } else {
        ""  // No extension found
    }
}

fn get_domain(email: &str) -> &str {
    if let Some(pos) = email.find('@') {
        &email[pos + 1..]  // Return slice after the @ symbol
    } else {
        ""  // No @ found (invalid email)
    }
}

fn safe_substring(text: &str, start: usize, end: usize) -> &str {
    if start <= end && end <= text.len() {
        &text[start..end]  // Safe to create slice
    } else {
        ""  // Invalid bounds, return empty string
    }
}

fn process_chunks(data: &[i32], chunk_size: usize) {
    let mut chunk_num = 1;
    
    for chunk in data.chunks(chunk_size) {
        if chunk.len() == chunk_size {
            println!("Chunk {}: {:?}", chunk_num, chunk);
        } else {
            println!("Remaining: {:?}", chunk);  // Last partial chunk
        }
        chunk_num += 1;
    }
}