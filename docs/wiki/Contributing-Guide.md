# Contributing Guide

Thank you for your interest in contributing to Rust Tour! This guide will help you understand how to contribute effectively to our interactive Rust learning platform.

## Ways to Contribute

### 1. Exercise Creation
- **New exercises** for existing chapters
- **Additional chapters** for advanced topics
- **Alternative solutions** to existing exercises
- **Improved hints** and explanations

### 2. Platform Improvements
- **Web interface** enhancements
- **Performance optimizations**
- **User experience** improvements
- **Accessibility** features

### 3. Documentation
- **Tutorial content** and guides
- **API documentation**
- **Learning resources** and references
- **Translation** to other languages

### 4. Community Support
- **Answer questions** in Discussions
- **Review pull requests**
- **Report bugs** and issues
- **Share feedback** and suggestions

## Getting Started

### Development Environment Setup

1. **Fork and clone**
   ```bash
   # Fork the repository on GitHub
   git clone https://github.com/your-username/rust-tour.git
   cd rust-tour
   ```

2. **Set up development environment**
   ```bash
   # Install dependencies and set up the project
   ./scripts/run.sh setup
   ```

3. **Verify setup**
   ```bash
   # Run tests to ensure everything works
   cargo test --workspace
   cd web && npm test
   ```

4. **Start development servers**
   ```bash
   # Start both Rust server and web dev server
   ./scripts/run.sh dev
   ```

### Code Quality Standards

All contributions must meet these standards:

```bash
# Rust code formatting
cargo fmt --all

# Rust linting (must pass without warnings)
cargo clippy --all-targets --all-features -- -D warnings

# Web code linting
cd web && npm run lint

# Run all tests
cargo test --workspace
```

## Exercise Contribution Guidelines

### Exercise Structure Requirements

Every exercise must include:

```
exercises/chXX_topic/exYY_exercise_name/
├── Cargo.toml          # Dependencies and metadata
├── README.md           # Description and learning objectives
├── metadata.json       # Exercise configuration
├── src/
│   └── main.rs         # Student implementation area
├── tests/
│   └── unit_tests.rs   # Functional tests
├── hints.md            # Progressive hint system
└── solutions/
    ├── reference.rs    # Primary solution
    └── explained.md    # Solution explanation
```

### Creating a New Exercise

1. **Choose the location**
   ```bash
   # Create exercise directory
   mkdir -p exercises/ch04_understanding_ownership/ex06_borrowing_practice
   cd exercises/ch04_understanding_ownership/ex06_borrowing_practice
   ```

2. **Initialize Cargo project**
   ```bash
   cargo init
   ```

3. **Create required files**
   ```bash
   mkdir tests solutions
   touch README.md metadata.json hints.md
   touch tests/unit_tests.rs solutions/reference.rs solutions/explained.md
   ```

4. **Follow the templates** (see Exercise Templates section below)

### Exercise Design Principles

#### Educational Focus
- **Single concept per exercise** - focus on one main learning objective
- **Progressive difficulty** - build on previous concepts gradually
- **Real-world relevance** - use practical examples and scenarios

#### Technical Requirements
- **Functional tests only** - test behavior, not implementation details
- **Clear specifications** - unambiguous requirements and expected outcomes
- **Multiple solutions supported** - allow different valid approaches
- **Performance considerations** - exercises should complete quickly

#### Hint System Design

```markdown
# Hints for Exercise Name

## Level 1: Conceptual Hint
High-level explanation of the concepts involved.

## Level 2: Strategic Hint
Implementation approach and methodology.

## Level 3: Implementation Hint
Near-complete solution with detailed explanation.
```

### Exercise Templates

#### README.md Template

```markdown
# Exercise: [Exercise Name]

## Learning Objectives
- Understand [concept 1]
- Practice [skill 2]
- Apply [technique 3]

## Problem Description
[Clear description of what needs to be implemented]

## Requirements
- [ ] Implement [specific requirement 1]
- [ ] Handle [specific requirement 2]
- [ ] Ensure [specific requirement 3]

## Examples

```rust
// Input example
let input = "example";

// Expected output
let output = "expected_result";
```

## Testing
Run `cargo test` to validate your solution.

## Resources
- [Rust Book Chapter X.Y](https://doc.rust-lang.org/book/...)
- [Rust by Example: Topic](https://doc.rust-lang.org/rust-by-example/...)
```

#### metadata.json Template

```json
{
  "id": "ch04-ex06-borrowing-practice",
  "title": "Borrowing Practice",
  "description": "Practice borrowing and references in Rust",
  "chapter": 4,
  "exercise_number": 6,
  "difficulty": "intermediate",
  "estimated_time_minutes": 20,
  "concepts": ["borrowing", "references", "ownership"],
  "prerequisites": ["ch04-ex01", "ch04-ex05"],
  "exercise_type": "code_completion",
  "rust_book_refs": {
    "primary_chapter": "4.2",
    "supporting_chapters": ["4.1"]
  },
  "hints": {
    "available": 3,
    "auto_unlock": false
  },
  "testing": {
    "timeout_seconds": 10,
    "memory_limit_mb": 50
  }
}
```

