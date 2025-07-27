use ex08_student_manager::*;
use std::process::Command;

#[test]
fn test_student_creation_and_basic_methods() {
    // Test basic constructor
    let student = Student::new(String::from("Alice"), 20);
    assert_eq!(student.name, "Alice");
    assert_eq!(student.age, 20);
    assert_eq!(student.grade, 0.0);
    assert_eq!(student.courses.len(), 0);
    assert_eq!(student.gpa(), 0.0);
    assert!(!student.is_passing());
    assert_eq!(student.course_count(), 0);
}

#[test]
fn test_honor_student_constructor() {
    let student = Student::honor_student(String::from("Bob"), 19);
    assert_eq!(student.name, "Bob");
    assert_eq!(student.age, 19);
    assert_eq!(student.grade, 95.0);
    assert_eq!(student.gpa(), 95.0);
    assert!(student.is_passing());
}

#[test]
fn test_from_name_constructor() {
    let student = Student::from_name(String::from("Charlie"));
    assert_eq!(student.name, "Charlie");
    assert_eq!(student.age, 18);
    assert_eq!(student.grade, 0.0);
}

#[test]
fn test_student_course_management() {
    let mut student = Student::new(String::from("Diana"), 21);
    
    // Initially no courses
    assert_eq!(student.course_count(), 0);
    
    // Add courses
    student.add_course(String::from("Math"));
    assert_eq!(student.course_count(), 1);
    
    student.add_course(String::from("Physics"));
    student.add_course(String::from("Chemistry"));
    assert_eq!(student.course_count(), 3);
    
    // Verify courses are stored
    assert!(student.courses.contains(&String::from("Math")));
    assert!(student.courses.contains(&String::from("Physics")));
    assert!(student.courses.contains(&String::from("Chemistry")));
}

#[test]
fn test_student_grade_management() {
    let mut student = Student::new(String::from("Eve"), 22);
    
    // Initial grade
    assert_eq!(student.gpa(), 0.0);
    assert!(!student.is_passing());
    
    // Set passing grade
    student.set_grade(75.5);
    assert_eq!(student.gpa(), 75.5);
    assert!(student.is_passing());
    
    // Set failing grade
    student.set_grade(45.0);
    assert_eq!(student.gpa(), 45.0);
    assert!(!student.is_passing());
    
    // Test edge case - exactly passing
    student.set_grade(60.0);
    assert!(student.is_passing());
    
    // Test edge case - just below passing
    student.set_grade(59.9);
    assert!(!student.is_passing());
}

#[test]
fn test_student_manager_creation() {
    let manager = StudentManager::new();
    assert_eq!(manager.students.len(), 0);
}

#[test]
fn test_student_manager_add_and_find() {
    let mut manager = StudentManager::new();
    
    // Create and add students
    let alice = Student::new(String::from("Alice"), 20);
    let bob = Student::honor_student(String::from("Bob"), 19);
    
    manager.add_student(alice);
    manager.add_student(bob);
    
    assert_eq!(manager.students.len(), 2);
    
    // Test finding existing students
    let found_alice = manager.find_student("Alice");
    assert!(found_alice.is_some());
    assert_eq!(found_alice.unwrap().name, "Alice");
    assert_eq!(found_alice.unwrap().age, 20);
    
    let found_bob = manager.find_student("Bob");
    assert!(found_bob.is_some());
    assert_eq!(found_bob.unwrap().name, "Bob");
    assert_eq!(found_bob.unwrap().grade, 95.0);
    
    // Test finding non-existent student
    let not_found = manager.find_student("Charlie");
    assert!(not_found.is_none());
}

#[test]
fn test_student_manager_average_grade() {
    let mut manager = StudentManager::new();
    
    // Test empty manager
    assert_eq!(manager.average_grade(), 0.0);
    
    // Add students with known grades
    let mut alice = Student::new(String::from("Alice"), 20);
    alice.set_grade(80.0);
    
    let mut bob = Student::new(String::from("Bob"), 19);
    bob.set_grade(90.0);
    
    let mut charlie = Student::new(String::from("Charlie"), 21);
    charlie.set_grade(70.0);
    
    manager.add_student(alice);
    manager.add_student(bob);
    manager.add_student(charlie);
    
    // Average should be (80 + 90 + 70) / 3 = 80.0
    assert_eq!(manager.average_grade(), 80.0);
}

