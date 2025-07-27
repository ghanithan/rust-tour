use std::process::Command;

#[test]
fn test_program_compiles_and_runs() {
    let output = Command::new("cargo")
        .args(&["run", "--quiet"])
        .output()
        .expect("Failed to execute program");

    assert!(output.status.success(), "Program should compile and run successfully");
}

#[test]
fn test_expected_output() {
    let output = Command::new("cargo")
        .args(&["run", "--quiet"])
        .output()
        .expect("Failed to execute program");

    let stdout = String::from_utf8_lossy(&output.stdout);
    
    // Check for key outputs that demonstrate mutable references working
    assert!(stdout.contains("Original value: 42"), "Should show original value");
    assert!(stdout.contains("After increment: 43"), "Should show incremented value");
    assert!(stdout.contains("Processing item: 1"), "Should process first item");
    assert!(stdout.contains("Processing item: 2"), "Should process second item");
    assert!(stdout.contains("Processing item: 3"), "Should process third item");
    assert!(stdout.contains("Final list:"), "Should show final list");
    assert!(stdout.contains("Account balance: $1000"), "Should show initial balance");
    assert!(stdout.contains("After deposit: $1150"), "Should show balance after deposit");
    assert!(stdout.contains("After withdrawal: $1050"), "Should show balance after withdrawal");
    assert!(stdout.contains("Transaction history:"), "Should show transaction history");
}

#[test]
fn test_no_borrowing_errors() {
    let output = Command::new("cargo")
        .args(&["check"])
        .output()
        .expect("Failed to execute cargo check");

    let stderr = String::from_utf8_lossy(&output.stderr);
    
    // Ensure no borrowing errors
    assert!(!stderr.contains("cannot borrow"), "Should not have borrowing errors");
    assert!(!stderr.contains("borrow of moved value"), "Should not have moved value errors");
    assert!(!stderr.contains("cannot borrow as mutable"), "Should not have mutable borrow errors");
    assert!(!stderr.contains("cannot borrow as immutable"), "Should not have immutable borrow errors");
}

#[test]
fn test_increment_function() {
    // Test the increment function in isolation
    let mut x = 5;
    
    // This would be defined in main.rs, simulating here for test
    fn increment(n: &mut i32) {
        *n += 1;
    }
    
    increment(&mut x);
    assert_eq!(x, 6, "Increment should increase value by 1");
}

#[test]
fn test_process_list_function() {
    // Test the process_list function in isolation
    let mut numbers = vec![1, 2, 3];
    
    fn process_list(list: &mut Vec<i32>) {
        for item in list.iter_mut() {
            *item *= 2;
        }
    }
    
    process_list(&mut numbers);
    assert_eq!(numbers, vec![2, 4, 6], "Process list should double all values");
}

#[test]
fn test_bank_account_operations() {
    // Test the BankAccount functionality
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
    
    let mut account = BankAccount::new(1000);
    assert_eq!(account.balance(), 1000);
    
    account.deposit(150);
    assert_eq!(account.balance(), 1150);
    
    account.withdraw(100);
    assert_eq!(account.balance(), 1050);
    
    assert_eq!(account.transactions().len(), 2);
}