// TODO: Define a Student struct with:
// - name: String
// - age: u32  
// - grade: f64
// - courses: Vec<String>

// TODO: Implement methods for Student:
// - new(name, age) -> Student (creates with empty courses and grade 0.0)
// - add_course(&mut self, course: String)
// - set_grade(&mut self, grade: f64)
// - gpa(&self) -> f64 (returns the grade)
// - is_passing(&self) -> bool (grade >= 60.0)
// - course_count(&self) -> usize

// TODO: Implement associated functions:
// - honor_student(name: String, age: u32) -> Student (creates with grade 95.0)
// - from_name(name: String) -> Student (creates with age 18)

// TODO: Define a StudentManager struct with:
// - students: Vec<Student>

// TODO: Implement methods for StudentManager:
// - new() -> StudentManager
// - add_student(&mut self, student: Student)
// - find_student(&self, name: &str) -> Option<&Student>
// - average_grade(&self) -> f64
// - passing_students(&self) -> Vec<&Student>

fn main() {
    println!("Creating Student Manager System...");
    
    // TODO: Create a new student manager
    
    // TODO: Create students using different constructors
    
    // TODO: Add courses and grades to students
    
    // TODO: Add students to the manager
    
    // TODO: Test finding students
    
    // TODO: Calculate and display average grade
    
    // TODO: Show passing students
    
    println!("Student management system demo completed!");
}

#[cfg(test)]
mod tests {
    use ex08_student_manager::*;

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
}