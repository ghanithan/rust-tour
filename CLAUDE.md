# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview
This is a Rust learning platform that provides progressive, exercise-based education following "The Rust Programming Language" book structure. The platform uses a hybrid approach with GitHub Repository + Codespaces as the primary delivery method.

## Development Commands

### Project Setup
```bash
# Initialize the project structure
./scripts/setup.sh

# Set up development container (if using Codespaces)
# The .devcontainer/ configuration handles this automatically

# Start the complete platform
./scripts/run.sh platform

# Start web development server only
./scripts/run.sh web
```

### Testing and Validation
```bash
# Run all tests for exercises
cargo test

# Run tests for specific exercise
cd exercises/ch01_getting_started/ex01_hello_world && cargo test

# Check exercise solution
./scripts/check-exercise.sh ch01 ex01

# Run all exercise validations
./scripts/run.sh test

# Lint code (Clippy integration)
cargo clippy -- -D warnings

# Format code
cargo fmt
```

### Progress Tracking
```bash
# Update progress (automated via scripts)
./scripts/progress-tracker.py update ch01 ex01

# View current progress
cat progress.json
```

## Architecture

### Core Structure
The project follows a chapter-based exercise structure:
- `exercises/chXX_topic/` - Individual chapter exercises
- `solutions/` - Reference solutions for all exercises
- `scripts/` - Automation scripts for testing and progress tracking
- `cli-tool/` - Optional CLI companion tool
- `progress.json` - Git-trackable progress file

### Exercise Framework
Each exercise follows this pattern:
- `src/main.rs` or `src/lib.rs` - Student implementation area
- `tests/` - Automated test cases
- `hints.md` - Progressive hint system (3 levels)
- Individual `Cargo.toml` for exercise-specific dependencies

### Exercise Types
1. **Code Completion** - Fill missing parts in skeleton code
2. **Bug Fixing** - Fix compilation/logic errors
3. **From Scratch** - Complete implementation from specs
4. **Code Review** - Analyze and improve existing code
5. **Performance Challenges** - Optimization exercises with benchmarks

## Development Workflow

### Creating New Exercises
1. Create chapter directory: `exercises/chXX_topic_name/`
2. Add exercise subdirectory: `exYY_exercise_name/`
3. Include: `src/`, `tests/`, `hints.md`, `Cargo.toml`
4. Update chapter README with exercise list
5. Add automated tests in `tests/` directory
6. Create progressive hints (beginner → intermediate → advanced)

### Exercise Validation
- All exercises must pass `cargo test`
- Include both unit tests and integration tests
- Use `cargo clippy` for code quality
- Performance exercises should include benchmarks
- Test cases should cover edge cases and common mistakes

### Progress System
- JSON-based tracking in `progress.json`
- Automatic updates via exercise completion scripts
- Tracks: completion status, time spent, difficulty rating, hints used
- Supports skill tree visualization and adaptive difficulty

## Testing Strategy
- Unit tests for individual functions/modules
- Integration tests for complete exercise solutions
- Benchmark tests for performance challenges
- Automated CI via `.github/workflows/test.yml`
- Clippy linting integrated into validation pipeline

## File Naming Conventions
- Chapters: `chXX_topic_name/` (snake_case)
- Exercises: `exYY_exercise_name/` (snake_case)
- Scripts: `kebab-case.sh` or `snake_case.py`
- Rust files: standard Rust conventions

## Key Implementation Notes
- Each exercise is a separate Cargo project for isolation
- Progressive difficulty within chapters (beginner → intermediate → advanced)
- Hint system provides 3 levels of assistance
- Solutions include multiple approaches where applicable
- Community contribution system for additional exercises
- Adaptive difficulty based on user performance metrics

# Exercise Creation Guidelines

## Exercise Structure Standards

### Required Directory Structure
```
exercises/chXX_topic_name/exYY_exercise_name/
├── Cargo.toml              # Exercise-specific dependencies
├── README.md               # Exercise description and instructions
├── hints.md                # Progressive hint system (3 levels)
├── metadata.json           # Exercise metadata and configuration
├── src/
│   ├── main.rs            # Primary implementation file
│   └── lib.rs             # Library implementation (if applicable)
├── tests/
│   └── unit_tests.rs      # Functional, outcome-based tests
└── solutions/
    ├── reference.rs       # Primary reference solution
    ├── alternative.rs     # Alternative approaches (optional)
    └── explained.md       # Solution explanation
```

### Metadata Requirements (metadata.json)
```json
{
  "id": "ch01-ex01-hello-world",
  "title": "Exercise Title",
  "description": "Clear, concise description of what the exercise teaches",
  "chapter": 1,
  "exercise_number": 1,
  "difficulty": "beginner|intermediate|advanced",
  "estimated_time_minutes": 10,
  "concepts": ["list", "of", "rust", "concepts"],
  "prerequisites": ["ch01-ex01", "ch02-ex03"],
  "exercise_type": "code_completion|bug_fixing|from_scratch|code_review|performance",
  "rust_book_refs": {
    "primary_chapter": "1.2",
    "supporting_chapters": ["1.1", "1.3"],
    "specific_sections": [
      {
        "chapter": "1.2",
        "title": "Section Title",
        "url": "https://doc.rust-lang.org/book/ch01-02-hello-world.html",
        "relevance": "core_concept|supporting|background"
      }
    ]
  },
  "hints": {
    "available": 3,
    "auto_unlock": false
  },
  "testing": {
    "timeout_seconds": 10,
    "memory_limit_mb": 50,
    "allow_std_only": true,
    "custom_checks": ["output_contains_hello_world"]
  },
  "validation": {
    "clippy_level": "warn|deny",
    "format_required": true,
    "custom_checks": ["no_hardcoded_values"]
  }
}
```

