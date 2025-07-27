# Exercise 1.5: Understanding Rust Editions

Learn about Rust's edition system and how it enables backward-compatible language evolution while introducing new features and improvements.

## 🎯 Learning Objectives

By completing this exercise, you will:
- Understand what Rust editions are and why they exist
- Learn about the differences between Rust 2015, 2018, and 2021 editions  
- Know how to configure the edition in `Cargo.toml`
- Understand backward compatibility guarantees
- See examples of edition-specific features
- Learn how to migrate between editions

## 📚 Background

Rust editions are a way to introduce breaking changes to the language while maintaining backward compatibility. Each edition represents a set of language features and changes that are "locked in" together. Code written for one edition will continue to compile with future versions of the Rust compiler.

### The Three Editions

**Rust 2015** (Original)
- The original Rust 1.0 release
- More verbose syntax in some areas
- Some features require fully qualified syntax

**Rust 2018** 
- Introduced the module system improvements
- Path clarity with `use` statements
- `async`/`await` syntax preparation
- Raw identifiers with `r#` prefix

**Rust 2021**
- Disjoint captures in closures
- IntoIterator for arrays
- New reserved keywords (`async`, `await`)
- Panic macro consistency

## 📖 Rust Book References

Read these sections:
- [Appendix E: Editions](https://doc.rust-lang.org/book/appendix-05-editions.html) - Essential reading
- [Chapter 1.3: Hello, Cargo!](https://doc.rust-lang.org/book/ch01-03-hello-cargo.html) - Cargo.toml configuration

## ✅ Your Task

Complete the exercise by:

1. **Configuring different editions** in the provided Cargo.toml examples
2. **Understanding edition differences** through code examples
3. **Completing TODO markers** in the source code to demonstrate edition features
4. **Learning migration concepts** between editions

The program demonstrates:
- Edition configuration in Cargo.toml
- Features available in different editions
- Backward compatibility principles
- Best practices for edition usage

## 🧪 Testing Your Solution

Run your program:
```bash
cargo run
```

Test the different editions (you'll modify configurations):
```bash
# Test with different edition settings
cargo check
```

Run the tests:
```bash
cargo test
```

Generate documentation:
```bash
cargo doc --no-deps
```

## 💡 Hints Available

1. **Conceptual**: Understanding what editions are and why they matter
2. **Strategic**: How to configure editions and use edition-specific features
3. **Implementation**: Specific code examples and configuration details

## 🌟 Key Concepts

### What Are Editions?

Editions allow Rust to:
- **Evolve the language** without breaking existing code
- **Group related changes** together in a coherent package
- **Provide migration paths** for new features
- **Maintain stability** while enabling innovation

### Edition Configuration

In `Cargo.toml`:
```toml
[package]
name = "my_project"
version = "0.1.0"
edition = "2021"  # Choose your edition
```

### Backward Compatibility

- **Code written in Rust 2015** still compiles today
- **Dependencies can use different editions** in the same project
- **No forced upgrades** - you choose when to migrate
- **Tooling support** helps with migration

### Edition Differences

**Module System (2018)**
```rust
// 2015 style
extern crate serde;
use serde::Serialize;

// 2018 style (cleaner)
use serde::Serialize;  // No extern crate needed
```

**Array IntoIterator (2021)**
```rust
// 2018 and earlier
for item in array.iter() { }

// 2021 (also works)
for item in array { }  // Direct iteration
```

### Migration Best Practices

1. **Test thoroughly** before migrating
2. **Use `cargo fix --edition`** for automated migration
3. **Update one edition at a time** (2015 → 2018 → 2021)
4. **Check dependencies** for edition compatibility
5. **Review new features** available in target edition

### When to Upgrade Editions

**Good reasons to upgrade:**
- Access to new language features
- Improved ergonomics and clarity
- Better tooling support
- Project-wide consistency

**When to stay on older edition:**
- Large codebase with many dependencies
- Strict stability requirements
- Team not ready for new features
- Legacy system constraints

## ⏭️ What's Next

After understanding Rust editions, you'll create your first substantial Rust program - a calculator that demonstrates variables, functions, user input, and error handling!