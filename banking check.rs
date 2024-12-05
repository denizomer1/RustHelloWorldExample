// Define the Account trait with error handling
trait Account {
    fn deposit(&mut self, amount: f64) -> Result<(), String>;
    fn withdraw(&mut self, amount: f64) -> Result<(), String>;
    fn balance(&self) -> f64;
}

// Define the BankAccount struct
struct BankAccount {
    account_number: u32,
    holder_name: String,
    balance: f64,
}

// Implement the Account trait for BankAccount with error handling
impl Account for BankAccount {
    // Deposit method: returns Ok(()) on success or Err with a message on failure
    fn deposit(&mut self, amount: f64) -> Result<(), String> {
        if amount > 0.0 {
            self.balance += amount;
            Ok(())
        } else {
            Err("Deposit amount must be positive.".to_string())
        }
    }

    // Withdraw method: returns Ok(()) on success or Err with a message on failure
    fn withdraw(&mut self, amount: f64) -> Result<(), String> {
        if amount <= 0.0 {
            Err("Withdrawal amount must be positive.".to_string())
        } else if amount > self.balance {
            Err("Insufficient funds for withdrawal.".to_string())
        } else {
            self.balance -= amount;
            Ok(())
        }
    }

    // Balance method: returns the current balance
    fn balance(&self) -> f64 {
        self.balance
    }
}

fn main() {
    // Create two BankAccount instances
    let mut account1 = BankAccount {
        account_number: 1001,
        holder_name: String::from("Alice"),
        balance: 500.0,
    };

    let mut account2 = BankAccount {
        account_number: 1002,
        holder_name: String::from("Bob"),
        balance: 300.0,
    };

    // Perform operations and handle errors
    // Deposit into account1
    match account1.deposit(200.0) {
        Ok(()) => println!("Deposit successful for account {}.", account1.account_number),
        Err(e) => println!("Deposit failed for account {}: {}", account1.account_number, e),
    }

    // Withdraw from account2
    match account2.withdraw(400.0) {
        Ok(()) => println!("Withdrawal successful for account {}.", account2.account_number),
        Err(e) => println!("Withdrawal failed for account {}: {}", account2.account_number, e),
    }

    // Print balances
    println!(
        "Balance for account {} ({}): {:.2}",
        account1.account_number,
        account1.holder_name,
        account1.balance()
    );
    println!(
        "Balance for account {} ({}): {:.2}",
        account2.account_number,
        account2.holder_name,
        account2.balance()
    );
}
