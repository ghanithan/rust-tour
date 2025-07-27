// Solution: StringBuffer implementation demonstrating move semantics

struct StringBuffer {
    strings: Vec<String>
}

impl StringBuffer {
    fn new() -> Self {
        StringBuffer {
            strings: Vec::new()
        }
    }
    
    fn add(&mut self, text: String) {
        // Take ownership of the string and store it in our Vec
        self.strings.push(text);
    }
    
    fn remove_last(&mut self) -> Option<String> {
        // Pop removes the last element and transfers ownership to caller
        self.strings.pop()
    }
    
    fn flush(&mut self) -> String {
        // Drain all strings from the vec and concatenate them
        // This moves each string out of the vec, leaving it empty
        self.strings.drain(..).collect()
    }
    
    fn len(&self) -> usize {
        self.strings.len()
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