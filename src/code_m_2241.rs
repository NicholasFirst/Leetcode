#[derive(Default, Debug)]
struct ATM {
    cnt: Vec<i64>,   // 每张钞票剩余数量
    value: Vec<i64>,   // 每张钞票面额
}

impl ATM {

    fn new() -> Self {
        ATM {
            cnt: vec![0, 0, 0, 0, 0],
            value: vec![20, 50, 100, 200, 500],
        }
    }
    
    fn deposit(&mut self, banknotes_count: Vec<i32>) {
        for i in 0..5 {
            self.cnt[i] += banknotes_count[i] as i64;
        }
    }
    
    fn withdraw(&mut self, mut amount: i32) -> Vec<i32> {
        let mut res = vec![0; 5];
        // 模拟尝试取出钞票的过程
        for i in (0..5).rev() {
            res[i] = std::cmp::min(self.cnt[i], amount as i64 / self.value[i]) as i32;
            amount -= res[i] * self.value[i] as i32;
        }
        if amount > 0 {
            // 无法完成该操作
            vec![-1]
        } else {
            // 可以完成该操作
            for i in 0..5 {
                self.cnt[i] -= res[i] as i64;
            }
            res
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let mut atm = ATM::new();
        atm.deposit(vec![0,0,1,2,1]);
        dbg!(atm);
        // assert_eq!(atm.withdraw(200), vec![0,1,1,0,0]);
    }
}