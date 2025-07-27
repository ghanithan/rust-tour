# FAQ & Troubleshooting

Find quick answers to common questions and solutions to frequent issues with Rust Tour.

## Frequently Asked Questions

### General Questions

#### Q: What is Rust Tour?
**A:** Rust Tour is an interactive learning platform that teaches Rust programming through hands-on exercises aligned with "The Rust Programming Language" book. It provides a structured, test-driven approach to mastering Rust concepts.

#### Q: Do I need prior programming experience?
**A:** While some programming experience is helpful, Rust Tour is designed to be accessible to beginners. The exercises start with basic concepts and gradually build complexity. If you're completely new to programming, consider reviewing basic programming concepts first.

#### Q: How long does it take to complete Rust Tour?
**A:** This varies greatly depending on your background and study pace. Most learners complete the core curriculum in 2-8 weeks with consistent daily practice (1-2 hours per day).

#### Q: Is Rust Tour free?
**A:** Yes, Rust Tour is completely free and open-source. You can use it without any cost or registration requirements.

#### Q: Can I use Rust Tour offline?
**A:** Partially. The `cargo install rust-tour` method provides offline capability once installed. However, the GitHub Codespaces and web-based versions require internet connectivity.

### Technical Questions

#### Q: Which installation method should I choose?
**A:** 
- **Beginners**: GitHub Codespaces (no setup required)
- **Local development**: Cargo installation (simplest)
- **Offline use**: Local repository clone
- **Contributors**: Development setup with full toolchain

#### Q: Can I save my progress?
**A:** Yes, if you fork the repository and use GitHub Codespaces, or if you use any local installation method. Progress is saved automatically in JSON format.

