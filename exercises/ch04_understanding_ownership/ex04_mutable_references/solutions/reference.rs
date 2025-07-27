// Solution: Fixed borrowing errors in mutable references

fn main() {
    // Example 1: Basic mutable reference (this already worked)
    let mut x = 42;
    println!("Original value: {}", x);
    
    increment(&mut x);
    println!("After increment: {}", x);
    
    // Example 2: Fix multiple reference error by limiting scope
    let mut numbers = vec![1, 2, 3];
    
    {
        let first = &numbers[0];  // Immutable reference in limited scope
        println!("First element: {}", first);
    }  // first goes out of scope here, ending the immutable borrow
    
    process_list(&mut numbers);  // Now we can create a mutable reference
    
    // Example 3: Fix iterator and modification by collecting first
    let mut data = vec![1, 2, 3];
    
    // Collect what we want to add without modifying original
    let to_add: Vec<i32> = data.iter().map(|item| {
        println!("Processing item: {}", item);
        item * 2
    }).collect();
    
    // Now modify the original vector
    data.extend(to_add);
    println!("Final list: {:?}", data);
    
    // Example 4: Fix multiple mutable references by using sequential operations
    let mut account = BankAccount::new(1000);
    println!("Account balance: ${}", account.balance());
    
    // Use the account directly instead of creating multiple references
    account.deposit(150);
    println!("After deposit: ${}", account.balance());
    
    account.withdraw(100);
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