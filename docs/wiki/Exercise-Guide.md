# Exercise Guide

Master Rust Tour exercises with this comprehensive guide covering exercise types, workflow, testing strategies, and learning techniques.

## Exercise Structure

Every Rust Tour exercise follows a consistent structure designed for progressive learning:

```
exercises/ch01_getting_started/ex01_hello_world/
├── Cargo.toml          # Project dependencies and metadata
├── README.md           # Exercise description and learning objectives
├── metadata.json       # Exercise configuration and requirements
├── src/
│   ├── main.rs         # Your implementation area
│   └── lib.rs          # Library code (when applicable)
├── tests/
│   └── unit_tests.rs   # Automated test cases
├── hints.md            # Progressive hint system (3 levels)
└── solutions/
    ├── reference.rs    # Primary reference solution
    ├── alternative.rs  # Alternative approaches
    └── explained.md    # Solution explanation
```

## Exercise Types

### 1. Code Completion
**Purpose**: Fill in missing parts of skeleton code
**Approach**: Focus on specific concepts while providing context

```rust
fn main() {
    // TODO: Print "Hello, world!" to the console
    // Your code here
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_output() {
        // Test checks for correct output
    }
}
```

**Strategy**:
- Read the TODO comments carefully
- Understand what the tests expect
- Start with the simplest implementation

### 2. Bug Fixing
**Purpose**: Find and fix compilation and logic errors
**Approach**: Develop debugging skills and learn common mistakes

```rust
fn main() {
    let x = 5    // Missing semicolon
    println!("The value is {}", y);  // Wrong variable name
}
```

**Strategy**:
- Read error messages carefully
- Fix compilation errors first
- Run tests to identify logic errors
- Use `cargo clippy` for additional insights

### 3. From Scratch Implementation
**Purpose**: Build complete programs from specifications
**Approach**: Apply learned concepts to solve real problems

**Strategy**:
- Read the README thoroughly
- Understand the test cases
- Start with a basic implementation
- Iterate based on test feedback

### 4. Code Review
**Purpose**: Improve working but non-idiomatic code
**Approach**: Learn Rust best practices and idioms

**Strategy**:
- Run the existing code to understand behavior
- Identify performance or safety improvements
- Apply Rust idioms and best practices
- Ensure tests still pass

### 5. Performance Challenges
**Purpose**: Optimization and benchmarking
**Approach**: Learn about Rust's performance characteristics

**Strategy**:
- Establish baseline performance
- Use profiling tools
- Apply optimization techniques
- Validate improvements with benchmarks

## Working with Exercises

### Basic Workflow

1. **Navigate to exercise directory**
   ```bash
   cd exercises/ch01_getting_started/ex01_hello_world
   ```

2. **Read the exercise description**
   ```bash
   cat README.md
   ```

3. **Understand the requirements**
   - Check `metadata.json` for specific requirements
   - Review test cases in `tests/unit_tests.rs`

4. **Implement your solution**
   ```bash
   # Edit the source files
   code src/main.rs  # or vim, nano, etc.
   ```

5. **Test your implementation**
   ```bash
   # Run all tests
   cargo test
   
   # Run with output visible
   cargo test -- --nocapture
   
   # Run specific test
   cargo test test_hello_world_output
   ```

6. **Verify code quality**
   ```bash
   # Check for common issues
   cargo clippy -- -D warnings
   
   # Format code
   cargo fmt
   
   # Run your program
   cargo run
   ```

### Using the Hint System

Each exercise includes a three-level progressive hint system:

#### Level 1: Conceptual Hints
- High-level approach explanation
- Relevant Rust Book sections
- Key concepts to understand

#### Level 2: Strategic Hints
- Implementation methodology
- Syntax patterns and examples
- Common approaches

#### Level 3: Implementation Hints
- Near-complete solutions
- Detailed explanations
- Alternative approaches

**Strategy**: Try to solve exercises without hints first. Use hints progressively when stuck.

### Testing Strategy

#### Understanding Test Output
```bash
$ cargo test
running 3 tests
test tests::test_basic_functionality ... ok
test tests::test_edge_cases ... FAILED
test tests::test_error_handling ... ok

failures:

---- tests::test_edge_cases stdout ----
thread 'tests::test_edge_cases' panicked at 'assertion failed: ...'
```

#### Debugging Failed Tests
1. **Read the failure message carefully**
2. **Understand what the test expects**
3. **Add debug prints to your code**
   ```rust
   println!("Debug: value = {}", value);
   ```
4. **Run tests with output visible**
   ```bash
   cargo test -- --nocapture
   ```

#### Test-Driven Development
1. Make the test compile
2. Make the test pass
3. Refactor for clarity and performance
4. Ensure all tests still pass

## Learning Strategies

### Progressive Mastery
- **Complete exercises in order** within each chapter
- **Don't skip exercises** - each builds on previous knowledge
- **Review previous exercises** when concepts reappear

### Active Learning Techniques
- **Explain concepts aloud** - teach the rubber duck
- **Implement variations** - try different approaches
- **Connect to prior knowledge** - relate to other programming languages

### When You're Stuck
1. **Re-read the exercise description**
2. **Check Level 1 hints**
3. **Review relevant Rust Book sections**
4. **Search previous exercises for similar patterns**
5. **Ask specific questions in [Discussions](https://github.com/ghanithan/rust-tour/discussions)**

### Effective Practice
- **Set consistent study time** - regular practice beats marathon sessions
- **Focus on understanding** over speed
- **Track your progress** - celebrate small wins
- **Join community discussions** - learn from others' approaches

## Advanced Techniques

### Using Rust Tools
```bash
# Detailed error information
RUST_BACKTRACE=1 cargo run

# Watch for changes and re-run tests
cargo watch -x test

# Generate documentation
cargo doc --open

# Check for unused dependencies
cargo machete
```

### IDE Integration
- **VS Code**: Install rust-analyzer extension
- **IntelliJ**: Rust plugin
- **Vim/Neovim**: rust.vim and coc-rust-analyzer
- **Emacs**: rustic-mode

### Debugging Techniques
```rust
// Add debug prints
dbg!(variable);

// Use the debugger
// Set breakpoints in your IDE
// Or use gdb/lldb with cargo
```

## Common Patterns

### Error Handling
```rust
// Basic error handling
match result {
    Ok(value) => println!("Success: {}", value),
    Err(error) => eprintln!("Error: {}", error),
}

// Using ? operator
fn process() -> Result<String, Box<dyn std::error::Error>> {
    let content = std::fs::read_to_string("file.txt")?;
    Ok(content.trim().to_string())
}
```

### Ownership Patterns
```rust
// Borrowing
fn process_data(data: &str) -> usize {
    data.len()
}

// Taking ownership
fn consume_data(data: String) -> String {
    data.to_uppercase()
}

// Mutable borrowing
fn modify_data(data: &mut String) {
    data.push_str(" modified");
}
```

## Progress Tracking

Your progress is automatically tracked:
- **Exercise completion status**
- **Time spent per exercise**
- **Hint usage patterns**
- **Code quality metrics**
- **Learning velocity**

View your progress in the web interface dashboard.

## Getting Help

- **Exercise-specific questions**: Use [Q&A Discussions](https://github.com/ghanithan/rust-tour/discussions/categories/q-a)
- **General Rust questions**: Visit [users.rust-lang.org](https://users.rust-lang.org)
- **Platform issues**: Check [Troubleshooting](Troubleshooting) or file an [issue](https://github.com/ghanithan/rust-tour/issues)

Remember: Every expert was once a beginner. The key is consistent practice and asking good questions when you need help.