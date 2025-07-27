# Student Manager - Comprehensive Struct Design

## Learning Objectives

By completing this exercise, you will:
- Design and implement complex structs with multiple fields
- Create and use constructor functions (associated functions)
- Implement instance methods for struct behavior
- Work with compound data types (Vec, String, Option)
- Apply real-world struct composition patterns
- Practice ownership and borrowing in struct methods

## Exercise Description

You'll build a comprehensive student management system that demonstrates all the key struct concepts from Chapter 5 of The Rust Programming Language. This system will manage student records and provide various operations for working with student data.

## Requirements

### Part 1: Student Struct

Create a `Student` struct with the following fields:
- `name`: String - The student's name
- `age`: u32 - The student's age  
- `grade`: f64 - The student's current grade/GPA
- `courses`: Vec<String> - List of courses the student is enrolled in

### Part 2: Student Methods

Implement the following methods for the `Student` struct:

**Constructor Methods (Associated Functions):**
- `new(name: String, age: u32) -> Student` - Creates a new student with empty courses and grade 0.0
- `honor_student(name: String, age: u32) -> Student` - Creates a student with grade 95.0
- `from_name(name: String) -> Student` - Creates a student with default age 18

**Instance Methods:**
- `add_course(&mut self, course: String)` - Adds a course to the student's course list
- `set_grade(&mut self, grade: f64)` - Updates the student's grade
- `gpa(&self) -> f64` - Returns the student's current grade
- `is_passing(&self) -> bool` - Returns true if grade >= 60.0
- `course_count(&self) -> usize` - Returns the number of enrolled courses

### Part 3: StudentManager Struct

Create a `StudentManager` struct with:
- `students`: Vec<Student> - Collection of all students

### Part 4: StudentManager Methods

Implement the following methods for `StudentManager`:

- `new() -> StudentManager` - Creates a new empty student manager
- `add_student(&mut self, student: Student)` - Adds a student to the collection
- `find_student(&self, name: &str) -> Option<&Student>` - Finds a student by name
- `average_grade(&self) -> f64` - Calculates the average grade of all students
- `passing_students(&self) -> Vec<&Student>` - Returns references to all passing students

### Part 5: Main Function Implementation

In the `main()` function, demonstrate the system by:
1. Creating a new StudentManager
2. Creating students using different constructor methods
3. Adding courses and setting grades for students
4. Adding students to the manager
5. Testing the find_student functionality
6. Calculating and displaying the average grade
7. Showing all passing students

## Key Concepts Practiced

- **Struct Definition**: Creating structs with multiple field types
- **Associated Functions**: Constructor patterns and factory methods
- **Instance Methods**: Behavior implementation for structs
- **Ownership**: Working with owned vs borrowed data
- **Option Type**: Handling cases where data might not exist
- **Vector Operations**: Managing collections of structs
- **Method Chaining**: Designing methods that work well together

## Expected Output

Your program should produce output demonstrating:
- Student creation with different constructors
- Course enrollment and grade assignment
- Student lookup by name
- Average grade calculation
- Identification of passing students

## Testing

The exercise includes comprehensive tests that verify:
- Struct creation and initialization
- Method functionality and correctness
- StudentManager operations
- Edge cases and error conditions

Run tests with: `cargo test`

## Rust Book References

- [Chapter 5.1 - Defining and Instantiating Structs](https://doc.rust-lang.org/book/ch05-01-defining-structs.html)
- [Chapter 5.2 - An Example Program Using Structs](https://doc.rust-lang.org/book/ch05-02-example-structs.html)
- [Chapter 5.3 - Method Syntax](https://doc.rust-lang.org/book/ch05-03-method-syntax.html)

## Tips

- Start by defining the Student struct with all required fields
- Implement the associated functions (constructors) first
- Add instance methods one at a time, testing each
- Use `&self` for methods that don't modify the struct
- Use `&mut self` for methods that need to modify the struct
- Remember that `find_student` should return an `Option` since the student might not exist
- The `average_grade` method should handle the case of no students (return 0.0)

Good luck building your student management system!