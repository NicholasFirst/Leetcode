struct Bank {
    #[allow(unused)]
    total: i32,
    #[allow(unused)]
    balance: Vec<i64>
}


impl Bank {

    #[allow(dead_code)]
    fn new(balance: Vec<i64>) -> Self {
        Self {
            total: balance.len() as i32,
            balance: balance
        }
    }
    
    #[allow(dead_code)]
    fn transfer(&mut self, account1: i32, account2: i32, money: i64) -> bool {
        if account1 <= self.total && account2 <= self.total {
            if self.withdraw(account1, money) {
                self.deposit(account2, money);
                return true;
            }
        }
        false
    }
    
    #[allow(dead_code)]
    fn deposit(&mut self, account: i32, money: i64) -> bool {
        if account > self.total {
            return false;
        }
        let num = (account - 1) as usize;
        self.balance[num] += money;
        true
    }
    
    #[allow(dead_code)]
    fn withdraw(&mut self, account: i32, money: i64) -> bool {
        if account > self.total {
            return false;
        }
        let num = (account - 1) as usize;
        let left = self.balance[num];
        if left >= money {
            self.balance[num] -= money;
            return true;
        }
        false
    }
}


#[cfg(test)]
mod tests {
    
    use super::*;

    #[test]
    fn test_2043() {
        let mut bank = Bank::new(vec![10, 100, 20, 50, 30]);
        assert_eq!(bank.withdraw(3, 10), true);
        assert_eq!(bank.transfer(5, 1, 20), true);
        assert_eq!(bank.deposit(5, 20), true);
        assert_eq!(bank.transfer(3, 4, 15), false);
        assert_eq!(bank.withdraw(10, 50), false);
    }
}