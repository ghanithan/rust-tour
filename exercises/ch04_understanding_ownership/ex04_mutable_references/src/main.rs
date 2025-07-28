// Fix the borrowing errors in this program
// Pay attention to Rust's borrowing rules!

fn main() {
    // Example 1: Basic mutable reference
    let mut x = 42;
    println!("Original value: {}", x);
    
    increment(&mut x);
    println!("After increment: {}", x);
    
    // Example 2: Multiple reference error
    let mut numbers = vec![1, 2, 3];
    
    let first = &numbers[0];  // Immutable reference
    let mut_ref = &mut numbers;  // ERROR: Cannot have both!
    
    println!("First element: {}", first);
    process_list(mut_ref);
    
    // Example 3: Iterator and modification
    let mut data = vec![1, 2, 3];
    
    for item in &data {  // Immutable borrow for iteration
        println!("Processing item: {}", item);
        data.push(*item * 2);  // ERROR: Cannot modify while iterating!
    }
    
    println!("Final list: {:?}", data);
    
    // Example 4: Bank account with transaction tracking
    let mut account = BankAccount::new(1000);
    println!("Account balance: ${}", account.balance());
    
    // Multiple mutable references problem
    let balance_ref = &mut account;
    let transaction_ref = &mut account;  // ERROR: Two mutable references!
    
    balance_ref.deposit(150);
    transaction_ref.withdraw(100);
    
    println!("After deposit: ${}", account.balance());
    println!("After withdrawal: ${}", account.balance());
    println!("Transaction history: {:?}", account.transactions());
}

fn increment(n: &mut i32) {
    *n += 1;  // Dereference and modify
}

fn process_list(list: &mut Vec<i32>) {
    for item in list.iter_mut() {
        *item *= 2;
    }
}

#[derive(Debug)]
enum Transaction {
    Deposit(u32),
    Withdrawal(u32),
}

struct BankAccount {
    balance: u32,
    transactions: Vec<Transaction>,
}

impl BankAccount {
    fn new(initial_balance: u32) -> Self {
        BankAccount {
            balance: initial_balance,
            transactions: Vec::new(),
        }
    }
    
    fn balance(&self) -> u32 {
        self.balance
    }
    
    fn deposit(&mut self, amount: u32) {
        self.balance += amount;
        self.transactions.push(Transaction::Deposit(amount));
    }
    
    fn withdraw(&mut self, amount: u32) {
        if self.balance >= amount {
            self.balance -= amount;
            self.transactions.push(Transaction::Withdrawal(amount));
        }
    }
    
    fn transactions(&self) -> &Vec<Transaction> {
        &self.transactions
    }
}