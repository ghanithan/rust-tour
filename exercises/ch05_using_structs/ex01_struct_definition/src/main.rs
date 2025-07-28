// TODO: Define a User struct with the following fields:
// - email: String
// - username: String  
// - active: bool
// - sign_in_count: u64

fn main() {
    println!("Creating users...");
    
    // TODO: Create a user instance for Alice
    // email: "alice@example.com"
    // username: "Alice Smith" 
    // active: true
    // sign_in_count: 1
    
    // TODO: Create a user instance for Bob  
    // email: "bob@example.com"
    // username: "Bob Jones"
    // active: false
    // sign_in_count: 0
    
    // TODO: Print user information using the display_user function
    // display_user(&alice);
    // display_user(&bob);
    
    println!("After transferring alice...");
    
    // TODO: Move alice to a new variable called transferred_user
    // let transferred_user = alice;
    
    // TODO: Access bob's email and print it
    // println!("Bob's email: {}", bob.email);
    
    // TODO: Try to access alice.email (this should cause a compile error)
    // Uncomment the line below to see the ownership error:
    // println!("Alice's email: {}", alice.email);
    
    println!("Alice was moved and is no longer accessible");
}

// TODO: Implement this function to display user information
// It should take a reference to a User and print:
// "User: email (username) - Active: active, Sign-in count: sign_in_count"
fn display_user(user: &User) {
    println!(
        "User: {} ({}) - Active: {}, Sign-in count: {}",
        user.email, user.username, user.active, user.sign_in_count
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_creation() {
        let user = User {
            email: String::from("test@example.com"),
            username: String::from("Test User"),
            active: true,
            sign_in_count: 5,
        };
        
        assert_eq!(user.email, "test@example.com");
        assert_eq!(user.username, "Test User");
        assert_eq!(user.active, true);
        assert_eq!(user.sign_in_count, 5);
    }

    #[test]
    fn test_field_access() {
        let mut user = User {
            email: String::from("mutable@example.com"),
            username: String::from("Mutable User"),
            active: false,
            sign_in_count: 0,
        };
        
        // Test field modification
        user.active = true;
        user.sign_in_count = 1;
        
        assert_eq!(user.active, true);
        assert_eq!(user.sign_in_count, 1);
    }
}