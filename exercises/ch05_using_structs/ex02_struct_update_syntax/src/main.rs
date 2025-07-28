struct User {
    email: String,
    username: String,
    active: bool,
    sign_in_count: u64,
}

fn main() {
    println!("Creating original user...");
    
    // Create the original user
    let user1 = User {
        email: String::from("alice@example.com"),
        username: String::from("Alice Smith"),
        active: true,
        sign_in_count: 1,
    };
    
    display_user(&user1);
    
    println!("\nCreating updated user with struct update syntax...");
    
    // BUG: Fix the struct update syntax to change only the username
    // Should create a new user with username "Alice Johnson" but keep other fields
    let user2 = User {
        username: String::from("Alice Johnson"),
        user1  // This syntax is incorrect
    };
    
    display_user(&user2);
    
    println!("\nCreating user with field init shorthand...");
    
    // BUG: Use field init shorthand syntax where possible
    let email = String::from("bob@example.com");
    let username = String::from("Bob Wilson");
    let active = false;
    let sign_in_count = 0;
    
    let user3 = User {
        email: email,          // Can be shortened
        username: username,    // Can be shortened  
        active: active,        // Can be shortened
        sign_in_count: sign_in_count,  // Can be shortened
    };
    
    display_user(&user3);
    
    println!("\nCreating admin from user template...");
    
    // Create a template user first
    let template_user = User {
        email: String::from("alice@example.com"),
        username: String::from("Alice Johnson"),
        active: true,
        sign_in_count: 1,
    };
    
    // BUG: Create admin_user using update syntax, changing email and sign_in_count
    // Should have email "admin@company.com" and sign_in_count 100
    let admin_user = User {
        email: String::from("admin@company.com"),
        sign_in_count: 100,
        // Missing the update syntax to copy other fields
    };
    
    display_user(&admin_user);
    
    println!("\nDemonstrating ownership after update...");
    
    // BUG: This will cause an ownership error because template_user.email was moved
    // Fix this by understanding ownership or restructuring to avoid the move
    println!("Template user email: {}", template_user.email);
    
    println!("Admin was created using update syntax");
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
    fn test_struct_update_syntax() {
        let original = User {
            email: String::from("test@example.com"),
            username: String::from("Original"),
            active: true,
            sign_in_count: 5,
        };
        
        let updated = User {
            username: String::from("Updated"),
            ..original
        };
        
        assert_eq!(updated.email, "test@example.com");
        assert_eq!(updated.username, "Updated");
        assert_eq!(updated.active, true);
        assert_eq!(updated.sign_in_count, 5);
    }

    #[test]
    fn test_field_init_shorthand() {
        let email = String::from("shorthand@example.com");
        let username = String::from("Shorthand User");
        let active = true;
        let sign_in_count = 10;
        
        let user = User {
            email,
            username,
            active,
            sign_in_count,
        };
        
        assert_eq!(user.email, "shorthand@example.com");
        assert_eq!(user.username, "Shorthand User");
        assert_eq!(user.active, true);
        assert_eq!(user.sign_in_count, 10);
    }
}