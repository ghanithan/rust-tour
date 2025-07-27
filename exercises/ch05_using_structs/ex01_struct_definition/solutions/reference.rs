struct User {
    email: String,
    username: String, 
    active: bool,
    sign_in_count: u64,
}

fn main() {
    println!("Creating users...");
    
    let alice = User {
        email: String::from("alice@example.com"),
        username: String::from("Alice Smith"),
        active: true,
        sign_in_count: 1,
    };
    
    let bob = User {
        email: String::from("bob@example.com"),
        username: String::from("Bob Jones"),
        active: false,
        sign_in_count: 0,
    };
    
    display_user(&alice);
    display_user(&bob);
    
    println!("After transferring alice...");
    
    let transferred_user = alice;
    
    println!("Bob's email: {}", bob.email);
    
    // alice is moved and no longer accessible
    // println!("Alice's email: {}", alice.email); // This would error
    
    println!("Alice was moved and is no longer accessible");
}

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
        
        user.active = true;
        user.sign_in_count = 1;
        
        assert_eq!(user.active, true);
        assert_eq!(user.sign_in_count, 1);
    }
}