#### Unit Test Template

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_functionality() {
        let result = your_function("input");
        assert_eq!(result, "expected_output");
    }

    #[test]
    fn test_edge_cases() {
        // Test boundary conditions
        let result = your_function("");
        assert_eq!(result, "");
    }

    #[test]
    fn test_error_handling() {
        // Test error conditions if applicable
        let result = your_function_that_can_fail("invalid");
        assert!(result.is_err());
    }
}
```

## Platform Contribution Guidelines

### Web Interface Development

#### Technology Stack
- **Frontend**: TypeScript, React, Vite
- **Editor**: Monaco Editor (VS Code editor)
- **Styling**: CSS Modules
- **State Management**: React Context/Hooks

#### Development Workflow

```bash
# Start development server
cd web && npm run dev

# Run type checking
npm run type-check

# Run tests
npm test

# Build for production
npm run build
```

### Backend Development

#### Technology Stack
- **Server**: Rust with Axum web framework
- **Exercise Framework**: Custom Rust library
- **Progress Tracking**: JSON file storage
- **Terminal Emulation**: Web-based terminal

#### Key Components
- **Web Server** (`web-server/`): HTTP API and static file serving
- **Exercise Framework** (`exercise-framework/`): Exercise loading and validation
- **Progress Tracking**: Learning analytics and achievement system

## Pull Request Process

### Before Submitting

1. **Test thoroughly**
   ```bash
   # Run all tests
   ./scripts/run.sh test
   cargo test --workspace
   cd web && npm test
   ```

2. **Check code quality**
   ```bash
   cargo clippy --all-targets --all-features -- -D warnings
   cargo fmt --all -- --check
   cd web && npm run lint
   ```

3. **Update documentation** if needed
4. **Test your changes** in the browser interface

### Pull Request Template

```markdown
## Description
[Brief description of changes]

## Type of Change
- [ ] New exercise
- [ ] Bug fix
- [ ] Feature enhancement
- [ ] Documentation update
- [ ] Performance improvement

## Exercise Details (if applicable)
- **Chapter**: [X]
- **Exercise Number**: [Y]
- **Concepts Covered**: [list]
- **Difficulty Level**: [beginner/intermediate/advanced]

## Testing
- [ ] All existing tests pass
- [ ] New tests added for new functionality
- [ ] Tested in web interface
- [ ] Code quality checks pass

## Checklist
- [ ] Self-reviewed the code
- [ ] Updated documentation
- [ ] Added appropriate comments
- [ ] Follows project conventions
```

### Review Process

1. **Automated checks** run on all PRs
2. **Maintainer review** for code quality and design
3. **Educational review** for exercise content and pedagogy
4. **Community feedback** period for significant changes
5. **Final approval** and merge

## Community Guidelines

### Code of Conduct
- **Be respectful** in all interactions
- **Be constructive** in feedback and criticism
- **Be inclusive** and welcoming to newcomers
- **Be collaborative** and open to different perspectives

### Communication Channels
- **GitHub Issues**: Bug reports and feature requests
- **GitHub Discussions**: Questions, ideas, and community chat
- **Pull Request Reviews**: Code-specific discussions
- **Wiki**: Documentation contributions

### Recognition
Contributors are recognized through:
- **Contributors file** listing all contributors
- **Release notes** highlighting significant contributions
- **Community showcase** featuring outstanding contributions

## Advanced Contribution Topics

### Exercise Framework Extension
For developers wanting to extend the core framework:

```rust
// Example: Adding a new exercise type
pub enum ExerciseType {
    CodeCompletion,
    BugFixing,
    FromScratch,
    CodeReview,
    Performance,
    YourNewType,  // Add new types here
}
```

### Performance Optimization
- **Compilation speed**: Minimize dependency chains
- **Runtime performance**: Profile exercise execution
- **Memory usage**: Monitor resource consumption
- **Load times**: Optimize asset delivery

### Internationalization
Guidelines for translating content:
- **Exercise descriptions**: Clear, culturally neutral language
- **Error messages**: Maintain technical accuracy
- **UI elements**: Consider text expansion
- **Code comments**: Keep technical terms in English

## Getting Help

### For Contributors
- **Technical questions**: Ask in [Discussions](https://github.com/ghanithan/rust-tour/discussions)
- **Design discussions**: Open an issue for significant changes
- **Exercise ideas**: Use the Ideas category in Discussions

### For Maintainers
- **Review guidelines**: See maintainer documentation
- **Release process**: Check release checklist
- **Community management**: Refer to moderation guidelines

## Recognition and Thanks

We deeply appreciate all forms of contribution, from fixing typos to creating comprehensive new chapters. Every contribution helps make Rust more accessible to learners worldwide.

Your contributions will be:
- **Credited** in the contributors list
- **Highlighted** in release notes
- **Showcased** in community updates
- **Appreciated** by the entire Rust learning community

Thank you for helping make Rust Tour an exceptional learning platform!