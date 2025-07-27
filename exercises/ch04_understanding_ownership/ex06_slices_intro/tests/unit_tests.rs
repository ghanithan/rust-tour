use std::process::Command;

#[test]
fn test_program_compiles_and_runs() {
    let output = Command::new("cargo")
        .args(&["run", "--quiet"])
        .output()
        .expect("Failed to execute program");

    assert!(output.status.success(), "Program should compile and run successfully");
}

#[test]
fn test_string_slice_output() {
    let output = Command::new("cargo")
        .args(&["run", "--quiet"])
        .output()
        .expect("Failed to execute program");

    let stdout = String::from_utf8_lossy(&output.stdout);
    
    assert!(stdout.contains("=== String Slices ==="), "Should have string slices section");
    assert!(stdout.contains("First word: The"), "Should find first word");
    assert!(stdout.contains("Last word: dog"), "Should find last word");
    assert!(stdout.contains("Middle section:"), "Should show middle section");
    assert!(stdout.contains("Words starting with 'o': over"), "Should find words starting with o");
}

#[test]
fn test_array_slice_output() {
    let output = Command::new("cargo")
        .args(&["run", "--quiet"])
        .output()
        .expect("Failed to execute program");

    let stdout = String::from_utf8_lossy(&output.stdout);
    
    assert!(stdout.contains("=== Array Slices ==="), "Should have array slices section");
    assert!(stdout.contains("Numbers: [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]"), "Should show all numbers");
    assert!(stdout.contains("First half: [1, 2, 3, 4, 5]"), "Should show first half");
    assert!(stdout.contains("Last half: [6, 7, 8, 9, 10]"), "Should show last half");
    assert!(stdout.contains("Middle section:"), "Should show middle section");
    assert!(stdout.contains("Even numbers found:"), "Should find even numbers");
}

#[test]
fn test_practical_examples_output() {
    let output = Command::new("cargo")
        .args(&["run", "--quiet"])
        .output()
        .expect("Failed to execute program");

    let stdout = String::from_utf8_lossy(&output.stdout);
    
    assert!(stdout.contains("=== Practical Examples ==="), "Should have practical examples section");
    assert!(stdout.contains("File extension: txt"), "Should extract file extension");
    assert!(stdout.contains("Domain: example.com"), "Should extract domain");
    assert!(stdout.contains("Safe substring: Hello"), "Should create safe substring");
    assert!(stdout.contains("Chunk"), "Should process chunks");
}

// Test individual functions with isolated implementations
#[cfg(test)]
mod slice_function_tests {
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

    fn safe_substring(text: &str, start: usize, end: usize) -> &str {
        if start <= end && end <= text.len() {
            &text[start..end]
        } else {
            ""
        }
    }

    #[test]
    fn test_first_word_function() {
        assert_eq!(first_word("Hello world"), "Hello");
        assert_eq!(first_word("SingleWord"), "SingleWord");
        assert_eq!(first_word("The quick brown fox"), "The");
    }

    #[test]
    fn test_last_word_function() {
        assert_eq!(last_word("Hello world"), "world");
        assert_eq!(last_word("SingleWord"), "SingleWord");
        assert_eq!(last_word("The quick brown fox"), "fox");
    }

    #[test]
    fn test_find_even_numbers_function() {
        let numbers = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let evens = find_even_numbers(&numbers);
        assert_eq!(evens, vec![2, 4, 6, 8, 10]);
        
        let odds = [1, 3, 5, 7, 9];
        let evens = find_even_numbers(&odds);
        assert_eq!(evens, Vec::<i32>::new());
    }

    #[test]
    fn test_get_file_extension_function() {
        assert_eq!(get_file_extension("file.txt"), "txt");
        assert_eq!(get_file_extension("document.pdf"), "pdf");
        assert_eq!(get_file_extension("archive.tar.gz"), "gz");
        assert_eq!(get_file_extension("noextension"), "");
    }

    #[test]
    fn test_safe_substring_function() {
        let text = "Hello, World!";
        assert_eq!(safe_substring(text, 0, 5), "Hello");
        assert_eq!(safe_substring(text, 7, 12), "World");
        assert_eq!(safe_substring(text, 0, 100), ""); // Out of bounds
        assert_eq!(safe_substring(text, 5, 3), ""); // Invalid range
    }

    #[test]
    fn test_slice_ranges() {
        let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        
        // Test different slice ranges
        assert_eq!(&numbers[0..5], &[1, 2, 3, 4, 5]);
        assert_eq!(&numbers[5..], &[6, 7, 8, 9, 10]);
        assert_eq!(&numbers[2..8], &[3, 4, 5, 6, 7, 8]);
        assert_eq!(&numbers[..3], &[1, 2, 3]);
        assert_eq!(&numbers[..], &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    }
}