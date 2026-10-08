// SOLUTION 5: Bank account

#[derive(Debug)]
struct Account {
    owner: String,
    balance: i64,
}

impl Account {
    fn new(owner: &str) -> Account {
        Account { owner: owner.to_string(), balance: 0 }
    }

    fn deposit(&mut self, amount: i64) {
        self.balance += amount;
    }

    fn withdraw(&mut self, amount: i64) -> bool {
        if amount <= self.balance {
            self.balance -= amount;
            true
        } else {
            false
        }
    }
}

fn main() {
    let mut a = Account::new("Ada");
    a.deposit(100);
    println!("withdraw 30: {}", a.withdraw(30));
    println!("withdraw 500: {}", a.withdraw(500));
    println!("{a:?}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bank() {
        let mut a = Account::new("Ada");
        a.deposit(100);
        assert!(a.withdraw(30));
        assert!(!a.withdraw(500));
        assert_eq!(a.balance, 70);
        assert_eq!(a.owner, "Ada");
    }
}
