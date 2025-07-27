# Hints for Struct Definition

## Level 1: Conceptual Hint

Structs in Rust are custom data types that let you package together and name multiple related values. Think of a struct as a blueprint or template for creating instances of data.

Key concepts to understand:
- **Structure Definition**: Define the fields and their types
- **Instance Creation**: Create actual instances with specific values
- **Field Access**: Access individual pieces of data using dot notation
- **Ownership**: Structs follow the same ownership rules as other Rust values

Rust Book Reference: [Chapter 5.1 - Defining and Instantiating Structs](https://doc.rust-lang.org/book/ch05-01-defining-structs.html)

The struct syntax allows you to group related data together in a meaningful way. Each field has a name and a type, making your code more readable and self-documenting.

## Level 2: Strategic Hint

Here's the approach to complete this exercise:

1. **Define the User struct** outside of any function:
```rust
struct User {
    field_name: field_type,
    // ... more fields
}
```

2. **Create struct instances** using struct initialization syntax:
```rust
let instance_name = StructName {
    field1: value1,
    field2: value2,
    // ... more fields
};
```

3. **Access fields** using dot notation:
```rust
let value = instance.field_name;
println!("Field value: {}", instance.field_name);
```

4. **Understand ownership**: When you assign a struct to another variable, ownership is moved (for non-Copy types like String).

Remember:
- All fields must be specified when creating a struct instance
- Field order in instantiation doesn't have to match definition order
- Use `String::from("text")` to create owned String values

## Level 3: Implementation Hint

Here's the complete implementation with explanations:

```rust
// Define the User struct with all required fields
struct User {
    email: String,      // Owned string for email address
    username: String,   // Owned string for display name
    active: bool,       // Boolean for account status
    sign_in_count: u64, // Unsigned integer for login count
}

fn main() {
    println!("Creating users...");
    
    // Create Alice's user account
    let alice = User {
        email: String::from("alice@example.com"),
        username: String::from("Alice Smith"),
        active: true,
        sign_in_count: 1,
    };
    
    // Create Bob's user account  
    let bob = User {
        email: String::from("bob@example.com"),
        username: String::from("Bob Jones"),
        active: false,
        sign_in_count: 0,
    };
    
    // Display user information using references
    display_user(&alice);
    display_user(&bob);
    
    println!("After transferring alice...");
    
    // Move alice to demonstrate ownership transfer
    let transferred_user = alice;
    
    // Bob is still accessible since we didn't move it
    println!("Bob's email: {}", bob.email);
    
    // alice is no longer accessible - it was moved
    // This would cause a compile error:
    // println!("Alice's email: {}", alice.email);
    
    println!("Alice was moved and is no longer accessible");
}

// Display function takes a reference to avoid taking ownership
fn display_user(user: &User) {
    println!(
        "User: {} ({}) - Active: {}, Sign-in count: {}",
        user.email, user.username, user.active, user.sign_in_count
    );
}
```

**Key Points Explained:**
- The struct is defined outside of `main()` so it's available throughout the module
- We use `String::from()` to create owned String values
- The `display_user` function takes `&User` (a reference) so it doesn't take ownership
- Moving `alice` to `transferred_user` makes `alice` inaccessible
- Field access uses dot notation: `user.field_name`