// Reference solution for Student Manager - Comprehensive Struct Design
// This demonstrates all key struct concepts from Chapter 5 of The Rust Programming Language

#[derive(Debug, Clone)]
pub struct Student {
    pub name: String,
    pub age: u32,
    pub grade: f64,
    pub courses: Vec<String>,
}

impl Student {
    /// Creates a new student with the given name and age
    /// Initializes with empty courses and grade 0.0
    pub fn new(name: String, age: u32) -> Student {
        Student {
            name,
            age,
            grade: 0.0,
            courses: Vec::new(),
        }
    }
    
    /// Creates an honor student with high initial grade (95.0)
    pub fn honor_student(name: String, age: u32) -> Student {
        Student {
            name,
            age,
            grade: 95.0,
            courses: Vec::new(),
        }
    }
    
    /// Creates a student with default age of 18
    pub fn from_name(name: String) -> Student {
        Self::new(name, 18)
    }
    
    /// Adds a course to the student's enrollment
    pub fn add_course(&mut self, course: String) {
        self.courses.push(course);
    }
    
    /// Updates the student's grade
    pub fn set_grade(&mut self, grade: f64) {
        self.grade = grade;
    }
    
    /// Returns the student's current GPA
    pub fn gpa(&self) -> f64 {
        self.grade
    }
    
    /// Checks if the student is passing (grade >= 60.0)
    pub fn is_passing(&self) -> bool {
        self.grade >= 60.0
    }
    
    /// Returns the number of courses the student is enrolled in
    pub fn course_count(&self) -> usize {
        self.courses.len()
    }
}

#[derive(Debug)]
pub struct StudentManager {
    pub students: Vec<Student>,
}

impl StudentManager {
    /// Creates a new empty student manager
    pub fn new() -> StudentManager {
        StudentManager {
            students: Vec::new(),
        }
    }
    
    /// Adds a student to the management system
    pub fn add_student(&mut self, student: Student) {
        self.students.push(student);
    }
    
    /// Finds a student by name, returns None if not found
    pub fn find_student(&self, name: &str) -> Option<&Student> {
        self.students.iter().find(|student| student.name == name)
    }
    
    /// Calculates the average grade of all students
    /// Returns 0.0 if no students are enrolled
    pub fn average_grade(&self) -> f64 {
        if self.students.is_empty() {
            0.0
        } else {
            let total: f64 = self.students.iter().map(|s| s.grade).sum();
            total / self.students.len() as f64
        }
    }
    
    /// Returns references to all students who are passing
    pub fn passing_students(&self) -> Vec<&Student> {
        self.students.iter().filter(|s| s.is_passing()).collect()
    }
}

fn main() {
    println!("Creating Student Manager System...");
    
    // Create a new student manager
    let mut manager = StudentManager::new();
    
    // Create students using different constructors
    let mut alice = Student::new(String::from("Alice"), 20);
    let mut bob = Student::honor_student(String::from("Bob"), 19);
    let mut charlie = Student::from_name(String::from("Charlie"));
    
    // Add courses and grades to students
    alice.add_course(String::from("Mathematics"));
    alice.add_course(String::from("Physics"));
    alice.set_grade(87.5);
    
    bob.add_course(String::from("Computer Science"));
    bob.add_course(String::from("Chemistry"));
    bob.add_course(String::from("Advanced Mathematics"));
    // Bob keeps his honor student grade of 95.0
    
    charlie.add_course(String::from("History"));
    charlie.add_course(String::from("Literature"));
    charlie.set_grade(45.0); // Below passing grade
    
    // Add students to the manager
    manager.add_student(alice);
    manager.add_student(bob);
    manager.add_student(charlie);
    
    // Test finding students
    println!("\n--- Student Lookup ---");
    if let Some(student) = manager.find_student("Alice") {
        println!("Found {}, age {}, GPA: {:.1}, courses: {}", 
                student.name, student.age, student.gpa(), student.course_count());
    }
    
    if let Some(student) = manager.find_student("Bob") {
        println!("Found {}, age {}, GPA: {:.1}, courses: {}", 
                student.name, student.age, student.gpa(), student.course_count());
    }
    
    // Test non-existent student
    match manager.find_student("David") {
        Some(student) => println!("Found {}", student.name),
        None => println!("David not found in system"),
    }
    
    // Calculate and display average grade
    println!("\n--- Grade Statistics ---");
    println!("Average grade: {:.1}", manager.average_grade());
    
    // Show passing students
    let passing = manager.passing_students();
    println!("Passing students: {}", passing.len());
    for student in passing {
        println!("- {}: {:.1} ({})", student.name, student.gpa(), 
                if student.gpa() >= 90.0 { "Excellent" } 
                else if student.gpa() >= 80.0 { "Good" } 
                else { "Passing" });
    }
    
    // Show failing students
    println!("\n--- Students Needing Support ---");
    let failing: Vec<&Student> = manager.students.iter()
        .filter(|s| !s.is_passing())
        .collect();
    
    if failing.is_empty() {
        println!("All students are passing!");
    } else {
        println!("Students below passing grade:");
        for student in failing {
            println!("- {}: {:.1} (needs improvement)", student.name, student.gpa());
        }
    }
    
    println!("\nStudent management system demo completed!");
}

// Tests are included in the main.rs file for this exercise
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_student_creation() {
        let student = Student::new(String::from("Alice"), 20);
        assert_eq!(student.name, "Alice");
        assert_eq!(student.age, 20);
        assert_eq!(student.grade, 0.0);
        assert_eq!(student.courses.len(), 0);
    }

    #[test]
    fn test_student_methods() {
        let mut student = Student::new(String::from("Bob"), 19);
        student.add_course(String::from("Math"));
        student.set_grade(85.0);
        
        assert_eq!(student.course_count(), 1);
        assert_eq!(student.gpa(), 85.0);
        assert!(student.is_passing());
    }

    #[test]
    fn test_student_manager() {
        let mut manager = StudentManager::new();
        let student = Student::new(String::from("Charlie"), 21);
        
        manager.add_student(student);
        
        assert!(manager.find_student("Charlie").is_some());
        assert!(manager.find_student("David").is_none());
    }
    
    #[test]
    fn test_honor_student() {
        let student = Student::honor_student(String::from("Jane"), 20);
        assert_eq!(student.grade, 95.0);
        assert!(student.is_passing());
    }
    
    #[test]
    fn test_from_name() {
        let student = Student::from_name(String::from("John"));
        assert_eq!(student.age, 18);
        assert_eq!(student.name, "John");
    }
    
    #[test]
    fn test_average_grade() {
        let mut manager = StudentManager::new();
        
        let mut s1 = Student::new(String::from("A"), 20);
        s1.set_grade(80.0);
        let mut s2 = Student::new(String::from("B"), 21);
        s2.set_grade(90.0);
        
        manager.add_student(s1);
        manager.add_student(s2);
        
        assert_eq!(manager.average_grade(), 85.0);
    }
    
    #[test]
    fn test_passing_students() {
        let mut manager = StudentManager::new();
        
        let mut passing = Student::new(String::from("Pass"), 20);
        passing.set_grade(70.0);
        let mut failing = Student::new(String::from("Fail"), 21);
        failing.set_grade(50.0);
        
        manager.add_student(passing);
        manager.add_student(failing);
        
        let passing_list = manager.passing_students();
        assert_eq!(passing_list.len(), 1);
        assert_eq!(passing_list[0].name, "Pass");
    }
}