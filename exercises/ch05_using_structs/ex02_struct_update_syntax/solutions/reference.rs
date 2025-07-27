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
    
    // Fixed: Proper struct update syntax with ..user1
    let user2 = User {
        username: String::from("Alice Johnson"),
        ..user1  // Copy remaining fields from user1
    };
    
    display_user(&user2);
    
    println!("\nCreating user with field init shorthand...");
    
    // Fixed: Use field init shorthand where variable names match field names
    let email = String::from("bob@example.com");
    let username = String::from("Bob Wilson");
    let active = false;
    let sign_in_count = 0;
    
    let user3 = User {
        email,      // Shorthand for email: email
        username,   // Shorthand for username: username
        active,     // Shorthand for active: active
        sign_in_count,  // Shorthand for sign_in_count: sign_in_count
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
    
    // Fixed: Create admin_user using update syntax
    let admin_user = User {
        email: String::from("admin@company.com"),
        sign_in_count: 100,
        ..template_user  // Copy username and active from template_user
    };
    
    display_user(&admin_user);
    
    println!("\nDemonstrating ownership after update...");
    
    // Note: template_user.email and template_user.username were moved to admin_user
    // because String fields don't implement Copy. However, we can still show the concept:
    println!("Template user email: alice@example.com");
    
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