#[test]
fn test_student_manager_passing_students() {
    let mut manager = StudentManager::new();
    
    // Add mix of passing and failing students
    let mut alice = Student::new(String::from("Alice"), 20);
    alice.set_grade(85.0); // Passing
    
    let mut bob = Student::new(String::from("Bob"), 19);
    bob.set_grade(45.0); // Failing
    
    let mut charlie = Student::new(String::from("Charlie"), 21);
    charlie.set_grade(75.0); // Passing
    
    let mut diana = Student::new(String::from("Diana"), 22);
    diana.set_grade(55.0); // Failing
    
    let mut eve = Student::new(String::from("Eve"), 23);
    eve.set_grade(60.0); // Exactly passing
    
    manager.add_student(alice);
    manager.add_student(bob);
    manager.add_student(charlie);
    manager.add_student(diana);
    manager.add_student(eve);
    
    let passing = manager.passing_students();
    assert_eq!(passing.len(), 3); // Alice, Charlie, Eve
    
    // Verify the correct students are returned
    let passing_names: Vec<&str> = passing.iter().map(|s| s.name.as_str()).collect();
    assert!(passing_names.contains(&"Alice"));
    assert!(passing_names.contains(&"Charlie"));
    assert!(passing_names.contains(&"Eve"));
    assert!(!passing_names.contains(&"Bob"));
    assert!(!passing_names.contains(&"Diana"));
}

#[test]
fn test_comprehensive_student_workflow() {
    let mut manager = StudentManager::new();
    
    // Create students using different constructors
    let mut regular_student = Student::new(String::from("John"), 19);
    let honor_student = Student::honor_student(String::from("Jane"), 20);
    let default_age_student = Student::from_name(String::from("Jake"));
    
    // Modify regular student
    regular_student.add_course(String::from("Math"));
    regular_student.add_course(String::from("Science"));
    regular_student.set_grade(82.5);
    
    // Add all students to manager
    manager.add_student(regular_student);
    manager.add_student(honor_student);
    manager.add_student(default_age_student);
    
    // Test comprehensive functionality
    assert_eq!(manager.students.len(), 3);
    
    // Verify John's details
    let john = manager.find_student("John").unwrap();
    assert_eq!(john.course_count(), 2);
    assert_eq!(john.gpa(), 82.5);
    assert!(john.is_passing());
    
    // Verify Jane's details
    let jane = manager.find_student("Jane").unwrap();
    assert_eq!(jane.gpa(), 95.0);
    assert!(jane.is_passing());
    
    // Verify Jake's details
    let jake = manager.find_student("Jake").unwrap();
    assert_eq!(jake.age, 18);
    assert!(!jake.is_passing());
    
    // Test average (82.5 + 95.0 + 0.0) / 3 = 59.166...
    let avg = manager.average_grade();
    assert!((avg - 59.16666666666667).abs() < 0.0001);
    
    // Test passing students (should be John and Jane)
    let passing = manager.passing_students();
    assert_eq!(passing.len(), 2);
}

#[test]
fn test_program_runs_successfully() {
    let output = Command::new("cargo")
        .args(&["run", "--quiet"])
        .current_dir("/workspaces/rust-tour/exercises/ch05_using_structs/ex08_student_manager")
        .output()
        .expect("Failed to execute program");
    
    assert!(output.status.success(), "Program should compile and run successfully");
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Creating Student Manager System"), "Should show system creation message");
    assert!(stdout.contains("Student management system demo completed"), "Should show completion message");
}

#[test]
fn test_program_demonstrates_functionality() {
    let output = Command::new("cargo")
        .args(&["run", "--quiet"])
        .current_dir("/workspaces/rust-tour/exercises/ch05_using_structs/ex08_student_manager")
        .output()
        .expect("Failed to execute program");
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    
    // For an incomplete exercise, just check that it compiles and shows the basic messages
    // Students need to implement the functionality to pass more advanced tests
    assert!(stdout.contains("Creating Student Manager System"), 
           "Should show system creation message");
    
    assert!(stdout.contains("Student management system demo completed"), 
           "Should show completion message");
    
    // When students implement the solution, they can add tests for:
    // - Student lookup functionality
    // - Average grade calculation  
    // - Passing student identification
    // These would be checked by running cargo test on the implemented solution
}