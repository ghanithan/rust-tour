#[derive(Debug, Clone)]
struct Person {
    name: String,
    age: u32,
    email: Option<String>,
}

impl Person {
    fn new(name: String, age: u32, email: String) -> Person {
        Person {
            name,
            age,
            email: Some(email),
        }
    }
    
    fn child(name: String, age: u32) -> Person {
        Person {
            name,
            age,
            email: None,
        }
    }
    
    fn adult(name: String, age: u32, email: String) -> Person {
        Person {
            name,
            age,
            email: Some(email),
        }
    }
    
    fn new_validated(name: String, age: u32, email: String) -> Result<Person, String> {
        if age > 150 {
            return Err(String::from("Age must be between 0 and 150"));
        }
        
        if !email.contains('@') {
            return Err(String::from("Email must contain @ symbol"));
        }
        
        Ok(Person {
            name,
            age,
            email: Some(email),
        })
    }
    
    fn from_name(name: String) -> Person {
        Person {
            name,
            age: 0,
            email: None,
        }
    }
    
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
    
    println!("\nBasic constructor:");
    let alice = Person::new(
        String::from("Alice Johnson"),
        25,
        String::from("alice@email.com")
    );
    println!("Person: {}", alice.display());
    
    println!("\nChild constructor:");
    let bobby = Person::child(String::from("Bobby Smith"), 8);
    println!("Person: {}", bobby.display());
    
    println!("\nAdult constructor:");
    let carol = Person::adult(
        String::from("Carol Davis"),
        30,
        String::from("carol@email.com")
    );
    println!("Person: {}", carol.display());
    
    println!("\nValidation constructor (valid):");
    match Person::new_validated(
        String::from("David Wilson"),
        45,
        String::from("david@email.com")
    ) {
        Ok(person) => println!("Person: {}", person.display()),
        Err(error) => println!("Error creating person: {}", error),
    }
    
    println!("\nValidation constructor (invalid age):");
    match Person::new_validated(
        String::from("Invalid Person"),
        200,
        String::from("invalid@email.com")
    ) {
        Ok(person) => println!("Person: {}", person.display()),
        Err(error) => println!("Error creating person: {}", error),
    }
    
    println!("\nValidation constructor (invalid email):");
    match Person::new_validated(
        String::from("Another Invalid"),
        25,
        String::from("invalid-email")
    ) {
        Ok(person) => println!("Person: {}", person.display()),
        Err(error) => println!("Error creating person: {}", error),
    }
    
    println!("\nFrom name constructor:");
    let eve = Person::from_name(String::from("Eve Brown"));
    println!("Person: {}", eve.display());
    
    println!("\nTesting methods on created instances...");
    println!("Alice is an adult: {}", alice.is_adult());
    println!("Bobby is a child: {}", bobby.is_child());
    println!("Carol can vote: {}", carol.can_vote());
    
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