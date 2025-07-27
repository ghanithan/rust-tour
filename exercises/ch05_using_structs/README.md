# Chapter 5: Using Structs to Structure Related Data

This chapter teaches you how to use structs to create custom data types that group related data together and define behavior through methods.

## Learning Path

### Exercise 01: Struct Definition - Building Your First Data Structure
- **Concepts**: struct definition, instantiation, field access, ownership
- **Type**: Code Completion
- **Difficulty**: Beginner (15 min)
- Learn to define custom structs and work with their fields

### Exercise 02: Struct Update Syntax - Efficient Struct Creation  
- **Concepts**: struct update syntax, field init shorthand, ownership with updates
- **Type**: Bug Fixing
- **Difficulty**: Beginner (20 min)
- Master convenient syntax for creating structs from existing ones

### Exercise 03: Struct Methods - Adding Behavior to Data
- **Concepts**: impl blocks, self parameters, method vs function, borrowing self
- **Type**: Code Completion  
- **Difficulty**: Intermediate (25 min)
- Implement methods that operate on struct instances

### Exercise 04: Associated Functions - Constructor Patterns
- **Concepts**: associated functions, constructor patterns, static methods
- **Type**: From Scratch
- **Difficulty**: Intermediate (20 min)
- Create factory methods and constructors for structs

### Exercise 05: Tuple Structs and Unit-Like Structs
- **Concepts**: tuple structs, unit structs, newtype pattern, type safety
- **Type**: Bug Fixing
- **Difficulty**: Intermediate (20 min)
- Learn alternative struct forms and their use cases

### Exercise 06: Ownership Patterns with Structs
- **Concepts**: struct ownership, field borrowing, partial moves
- **Type**: Bug Fixing
- **Difficulty**: Intermediate (25 min)
- Understand how ownership works with struct fields

### Exercise 07: Rectangle Program - Practical Struct Example
- **Concepts**: practical structs, debug trait, area calculation
- **Type**: From Scratch
- **Difficulty**: Beginner (30 min)
- Build a complete program following the Rust Book's rectangle example

### Exercise 08: Student Manager - Comprehensive Struct Design
- **Concepts**: comprehensive design, struct composition, real-world application
- **Type**: From Scratch
- **Difficulty**: Advanced (45 min)
- Design a complete system using all struct concepts

## Key Concepts Covered

### Struct Fundamentals
- Defining structs with named fields
- Creating instances and accessing fields
- Struct ownership and borrowing patterns

### Struct Syntax Features
- Field init shorthand (`field` vs `field: field`)
- Struct update syntax (`..other_instance`)
- Different struct types (named, tuple, unit-like)

### Methods and Associated Functions
- Implementing behavior with `impl` blocks
- Different forms of `self` (`self`, `&self`, `&mut self`)
- Associated functions as constructors and factories

### Design Patterns
- Constructor patterns (`new`, `default`, `from_*`)
- Validation and error handling in constructors
- Newtype pattern for type safety
- Composition and method organization

## Prerequisites
- Chapter 4: Understanding Ownership (ownership rules, borrowing, moves)
- Chapter 3: Common Concepts (variables, data types, functions)

## Related Rust Book Sections
- [Chapter 5.1: Defining and Instantiating Structs](https://doc.rust-lang.org/book/ch05-01-defining-structs.html)
- [Chapter 5.2: An Example Program Using Structs](https://doc.rust-lang.org/book/ch05-02-example-structs.html)
- [Chapter 5.3: Method Syntax](https://doc.rust-lang.org/book/ch05-03-method-syntax.html)

## Skills You'll Gain
- Creating custom data types with structs
- Organizing related data and behavior together
- Understanding ownership implications with structs
- Implementing methods and associated functions
- Using struct syntax features for cleaner code
- Designing robust APIs with validation patterns