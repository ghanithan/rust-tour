# Contributing to Rust Learning Platform

Thank you for your interest in contributing! This document provides guidelines and information for contributors.

## 🎯 Ways to Contribute

### 📝 Exercise Contributions
- Create new exercises following our templates
- Improve existing exercises based on learner feedback
- Add alternative solutions and explanations
- Enhance hint systems and error messages

### 🐛 Bug Reports & Fixes
- Report issues with exercises or platform functionality
- Fix compilation errors, test failures, or UI bugs
- Improve error messages and user experience

### ✨ Feature Development
- Implement new platform features
- Enhance web UI or CLI functionality
- Add integration with external tools
- Improve progress tracking and analytics

### 📚 Documentation & Content
- Improve documentation and setup guides
- Create video tutorials or written guides
- Translate content to other languages
- Enhance Rust Book integration

## 🚀 Getting Started

### Development Setup
```bash
# Fork the repository on GitHub
git clone https://github.com/YOUR_USERNAME/rust-learning-platform.git
cd rust-learning-platform

# Run the setup script
./scripts/setup.sh

# Create a feature branch
git checkout -b feature/your-feature-name
```

### Development Environment
- **Primary**: GitHub Codespaces (recommended for consistency)
- **Alternative**: Local with Docker or native setup
- **Tools**: VS Code with Rust Analyzer extension

## 📝 Exercise Contribution Guidelines

### Creating New Exercises

1. **Use the Exercise Generator**
   ```bash
   ./scripts/create-exercise.sh ch03 temperature_conversion --type from_scratch
   ```

2. **Follow the Template Structure**
   ```
   exercises/chXX_topic/exYY_name/
   ├── Cargo.toml              # Exercise dependencies
   ├── README.md               # Exercise description
   ├── hints.md                # Progressive hint system (at root level)
   ├── metadata.json           # Exercise configuration
   ├── src/
   │   ├── main.rs            # Student implementation area
   │   └── lib.rs             # Library implementation (if applicable)
   ├── tests/
   │   └── unit_tests.rs      # Functional, outcome-based tests
   └── solutions/
       ├── reference.rs       # Primary solution
       ├── alternative.rs     # Alternative approaches
       └── explained.md       # Solution explanation
   ```

   **Note**: `hints.md` should be at the exercise root level, not inside `src/`. This keeps hints as project documentation rather than source code.

3. **Exercise Metadata Requirements**
   ```json
   {
     "id": "ch03-ex02-data-types",
     "title": "Temperature Conversion",
     "description": "Implement functions to convert between temperature scales",
     "difficulty": "intermediate",
     "estimated_time": "25m",
     "concepts": ["functions", "data-types", "arithmetic"],
     "prerequisites": ["ch02-ex03", "ch03-ex01"],
     "rust_book_refs": {
       "primary_chapter": "3.2",
       "supporting_chapters": ["3.1"]
     },
     "type": "from_scratch"
   }
   ```

### Exercise Quality Standards

#### ✅ Required Elements
- [ ] Clear, unambiguous exercise description
- [ ] Appropriate difficulty for target audience
- [ ] Functional, outcome-based tests (no heuristic testing)
- [ ] Progressive 3-level hint system at exercise root level
- [ ] Reference solution with explanation
- [ ] Rust Book chapter references with URLs
- [ ] Beginner-friendly error messages

#### 🎯 Educational Goals
- [ ] Teaches specific Rust concept clearly
- [ ] Builds on previous exercises logically
- [ ] Common mistakes addressed in hints
- [ ] Multiple solution approaches discussed
- [ ] Real-world applicability demonstrated

#### 🔧 Technical Requirements
- [ ] Compiles without warnings (`cargo clippy`)
- [ ] Follows Rust formatting (`cargo fmt`)
- [ ] All tests pass reliably
- [ ] Tests validate actual program behavior, not code patterns
- [ ] Performance within reasonable bounds
- [ ] Cross-platform compatibility

### Exercise Types Guidelines

#### 📝 Code Completion
- Provide working code with strategic blanks
- Use `// TODO:` or `unimplemented!()` markers
- Include context comments explaining the goal
- Tests should guide toward correct implementation

#### 🐛 Bug Fixing
- Include 2-4 intentional bugs (compilation + logic errors)
- Bugs should teach common mistakes
- Provide failing tests that pass when fixed
- Error messages should guide toward solutions

#### 🏗️ From Scratch
- Provide clear specification and requirements
- Include comprehensive test suite
- Start simple, build complexity gradually
- Multiple valid approaches acceptable

#### 👀 Code Review
- Provide working but non-idiomatic code
- Focus on Rust best practices and patterns
- Include performance or safety improvements
- Explain trade-offs in different approaches

