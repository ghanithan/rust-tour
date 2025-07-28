//! Student Manager Library
//! 
//! This library provides structs and methods for managing student records.
//! It demonstrates comprehensive struct usage including:
//! - Struct definition with multiple field types
//! - Associated functions (constructors)
//! - Instance methods for behavior
//! - Collection management with Vec
//! - Option handling for safe access

// The actual implementation will come from main.rs when students implement it
// For now, we provide the structure that allows tests to compile

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