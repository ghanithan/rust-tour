// TODO: Implement StringBuffer to manage a collection of owned strings
// Remember: When you add a string, the buffer takes ownership
// When you remove a string, ownership transfers to the caller

struct StringBuffer {
    // TODO: Add a field to store strings
}

impl StringBuffer {
    fn new() -> Self {
        // TODO: Create a new empty StringBuffer
        unimplemented!("Create a new StringBuffer")
    }
    
    fn add(&mut self, text: String) {
        // TODO: Take ownership of the string and add it to the buffer
        unimplemented!("Add string to buffer")
    }
    
    fn remove_last(&mut self) -> Option<String> {
        // TODO: Remove and return the last string, transferring ownership
        // Return None if buffer is empty
        unimplemented!("Remove and return last string")
    }
    
    fn flush(&mut self) -> String {
        // TODO: Combine all strings into one and clear the buffer
        // Hint: You'll need to move strings out of the vector
        unimplemented!("Flush buffer contents")
    }
    
    fn len(&self) -> usize {
        // TODO: Return the number of strings in the buffer
        unimplemented!("Get buffer length")
    }
}

fn main() {
    // Demonstrate ownership transfers with StringBuffer
    let mut buffer = StringBuffer::new();
    
    // These strings will be moved into the buffer
    let s1 = String::from("Hello");
    let s2 = String::from(", ");
    let s3 = String::from("World!");
    
    println!("Adding strings to buffer...");
    buffer.add(s1);  // s1 is moved
    buffer.add(s2);  // s2 is moved
    buffer.add(s3);  // s3 is moved
    
    // Can't use s1, s2, or s3 here - they were moved!
    // println!("{}", s1);  // This would error
    
    println!("Buffer contains {} strings", buffer.len());
    
    // Remove one string and take ownership
    if let Some(last) = buffer.remove_last() {
        println!("Removed: {}", last);  // We own 'last' now
    }
    
    // Flush remaining contents
    let result = buffer.flush();
    println!("Flushed: {}", result);
    println!("Buffer now contains {} strings", buffer.len());
    
    // Demonstrate moving strings from a function
    let generated = create_message();
    buffer.add(generated);  // Function's return value is moved
    
    println!("Final flush: {}", buffer.flush());
}

fn create_message() -> String {
    let msg = String::from("Generated message");
    msg  // Ownership transferred to caller
}