#[derive(Debug)]
struct BankAccount{
    balance: f64,
}

impl BankAccount{
    fn new(initial_balance: f64) -> BankAccount{
        BankAccount{
            balance: initial_balance,
        }
    }
    fn deposit(&mut self, amount: f64){
        if amount >= 0.0{
            self.balance = self.balance + amount;
        }
    }
    fn withdraw(&mut self, amount: f64){
        if amount >= 0.0&& amount <= self.balance{
            self.balance = self.balance - amount;
        }
    }
    fn balance(&self) -> f64{
        self.balance 
    }
}

fn main(){
    let mut account = BankAccount::new(100.0);

    println!("Balance: {}", account.balance());

    account.deposit(50.0);
    println!("After deposit: {}", account.balance());

    account.withdraw(30.0);
    println!("after withdraw: {}", account.balance());
}

#[cfg(test)]
mod test{ 
    use super::*;

    #[test]
    fn test_new_account(){
        let account = BankAccount::new(100.0);
        assert_eq!(account.balance(), 100.0);
    }

    #[test]
    fn test_deposit(){
        let mut account = BankAccount::new (100.0);
        account.deposit(50.0);
        assert_eq!(account.balance(), 150.0);
    }

    #[test]
    fn test_withdraw(){
        let mut account = BankAccount::new(100.0);
        account.withdraw(40.0);
        assert_eq!(account.balance(), 60.0);
    }
  #[test]  
  fn test_withdraw_too_much(){
    let mut account = BankAccount::new(100.0);
    account.withdraw(200.0);
    assert_eq!(account.balance(), 100.0);
  }
}
