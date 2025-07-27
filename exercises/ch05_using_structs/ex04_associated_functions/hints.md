# Hints for Associated Functions - Constructor Patterns

## Level 1: Conceptual Hint

Associated functions are functions that belong to a struct type but don't take `self` as their first parameter. They're called using the `::` syntax instead of the `.` syntax used for methods.

**Key Concepts:**
- **Associated Functions vs Methods**: Methods take `self` and are called on instances (`instance.method()`), while associated functions don't take `self` and are called on the type itself (`Type::function()`)
- **Constructor Pattern**: The most common use for associated functions is creating new instances of structs (constructors)
- **Multiple Constructors**: You can have many associated functions that create instances in different ways
- **Validation**: Associated functions can validate input before creating instances
- **Factory Methods**: Create instances with specific configurations or from other types

**Rust Book References:**
- Read [Chapter 5.3 - Associated Functions](https://doc.rust-lang.org/book/ch05-03-method-syntax.html#associated-functions)
- Review the difference between methods and associated functions
- Understand the `::` namespace syntax for calling associated functions

**Think About:**
- How would you create a `Person` with all required fields?
- What would a constructor for children (no email) look like?
- How can you validate input before creating an instance?
- What should happen when validation fails?

## Level 2: Strategic Hint

**Implementation Strategy:**

1. **Basic Constructor (`new`)**:
   ```rust
   impl Person {
       fn new(name: String, age: u32, email: String) -> Person {
           // Create Person instance with email wrapped in Some()
       }
   }
   ```

2. **Specialized Constructors**:
   - `child()`: No email parameter, set email to `None`
   - `adult()`: Similar to `new()` but semantically for adults
   - `from_name()`: Only name parameter, use defaults for age (0) and email (`None`)

3. **Validation Constructor**:
   ```rust
   fn new_validated(name: String, age: u32, email: String) -> Result<Person, String> {
       // Check age <= 150
       // Check email contains '@'
       // Return Ok(Person) or Err(String)
   }
   ```

**Error Handling Patterns:**
- Use `Result<Person, String>` for functions that can fail
- Return descriptive error messages that match the expected output
- Use early returns with `return Err(...)` for validation failures

**Common Patterns:**
- Use struct initialization shorthand when parameter names match field names
- Wrap email strings in `Some()` for the `Option<String>` field
- Return `Ok(Person { ... })` for successful validation

## Level 3: Implementation Hint

Here are the complete implementations:

```rust
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
}
```

**Key Implementation Details:**

1. **Struct Initialization**: Use the shorthand syntax when parameter names match field names (`name` instead of `name: name`)

2. **Option Handling**: 
   - For functions that take email: wrap in `Some(email)`
   - For functions without email: use `None`

3. **Validation Logic**:
   - Check age with `age > 150` (note: allows 0-150, not excluding 0)
   - Check email with `!email.contains('@')` for simple validation
   - Use exact error messages shown in the expected output

4. **Result Handling**: 
   - Return `Err(String::from("message"))` for validation failures
   - Return `Ok(Person { ... })` for successful creation
   - Use early returns for cleaner error handling

5. **Function Signatures**:
   - All take owned `String` parameters (not `&str`)
   - Return `Person` directly for infallible constructors
   - Return `Result<Person, String>` for validation constructor

**Testing Your Implementation:**
- Run `cargo run` to see the expected output
- Run `cargo test` to verify all test cases pass
- Check that validation messages match exactly (tests look for specific substrings)