#### ⚡ Performance Challenges
- Include baseline implementation
- Provide benchmarking framework
- Set reasonable performance targets
- Explain optimization techniques used

## 🧪 Testing Guidelines

### Running Tests
```bash
# Test all exercises
./scripts/validate-exercises.sh

# Test specific exercise
cd exercises/ch03_common_concepts/ex02_data_types
cargo test

# Check code quality
cargo clippy
cargo fmt --check
```

### Test Categories

#### Functional Tests (`tests/unit_tests.rs`)
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

#[test]
fn test_function_returns_correct_value() {
    let result = my_function(5, 10);
    assert_eq!(result, 15, "Function should return sum of inputs");
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

#### Integration Tests
```rust
#[test]
fn test_complete_workflow() {
    // End-to-end functionality testing
}
```

## 🔄 Pull Request Process

### Before Submitting
1. **Test Your Changes**
   ```bash
   # Run all tests
   cargo test --workspace
   ./scripts/validate-exercises.sh
   
   # Check formatting and linting
   cargo fmt
   cargo clippy -- -D warnings
   ```

2. **Update Documentation**
   - Update README if adding new features
   - Add exercise to chapter index
   - Update progress tracking if needed

3. **Self-Review Checklist**
   - [ ] Code follows project conventions
   - [ ] All tests pass
   - [ ] Documentation is updated
   - [ ] Commit messages are clear
   - [ ] No debug code or TODOs left

### PR Description Template
Use the provided template to describe:
- Type of change and motivation
- Exercise details (if applicable)
- Testing performed
- Breaking changes (if any)
- Additional context

### Review Process
1. **Automated Checks**: CI runs tests and quality checks
2. **Community Review**: Other contributors provide feedback
3. **Maintainer Review**: Core team final approval
4. **Merge**: Squash merge to main branch

## 💻 Platform Development

### Web UI Development
```bash
cd web
npm install
npm run dev        # Start development server
npm run test       # Run tests
npm run build      # Production build
```

### CLI Tool Development
```bash
cd cli
cargo run -- --help                    # Test CLI commands
cargo test                             # Run CLI tests
./scripts/test-cli-integration.sh      # Integration tests
```

### Exercise Framework
```bash
cd exercise-framework
cargo test                             # Test framework
cargo run --bin validate-exercise     # Validate exercises
```

## 📋 Code Style Guidelines

### Rust Code
- Follow standard Rust conventions
- Use `cargo fmt` for formatting
- Pass all `cargo clippy` suggestions
- Prefer explicit types in educational contexts
- Add comments explaining non-obvious concepts

### JavaScript/TypeScript (Web UI)
```javascript
// Use consistent formatting
const exerciseData = {
  id: 'ch03-ex02',
  title: 'Temperature Conversion',
  difficulty: 'intermediate'
};

// Prefer async/await over promises
async function loadExercise(id) {
  const response = await fetch(`/api/exercises/${id}`);
  return response.json();
}
```

### Documentation
- Use clear, concise language
- Include code examples where helpful
- Keep beginner audience in mind
- Link to relevant Rust Book sections

## 🏷️ Issue Labels

### Type Labels
- `bug` - Something isn't working correctly
- `enhancement` - New feature or improvement
- `exercise` - Related to learning exercises
- `documentation` - Documentation improvements
- `good first issue` - Good for new contributors

### Priority Labels
- `critical` - Blocking issues
- `high` - Important improvements
- `medium` - Standard enhancements
- `low` - Nice-to-have features

### Component Labels
- `web-ui` - Web interface related
- `cli` - Command-line tool related
- `framework` - Exercise framework
- `ci-cd` - Build and deployment

## 🎉 Recognition

Contributors are recognized in:
- Repository contributors list
- Release notes for significant contributions
- Special recognition for exercise contributions
- Community spotlights for outstanding help

## 📞 Getting Help

### Community Channels
- **GitHub Discussions**: Questions and feature discussions
- **Issues**: Bug reports and specific problems
- **Discord**: Real-time chat and collaboration (link in README)

### Maintainer Contact
- Create an issue for platform-related questions
- Use discussions for broader community questions
- Tag maintainers (@username) for urgent issues

## 📜 Code of Conduct

We follow the [Rust Code of Conduct](https://www.rust-lang.org/policies/code-of-conduct). In summary:
- Be welcoming and inclusive
- Be respectful and constructive
- Focus on learning and helping others learn
- Report unacceptable behavior to maintainers

## 🙏 Thank You

Every contribution, no matter how small, helps make Rust more accessible to learners worldwide. Thank you for being part of this community! 🦀❤️