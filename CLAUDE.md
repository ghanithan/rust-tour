# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview
This is a Rust learning platform that provides progressive, exercise-based education following "The Rust Programming Language" book structure. The platform uses a hybrid approach with GitHub Repository + Codespaces as the primary delivery method.

## Development Commands

### Project Setup
```bash
# Initialize the project structure
./scripts/init.sh

# Set up development container (if using Codespaces)
# The .devcontainer/ configuration handles this automatically
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
./scripts/run-tests.sh

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