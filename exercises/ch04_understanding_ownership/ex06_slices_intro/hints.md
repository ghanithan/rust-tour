# Hints for The Slice Type - Views Into Data

## Level 1: Conceptual Hint

**What are Slices?**
Slices are like "windows" into data. They let you look at part of a String, Vec, or array without copying the data or taking ownership.

**Two Main Types**:
1. **String slices** (`&str`) - view into string data
2. **Array slices** (`&[T]`) - view into array/vector data

**Key Advantages**:
- **No copying**: Slices reference existing data
- **No ownership transfer**: Original data remains owned by its owner
- **Efficient**: Just a pointer and length, very cheap to create
- **Safe**: Rust ensures slices are always valid

**Slice Syntax**:
- `&s[start..end]` - slice from start to end-1
- `&s[start..]` - slice from start to end of data
- `&s[..end]` - slice from beginning to end-1
- `&s[..]` - slice of entire data

**Mental Model**: Think of a slice as a "bookmark" that points to a specific section of a book. You can read that section without needing to own the entire book.

Review Rust Book section 4.3 to understand how slices solve the ownership challenges from earlier exercises.

## Level 2: Strategic Hint

**Function Signatures**:
Most of your functions should take `&str` for strings or `&[T]` for arrays:

```rust
fn first_word(text: &str) -> &str        // String slice parameter and return
fn find_even_numbers(nums: &[i32]) -> Vec<i32>  // Array slice parameter
```

**String Operations**:
- `s.find(' ')` - finds first space, returns `Option<usize>`
- `s.rfind(' ')` - finds last space
- `s.split_whitespace()` - iterator over words
- `&s[start..end]` - creates slice

**Array Operations**:
- `&arr[start..end]` - creates slice
- `slice.iter()` - iterate over slice
- `slice.chunks(n)` - iterate in chunks of size n

**Common Patterns**:

**Finding word boundaries**:
```rust
if let Some(pos) = text.find(' ') {
    &text[0..pos]  // Everything before the space
} else {
    text  // No space found, return whole string
}
```

**Range slicing**:
```rust
let len = numbers.len();
&numbers[0..len/2]      // First half
&numbers[len/2..]       // Second half
&numbers[2..8]          // Specific range
```

**Safe indexing**:
```rust
if end <= text.len() && start <= end {
    &text[start..end]
} else {
    ""  // Return empty string for invalid bounds
}
```

## Level 3: Implementation Hint

**Complete solutions**:

```rust
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
    
    let first = first_word(text);  // Pass &str directly
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
    
    let first_half = &numbers[0..5];
    println!("First half: {:?}", first_half);
    
    let last_half = &numbers[5..];
    println!("Last half: {:?}", last_half);
    
    let middle = &numbers[2..8];
    println!("Middle section: {:?}", middle);
    
    let evens = find_even_numbers(&numbers);
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
    process_chunks(&data, 3);
}

// Function implementations:

fn first_word(text: &str) -> &str {
    if let Some(pos) = text.find(' ') {
        &text[0..pos]
    } else {
        text
    }
}

fn last_word(text: &str) -> &str {
    if let Some(pos) = text.rfind(' ') {
        &text[pos + 1..]
    } else {
        text
    }
}

fn middle_section(text: &str) -> &str {
    let first_space = text.find(' ').unwrap_or(0);
    let last_space = text.rfind(' ').unwrap_or(text.len());
    
    if first_space < last_space {
        &text[first_space + 1..last_space]
    } else {
        ""
    }
}

fn find_words_starting_with(text: &str, ch: char) -> &str {
    for word in text.split_whitespace() {
        if word.starts_with(ch) {
            return word;
        }
    }
    ""
}

fn find_even_numbers(nums: &[i32]) -> Vec<i32> {
    nums.iter()
        .filter(|&&n| n % 2 == 0)
        .copied()
        .collect()
}

fn get_file_extension(filename: &str) -> &str {
    if let Some(pos) = filename.rfind('.') {
        &filename[pos + 1..]
    } else {
        ""
    }
}

fn get_domain(email: &str) -> &str {
    if let Some(pos) = email.find('@') {
        &email[pos + 1..]
    } else {
        ""
    }
}

fn safe_substring(text: &str, start: usize, end: usize) -> &str {
    if start <= end && end <= text.len() {
        &text[start..end]
    } else {
        ""
    }
}

fn process_chunks(data: &[i32], chunk_size: usize) {
    for (i, chunk) in data.chunks(chunk_size).enumerate() {
        if i < data.len() / chunk_size {
            println!("Chunk {}: {:?}", i + 1, chunk);
        } else {
            println!("Remaining: {:?}", chunk);
        }
    }
}
```

**Key insights**:
1. **String literals are already `&str`** - no need to convert
2. **Use `&` to create slices** from owned data like Vec or String
3. **Slices are references** - they don't own the data
4. **Range syntax** is inclusive of start, exclusive of end
5. **Methods like `find()` return `Option<usize>`** for safe indexing