#### Q: What if I get stuck on an exercise?
**A:** 
1. Use the progressive hint system (3 levels)
2. Review relevant Rust Book sections
3. Check similar exercises for patterns
4. Ask specific questions in [GitHub Discussions](https://github.com/ghanithan/rust-tour/discussions)

#### Q: Can I skip exercises?
**A:** While technically possible, it's not recommended. Each exercise builds on concepts from previous ones. If you're struggling, use hints or ask for help rather than skipping.

#### Q: How do I know if my solution is correct?
**A:** Run `cargo test` in the exercise directory. All tests should pass. Additionally, run `cargo clippy` to check for code quality issues.

#### Q: Can I see reference solutions?
**A:** Yes, most exercises include reference solutions in the `solutions/` directory. However, try solving exercises independently first for maximum learning benefit.

## Troubleshooting

### Installation Issues

#### Problem: `./scripts/run.sh` command not found
**Solution:**
```bash
# Make sure you're in the project root directory
pwd  # Should show path ending in /rust-tour

# Make scripts executable
chmod +x scripts/*.sh

# Try again
./scripts/run.sh start
```

#### Problem: Cargo installation fails
**Symptoms:** `cargo install rust-tour` returns errors
**Solutions:**
```bash
# Update Rust toolchain
rustup update stable

# Clear cargo cache and retry
cargo clean
cargo install rust-tour

# If still failing, install from source
git clone https://github.com/ghanithan/rust-tour.git
cd rust-tour
cargo install --path .
```

#### Problem: Node.js/npm errors during setup
**Solutions:**
```bash
# Check Node.js version (should be 18+)
node --version

# Clear npm cache
npm cache clean --force

# Delete node_modules and reinstall
rm -rf web/node_modules
cd web && npm install
```

#### Problem: Docker container won't start
**Solutions:**
```bash
# Check Docker is running
docker --version
docker ps

# Restart Docker service
sudo systemctl restart docker  # Linux
# or restart Docker Desktop app

# Clean up and retry
docker-compose down
docker-compose up -d
```

### Runtime Issues

#### Problem: Port 3000 is already in use
**Symptoms:** "Address already in use" or "Port 3000 is already in use"
**Solutions:**
```bash
# Find what's using port 3000
lsof -i :3000  # macOS/Linux
netstat -ano | findstr :3000  # Windows

# Kill the process
kill -9 <PID>  # Replace <PID> with actual process ID

# Or use a different port
RUST_TOUR_PORT=3001 ./scripts/run.sh start
```

#### Problem: Tests fail but solution looks correct
**Solutions:**
```bash
# Run tests with verbose output
cargo test -- --nocapture

# Check for hidden whitespace/formatting issues
cargo fmt

# Review test requirements carefully
cat tests/unit_tests.rs

# Look at expected vs actual output in test failures
```

#### Problem: Clippy warnings prevent progress
**Solutions:**
```bash
# See specific warnings
cargo clippy

# Fix common issues automatically
cargo clippy --fix

# Some warnings can be temporarily allowed
#[allow(clippy::warning_name)]
```

#### Problem: Web interface not loading
**Symptoms:** Browser shows connection refused or timeout
**Solutions:**
```bash
# Check if server is running
curl http://localhost:3000/health

# Check server logs
./scripts/run.sh start  # Look for error messages

# Try different browser or incognito mode
# Clear browser cache and cookies

# For Codespaces, check port forwarding settings
```

### Exercise-Specific Issues

#### Problem: Compilation errors in skeleton code
**Symptoms:** Exercise won't compile even before making changes
**Solutions:**
- This is often intentional for bug-fixing exercises
- Read the exercise README for specific instructions
- Look for TODO comments indicating where to make changes
- Check if the exercise type is "bug_fixing"

#### Problem: Tests pass but program doesn't run
**Solutions:**
```bash
# Build and run the program
cargo run

# Check if main.rs has a main function
cat src/main.rs

# Some exercises are libraries (lib.rs) not binaries
cargo test  # This is the primary way to validate
```

#### Problem: Can't understand exercise requirements
**Solutions:**
1. Read the README.md carefully
2. Examine the test cases to understand expected behavior
3. Check metadata.json for additional context
4. Use Level 1 hints for conceptual guidance
5. Ask in [Discussions](https://github.com/ghanithan/rust-tour/discussions) with specific questions

### Performance Issues

#### Problem: Exercises run slowly
**Solutions:**
```bash
# Use release mode for performance exercises
cargo test --release

# Check available system resources
# Close unnecessary applications

# For large exercises, consider using:
cargo test --jobs 1  # Reduce parallelism
```

#### Problem: High memory usage
**Solutions:**
- Close other applications while using Rust Tour
- Use local installation instead of Codespaces for resource-intensive exercises
- Monitor system resources and restart if needed

### Codespaces-Specific Issues

#### Problem: Codespace keeps stopping
**Solutions:**
- Codespaces auto-stop after inactivity
- Save work frequently
- Use forked repository to persist changes
- Consider upgrading to paid GitHub plan for longer runtime

#### Problem: Port forwarding not working
**Solutions:**
- Check the "Ports" tab in VS Code
- Make port 3000 public if needed
- Try refreshing the forwarded URL
- Restart the Codespace if necessary

### Git and Progress Issues

#### Problem: Progress not saving
**Solutions:**
```bash
# Check git status
git status

# Commit your progress
git add .
git commit -m "Progress update"

# Push to your fork (if using GitHub)
git push origin main
```

#### Problem: Merge conflicts when updating
**Solutions:**
```bash
# Stash your changes
git stash

# Pull latest changes
git pull upstream main

# Apply your changes back
git stash pop

# Resolve any conflicts manually
```

## Getting Additional Help

### When to Use Each Support Channel

#### GitHub Issues
Use for:
- **Bug reports** with specific reproduction steps
- **Feature requests** with detailed descriptions
- **Installation problems** not covered in troubleshooting

#### GitHub Discussions
Use for:
- **Exercise help** and learning questions
- **General Rust questions** related to the curriculum
- **Ideas and suggestions** for improvement
- **Community interaction** and sharing progress

#### Wiki
Use for:
- **Comprehensive guides** and documentation
- **Reference material** for common tasks
- **Detailed tutorials** and explanations

### Creating Effective Bug Reports

Include this information:
```markdown
**Environment:**
- OS: [Linux/macOS/Windows]
- Installation method: [Codespaces/Docker/Local/Cargo]
- Rust version: `rustc --version`
- Browser: [if web-related]

**Expected Behavior:**
[What should happen]

**Actual Behavior:**
[What actually happens]

**Steps to Reproduce:**
1. [First step]
2. [Second step]
3. [Result]

**Additional Context:**
[Screenshots, error messages, logs]
```

### Asking Good Questions

#### Before Asking
1. **Search existing discussions** for similar questions
2. **Try the troubleshooting steps** above
3. **Check the relevant wiki pages**
4. **Review exercise hints** if applicable

#### When Asking
- **Be specific** about which exercise or feature
- **Include error messages** exactly as they appear
- **Describe what you've already tried**
- **Share relevant code** using proper formatting

#### Example Good Question
```markdown
**Exercise:** Chapter 4, Exercise 3 - Borrowing References

**Problem:** My code compiles but the test `test_borrow_checker` fails with:
```
assertion failed: expected "Hello", got "Hello World"
```

**My approach:** I'm trying to... [explanation]

**Code:** 
```rust
// relevant code snippet
```

**Question:** I don't understand why the borrow checker is... [specific question]
```

Remember: The community is here to help, and well-asked questions often help others with similar issues!