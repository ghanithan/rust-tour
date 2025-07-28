# ADR-001: Remove Unused Exercise Framework

## Status
Proposed

## Context
The `exercise-framework` was created early in the project's development (commit 788c07b: "Implement core exercise framework and first sample exercises") as a sophisticated abstraction layer for managing Rust learning exercises. The framework was designed to provide:

- Advanced exercise management and loading
- Custom testing infrastructure beyond standard Rust tests
- Exercise validation and quality checks
- Progress tracking and analytics
- A three-level hint system
- Support for different exercise types (code completion, bug fixing, from scratch, etc.)

However, as the project evolved, simpler and more direct approaches were adopted:
- Exercises are standard Cargo projects with regular Rust tests
- The web server manages exercises through direct file system operations
- Progress tracking is handled via simple JSON files
- No exercises actually import or use the framework

The framework represents well-intentioned architectural planning that proved unnecessary in practice. The simpler solution has been working perfectly well.

## Decision
Remove the `exercise-framework` from the project entirely, as it represents unused technical debt that:
- Increases build times unnecessarily
- Adds complexity to the codebase without providing value
- Confuses contributors about the actual architecture
- Increases Docker image size

## Consequences

### Positive
- **Faster builds**: No need to compile unused code
- **Cleaner codebase**: Removes ~1000+ lines of unused code
- **Clearer architecture**: Contributors can understand the actual system more easily
- **Smaller artifacts**: Reduced Docker image size and binary size
- **Simplified maintenance**: Less code to maintain and keep up to date

### Negative
- None identified - the framework was never integrated or used

### Neutral
- If advanced exercise features are needed in the future, they can be built when actually required, following YAGNI (You Aren't Gonna Need It) principles

## Implementation Plan

### Files to Remove
- `/exercise-framework/` directory and all its contents
- Remove from workspace members in `/Cargo.toml`

### References to Update

#### Dockerfile
- Remove lines building `exercise-framework` (line 52, etc.)
- Remove references in build caching steps

#### Scripts
- `scripts/test-exercises.sh` - Remove test_framework function (lines 115-131)
- `scripts/setup.sh` - Remove any framework references
- `scripts/build-docker.sh` - Update build commands
- `scripts/start-platform.sh` - Check for references
- `scripts/start-production.sh` - Check for references

#### Documentation
- `README.md` - Remove exercise-framework from architecture section
- `CONTRIBUTING.md` - Remove framework testing instructions
- `docs/wiki/Contributing-Guide.md` - Update component list
- `docs/plan/PRD_Core_Platform.md` - Update internal components
- `docs/plan/DISCUSSION_SUMMARY.md` - Note this architectural evolution
- `docs/plan/TECHNICAL_DOCUMENTATION.md` - Update architecture diagram

### Migration Steps
1. Create this ADR and get approval
2. Remove `/exercise-framework/` directory
3. Update `/Cargo.toml` workspace members
4. Update all scripts and documentation
5. Test that builds still work correctly
6. Update CI/CD workflows if needed
7. Commit with clear message explaining the removal

## Alternative Considered
Keep the framework "just in case" - rejected because:
- It adds ongoing maintenance burden
- It confuses the actual architecture
- YAGNI principle - build abstractions when needed, not in advance
- The current simple approach has proven sufficient

## References
- Original framework commit: 788c07b
- Framework was planned in PRD_Exercise_Framework.md
- Current architecture works without it as shown in production usage