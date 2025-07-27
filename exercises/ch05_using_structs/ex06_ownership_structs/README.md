# Exercise 6: Ownership Patterns with Structs

## Learning Objectives

In this exercise, you will learn how to:
- Understand ownership rules when working with struct fields
- Recognize and fix partial move issues
- Use borrowing vs. cloning appropriately
- Write functions that work correctly with references
- Handle ownership transfer between variables

## Exercise Description

This is a **bug fixing** exercise focused on common ownership problems when working with structs. The provided code contains several intentional ownership violations that prevent compilation. Your task is to fix these issues while maintaining the program's intended functionality.

The code demonstrates a simple library management system with `Book` and `Library` structs. The bugs represent real-world ownership challenges that Rust developers frequently encounter.

## Instructions

1. Run `cargo check` to see the compilation errors
2. Read through the code and identify ownership issues
3. Fix each bug while preserving the program's logic
4. Use the hint system if you get stuck (3 levels available)
5. Ensure all tests pass with `cargo test`

## Key Concepts Covered

- **Partial moves**: What happens when you move a field out of a struct
- **Field borrowing**: How to access struct fields without taking ownership
- **Function parameters**: When to use references vs. owned values
- **Clone vs. Move**: Understanding when to copy data vs. transfer ownership
- **Return types**: Returning references from functions safely

## Expected Behavior

After fixing the bugs, the program should:
- Welcome users to the City Library
- Display information about books without consuming the library
- Handle book operations without ownership conflicts
- Complete successfully with "Books processed successfully!" message

## Rust Book References

- [Chapter 4.1: What is Ownership?](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html)
- [Chapter 4.2: References and Borrowing](https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html)
- [Chapter 5.1: Defining and Instantiating Structs](https://doc.rust-lang.org/book/ch05-01-defining-structs.html)

## Common Mistakes to Avoid

1. **Moving when you should borrow**: Use `&` to create references
2. **Forgetting to derive Clone**: Add `#[derive(Clone)]` when needed
3. **Taking ownership in display functions**: Functions that just read data should take references
4. **Returning owned values from borrowed data**: Return references or use Option types

## Success Criteria

- [ ] Code compiles without errors (`cargo check`)
- [ ] All tests pass (`cargo test`)
- [ ] Program runs and produces expected output
- [ ] No clippy warnings (`cargo clippy`)

**Estimated Time**: 25 minutes  
**Difficulty**: Intermediate