## Hint System Guidelines

### Three-Level Progressive Hints (hints.md)
```markdown
# Hints for [Exercise Title]

## Level 1: Conceptual Hint
- Explain the theoretical concepts
- Link to relevant Rust Book sections
- Provide high-level understanding
- No code examples, focus on concepts

## Level 2: Strategic Hint  
- Explain the approach and methodology
- Show syntax patterns without complete solutions
- Guide toward the right direction
- Include partial code examples

## Level 3: Implementation Hint
- Provide near-complete or complete solutions
- Explain every part of the implementation
- Show multiple approaches if applicable
- Include full working code examples
```

### Hint Quality Standards
- **Progressive disclosure**: Each level reveals more detail
- **Educational focus**: Teach concepts, not just solutions
- **Multiple learning styles**: Visual, textual, and code examples
- **Common mistakes**: Address typical student errors
- **Rust Book integration**: Link to relevant chapters and sections

## Exercise Types & Approaches

### 1. Code Completion Exercises
**Purpose**: Fill missing parts in skeleton code
```rust
fn main() {
    // TODO: Print "Hello, world!" to the console
    // Your code here
}
```
**Guidelines**:
- Use `// TODO:` or `unimplemented!()` markers
- Provide sufficient context around blanks
- Include comments explaining expected behavior
- Tests should guide toward correct implementation

### 2. Bug Fixing Exercises
**Purpose**: Fix compilation and logic errors
```rust
fn main() {
    let x = 5
    println!("The value is {}", y);  // Multiple intentional bugs
}
```
**Guidelines**:
- Include 2-4 strategic bugs (compilation + logic)
- Bugs should teach common Rust mistakes
- Error messages should guide toward solutions
- Tests should fail initially, pass when fixed

### 3. From Scratch Exercises
**Purpose**: Complete implementation from specifications
**Guidelines**:
- Provide clear requirements and specifications
- Include comprehensive test suite
- Start simple, build complexity gradually
- Multiple valid approaches should be acceptable

### 4. Code Review Exercises
**Purpose**: Improve working but non-idiomatic code
**Guidelines**:
- Provide functional but suboptimal code
- Focus on Rust best practices and idioms
- Include performance or safety improvements
- Explain trade-offs in different approaches

### 5. Performance Challenge Exercises
**Purpose**: Optimization and benchmarking
**Guidelines**:
- Include baseline implementation
- Provide benchmarking framework
- Set reasonable performance targets
- Explain optimization techniques used

## Testing Standards

### Test Philosophy: Functional & Outcome-Based
**DO**: Test actual program behavior and outcomes
```rust
#[test]
fn test_hello_world_output() {
    let output = Command::new("cargo")
        .args(&["run", "--quiet"])
        .output()
        .expect("Failed to execute program");
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.trim() == "Hello, world!", "Expected 'Hello, world!' output");
}
```

**DON'T**: Test for specific code patterns (heuristic testing)
```rust
// BAD - Don't do this
#[test]
fn test_contains_println() {
    let source = std::fs::read_to_string("src/main.rs").unwrap();
    assert!(source.contains("println!"), "Code should use println! macro");
}
```

### Test Categories

#### Unit Tests (tests/unit_tests.rs)
- Test individual functions and modules
- Verify correctness of implementation
- Cover edge cases and error conditions
- Use realistic inputs and expected outputs

#### Integration Tests
- Test complete program workflow
- Verify end-to-end functionality
- Test interaction between components
- Include performance benchmarks for optimization exercises

#### Quality Assurance
- Automated via `cargo clippy` and `cargo fmt`
- Custom validation in platform (not test files)
- Memory safety and best practices
- Documentation completeness

## Educational Approach

### Concept Progression
1. **Incremental Learning**: Each exercise builds on previous concepts
2. **Contextual Application**: Exercises demonstrate real-world usage
3. **Multiple Perspectives**: Show different ways to solve problems
4. **Error Learning**: Common mistakes become learning opportunities

### Difficulty Scaling
- **Beginner**: Single concept, clear instructions, guided implementation
- **Intermediate**: Multiple concepts, some ambiguity, require problem-solving
- **Advanced**: Complex scenarios, optimization, architectural decisions

### Rust Book Integration
- **Primary Chapter**: Core concept being taught
- **Supporting Chapters**: Background knowledge needed
- **Specific Sections**: Exact references with URLs
- **Relevance Tags**: How each section relates to the exercise

## Implementation Validation

### Quality Checklist
- [ ] Exercise compiles and runs successfully
- [ ] All tests pass reliably (`cargo test`)
- [ ] Follows Rust conventions (`cargo clippy`, `cargo fmt`)
- [ ] Hints provide progressive learning path
- [ ] Metadata accurately describes exercise
- [ ] README explains learning objectives clearly
- [ ] Tests are functional and outcome-based
- [ ] Rust Book references are accurate and helpful
- [ ] Exercise fits logically in chapter progression

### Performance Standards
- Compilation time < 30 seconds
- Test execution time < 10 seconds
- Memory usage < 50MB during execution
- Cross-platform compatibility (Windows, macOS, Linux)

### Accessibility Standards
- Clear, beginner-friendly language
- Consistent terminology with Rust Book
- Progressive complexity within chapters
- Multiple learning modalities (reading, coding, testing)
- Inclusive examples and scenarios