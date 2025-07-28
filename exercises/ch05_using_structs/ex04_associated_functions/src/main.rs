#[derive(Debug, Clone)]
struct Person {
    name: String,
    age: u32,
    email: Option<String>,
}

impl Person {
    // TODO: Implement the `new` associated function
    // Takes name (String), age (u32), and email (String)
    // Returns a Person with the provided values (email wrapped in Some)
    
    // TODO: Implement the `child` associated function  
    // Takes only name (String) and age (u32)
    // Returns a Person with no email (None)
    // Should only be used for ages < 18
    
    // TODO: Implement the `adult` associated function
    // Takes name (String), age (u32), and email (String)
    // Returns a Person with provided values
    // Should only be used for ages >= 18
    
    // TODO: Implement the `new_validated` associated function
    // Takes name (String), age (u32), and email (String)
    // Returns Result<Person, String>
    // Validates that age is <= 150 and email contains '@'
    // Returns error message if validation fails
    
    // TODO: Implement the `from_name` associated function
    // Takes only name (String)
    // Returns a Person with age 0 and no email
    
    // Methods (already implemented for you to use)
    fn is_adult(&self) -> bool {
        self.age >= 18
    }
    
    fn is_child(&self) -> bool {
        self.age < 18
    }
    
    fn can_vote(&self) -> bool {
        self.age >= 18
    }
    
    fn display(&self) -> String {
        match &self.email {
            Some(email) => format!("{} ({} years old, {})", self.name, self.age, email),
            None => format!("{} ({} years old, no email)", self.name, self.age),
        }
    }
}

fn main() {
    println!("Creating people with different constructors...");
    
    // TODO: Use Person::new to create Alice
    println!("\nBasic constructor:");
    let alice = Person::new(
        String::from("Alice Johnson"),
        25,
        String::from("alice@email.com")
    );
    println!("Person: {}", alice.display());
    
    // TODO: Use Person::child to create Bobby
    println!("\nChild constructor:");
    let bobby = Person::child(String::from("Bobby Smith"), 8);
    println!("Person: {}", bobby.display());
    
    // TODO: Use Person::adult to create Carol
    println!("\nAdult constructor:");
    let carol = Person::adult(
        String::from("Carol Davis"),
        30,
        String::from("carol@email.com")
    );
    println!("Person: {}", carol.display());
    
    // TODO: Use Person::new_validated to create David (should succeed)
    println!("\nValidation constructor (valid):");
    match Person::new_validated(
        String::from("David Wilson"),
        45,
        String::from("david@email.com")
    ) {
        Ok(person) => println!("Person: {}", person.display()),
        Err(error) => println!("Error creating person: {}", error),
    }
    
    // TODO: Use Person::new_validated with invalid age (should fail)
    println!("\nValidation constructor (invalid age):");
    match Person::new_validated(
        String::from("Invalid Person"),
        200,
        String::from("invalid@email.com")
    ) {
        Ok(person) => println!("Person: {}", person.display()),
        Err(error) => println!("Error creating person: {}", error),
    }
    
    // TODO: Use Person::new_validated with invalid email (should fail)
    println!("\nValidation constructor (invalid email):");
    match Person::new_validated(
        String::from("Another Invalid"),
        25,
        String::from("invalid-email")
    ) {
        Ok(person) => println!("Person: {}", person.display()),
        Err(error) => println!("Error creating person: {}", error),
    }
    
    // TODO: Use Person::from_name to create Eve
    println!("\nFrom name constructor:");
    let eve = Person::from_name(String::from("Eve Brown"));
    println!("Person: {}", eve.display());
    
    println!("\nTesting methods on created instances...");
    println!("Alice is an adult: {}", alice.is_adult());
    println!("Bobby is a child: {}", bobby.is_child());
    println!("Carol can vote: {}", carol.can_vote());
    
    // Test with the validated person if it was created successfully
    if let Ok(david) = Person::new_validated(
        String::from("David Wilson"),
        45,
        String::from("david@email.com")
    ) {
        println!("David can vote: {}", david.can_vote());
    }
    
    println!("Eve is a child: {}", eve.is_child());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_constructor() {
        let person = Person::new(
            String::from("Test Person"),
            25,
            String::from("test@example.com")
        );
        
        assert_eq!(person.name, "Test Person");
        assert_eq!(person.age, 25);
        assert_eq!(person.email, Some(String::from("test@example.com")));
    }

    #[test]
    fn test_child_constructor() {
        let child = Person::child(String::from("Child"), 10);
        
        assert_eq!(child.name, "Child");
        assert_eq!(child.age, 10);
        assert_eq!(child.email, None);
        assert!(child.is_child());
    }

    #[test]
    fn test_adult_constructor() {
        let adult = Person::adult(
            String::from("Adult"),
            25,
            String::from("adult@example.com")
        );
        
        assert_eq!(adult.name, "Adult");
        assert_eq!(adult.age, 25);
        assert_eq!(adult.email, Some(String::from("adult@example.com")));
        assert!(adult.is_adult());
    }

    #[test]
    fn test_validated_constructor_success() {
        let result = Person::new_validated(
            String::from("Valid Person"),
            30,
            String::from("valid@example.com")
        );
        
        assert!(result.is_ok());
        let person = result.unwrap();
        assert_eq!(person.name, "Valid Person");
        assert_eq!(person.age, 30);
    }

    #[test]
    fn test_validated_constructor_invalid_age() {
        let result = Person::new_validated(
            String::from("Invalid Age"),
            200,
            String::from("test@example.com")
        );
        
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Age must be"));
    }

    #[test]
    fn test_validated_constructor_invalid_email() {
        let result = Person::new_validated(
            String::from("Invalid Email"),
            25,
            String::from("invalid-email")
        );
        
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Email must contain"));
    }

    #[test]
    fn test_from_name_constructor() {
        let person = Person::from_name(String::from("Name Only"));
        
        assert_eq!(person.name, "Name Only");
        assert_eq!(person.age, 0);
        assert_eq!(person.email, None);
    }
}