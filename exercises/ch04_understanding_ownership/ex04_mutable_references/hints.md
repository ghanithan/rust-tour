# Hints for Mutable References - Controlled Modification

## Level 1: Conceptual Hint

**Understanding Mutable References**:
- `&mut T` allows you to modify data through a reference
- Only one mutable reference can exist at a time
- Cannot have mutable and immutable references simultaneously
- These rules prevent data races at compile time

**Why These Rules Exist**:
Imagine if you could have multiple mutable references:
- Thread A modifies data while Thread B reads it
- Reader gets corrupted/inconsistent data
- Classic data race condition

Rust prevents this entirely at compile time!

**The Borrowing Rules**:
1. **Either** multiple immutable references **OR** one mutable reference
2. **Never** both at the same time
3. References must always be valid (no dangling pointers)

**Mental Model**: Think of references like library books:
- Multiple people can read the same book (immutable references)
- Only one person can edit the book at a time (mutable reference)
- You can't read while someone is editing (no mixing)

Each error in this exercise violates one of these rules. Think about which rule is broken and how to fix it.

## Level 2: Strategic Hint

**Error Analysis and Solutions**:

**Error 1: Mixed immutable and mutable references**
```rust
let first = &numbers[0];        // Immutable reference
let mut_ref = &mut numbers;     // Mutable reference - ERROR!
```
**Solution approach**: Limit the scope of references so they don't overlap

**Error 2: Modifying while iterating**
```rust
for item in &data {             // Immutable borrow for loop
    data.push(*item * 2);       // Mutable borrow - ERROR!
}
```
**Solution approach**: Collect items first, then modify separately

**Error 3: Multiple mutable references**
```rust
let balance_ref = &mut account;      // First mutable reference
let transaction_ref = &mut account;  // Second mutable reference - ERROR!
```
**Solution approach**: Use one reference at a time, or restructure code

**General Patterns**:
1. **Scope limiting**: Use `{}` blocks to end borrows early
2. **Sequential operations**: Do one operation, then another
3. **Collect then modify**: For iteration problems
4. **Single reference**: Use one mutable reference per scope

## Level 3: Implementation Hint

Here's how to fix each error:

**Fix 1: Limit reference scope**
```rust
// Limit the scope of the immutable reference
{
    let first = &numbers[0];
    println!("First element: {}", first);
}  // first goes out of scope here

// Now we can create a mutable reference
let mut_ref = &mut numbers;
process_list(mut_ref);
```

**Fix 2: Collect then modify approach**
```rust
let mut data = vec![1, 2, 3];

// Collect the items we want to add
let to_add: Vec<i32> = data.iter().map(|item| {
    println!("Processing item: {}", item);
    item * 2
}).collect();

// Now modify the original vector
data.extend(to_add);
```

**Fix 3: Sequential mutable operations**
```rust
// Use one mutable reference at a time
account.deposit(150);
println!("After deposit: ${}", account.balance());

account.withdraw(100);
println!("After withdrawal: ${}", account.balance());

// Or use a scoped approach:
{
    let balance_ref = &mut account;
    balance_ref.deposit(150);
}  // balance_ref scope ends

{
    let transaction_ref = &mut account;
    transaction_ref.withdraw(100);
}  // transaction_ref scope ends
```

**Complete working solution**:
```rust
fn main() {
    // Example 1: Basic mutable reference (this works as-is)
    let mut x = 42;
    println!("Original value: {}", x);
    increment(&mut x);
    println!("After increment: {}", x);
    
    // Example 2: Fix scope overlap
    let mut numbers = vec![1, 2, 3];
    
    {
        let first = &numbers[0];
        println!("First element: {}", first);
    }  // first reference ends here
    
    process_list(&mut numbers);
    
    // Example 3: Collect then modify
    let mut data = vec![1, 2, 3];
    
    let to_add: Vec<i32> = data.iter().map(|item| {
        println!("Processing item: {}", item);
        item * 2
    }).collect();
    
    data.extend(to_add);
    println!("Final list: {:?}", data);
    
    // Example 4: Sequential operations
    let mut account = BankAccount::new(1000);
    println!("Account balance: ${}", account.balance());
    
    account.deposit(150);
    println!("After deposit: ${}", account.balance());
    
    account.withdraw(100);
    println!("After withdrawal: ${}", account.balance());
    println!("Transaction history: {:?}", account.transactions());
}
```

**Key insight**: Rust's borrowing rules might seem restrictive, but they prevent entire classes of bugs that plague other languages!