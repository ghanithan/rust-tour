#[path = "../src/main.rs"]
mod main;

use main::StringBuffer;

#[test]
fn test_new_buffer_is_empty() {
    let buffer = StringBuffer::new();
    assert_eq!(buffer.len(), 0, "New buffer should be empty");
}

#[test]
fn test_add_increases_length() {
    let mut buffer = StringBuffer::new();
    buffer.add(String::from("Hello"));
    assert_eq!(buffer.len(), 1);
    
    buffer.add(String::from("World"));
    assert_eq!(buffer.len(), 2);
}

#[test]
fn test_ownership_transfer_on_add() {
    let mut buffer = StringBuffer::new();
    let s = String::from("Test");
    buffer.add(s);
    // s is now moved and cannot be used
    // This test passes if the code compiles - ownership was properly transferred
    assert_eq!(buffer.len(), 1);
}

#[test]
fn test_remove_last_returns_ownership() {
    let mut buffer = StringBuffer::new();
    buffer.add(String::from("First"));
    buffer.add(String::from("Second"));
    
    let removed = buffer.remove_last();
    assert_eq!(removed, Some(String::from("Second")));
    assert_eq!(buffer.len(), 1);
    
    // We now own 'removed' and can use it
    if let Some(s) = removed {
        assert_eq!(s.len(), 6); // "Second" has 6 characters
    }
}

#[test]
fn test_remove_from_empty_buffer() {
    let mut buffer = StringBuffer::new();
    assert_eq!(buffer.remove_last(), None);
}

#[test]
fn test_flush_combines_strings() {
    let mut buffer = StringBuffer::new();
    buffer.add(String::from("Hello"));
    buffer.add(String::from(" "));
    buffer.add(String::from("World"));
    
    let result = buffer.flush();
    assert_eq!(result, "Hello World");
    assert_eq!(buffer.len(), 0, "Buffer should be empty after flush");
}

#[test]
fn test_flush_empty_buffer() {
    let mut buffer = StringBuffer::new();
    let result = buffer.flush();
    assert_eq!(result, "");
    assert_eq!(buffer.len(), 0);
}

#[test]
fn test_multiple_operations() {
    let mut buffer = StringBuffer::new();
    
    // Add some strings
    buffer.add(String::from("A"));
    buffer.add(String::from("B"));
    buffer.add(String::from("C"));
    assert_eq!(buffer.len(), 3);
    
    // Remove one
    let removed = buffer.remove_last();
    assert_eq!(removed, Some(String::from("C")));
    assert_eq!(buffer.len(), 2);
    
    // Add more
    buffer.add(String::from("D"));
    buffer.add(String::from("E"));
    assert_eq!(buffer.len(), 4);
    
    // Flush all
    let result = buffer.flush();
    assert_eq!(result, "ABDE");
    assert_eq!(buffer.len(), 0);
}

#[test]
fn test_ownership_demonstration() {
    // This test verifies the program runs and demonstrates ownership
    use std::process::Command;
    
    let output = Command::new("cargo")
        .args(&["run", "--quiet"])
        .output()
        .expect("Failed to execute program");
    
    assert!(output.status.success(), "Program should compile and run");
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Adding strings to buffer"), "Should show adding strings");
    assert!(stdout.contains("Buffer contains"), "Should show buffer length");
    assert!(stdout.contains("Removed:"), "Should show removed string");
    assert!(stdout.contains("Flushed:"), "Should show flushed content");
}