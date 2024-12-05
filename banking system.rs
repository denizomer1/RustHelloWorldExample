// Define the Account trait with required methods
trait Account {
    fn deposit(&mut self, amount: f64);
    fn withdraw(&mut self, amount: f64);
    fn balance(&self) -> f64;
}

// Define the BankAccount struct
struct BankAccount {
    account_number: u32,
    holder_name: String,
    balance: f64,
}

// Implement the Account trait for BankAccount
impl Account for BankAccount {
    // Deposit method: adds the specified amount to the balance
    fn deposit(&mut self, amount: f64) {
        if amount > 0.0 {
            self.balance += amount;
            println!(
                "Deposited {:.2} to account {}. New balance: {:.2}",
                amount, self.account_number, self.balance
            );
        } else {
            println!("Deposit amount must be positive.");
        }
    }

    // Withdraw method: subtracts the specified amount from the balance
    fn withdraw(&mut self, amount: f64) {
        if amount > 0.0 {
            if self.balance >= amount {
                self.balance -= amount;
                println!(
                    "Withdrew {:.2} from account {}. New balance: {:.2}",
                    amount, self.account_number, self.balance
                );
            } else {
                println!("Insufficient funds for withdrawal in account {}.", self.account_number);
            }
        } else {
            println!("Withdrawal amount must be positive.");
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

    // Perform operations on the accounts
    account1.deposit(200.0); // Deposit into account1
    account2.withdraw(50.0); // Withdraw from account2

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
