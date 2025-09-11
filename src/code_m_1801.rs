struct Solution;

impl Solution {

    // https://leetcode.cn/problems/number-of-orders-in-the-backlog/solutions/?languageTags=rust
    #[allow(dead_code)]
    pub fn get_number_of_backlog_orders(orders: Vec<Vec<i32>>) -> i32 {
        use std::cmp::{Ordering, Reverse};
        use std::collections::BinaryHeap;

        #[derive(Eq, Debug, Clone, Default)]
        struct Order {
            price: i32,
            amount: i32,
            order_type: i32,
        }
        impl Order {
            fn new(price: i32, amount: i32, order_type: i32) -> Self {
                Order { price, amount, order_type }
            }
        }
        impl PartialEq<Self> for Order {
            fn eq(&self, other: &Self) -> bool {
                self.price == other.price
            }
        }
        impl PartialOrd<Self> for Order {
            fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
                Some(self.cmp(other))
            }
        }
        impl Ord for Order {
            fn cmp(&self, other: &Self) -> Ordering {
                self.price.cmp(&other.price)
            }
        }

        let (mut buy, mut sell, m) = (BinaryHeap::new(), BinaryHeap::new(), 1_000_000_007);
        for order in orders {
            if order[2] == 0 { buy.push(Order::new(order[0], order[1], order[2])); } else { sell.push(Reverse(Order::new(order[0], order[1], order[2]))); }
            while !buy.is_empty() && !sell.is_empty() && buy.peek().unwrap().price >= sell.peek().unwrap().0.price {
                let amount = buy.peek().unwrap().amount.min(sell.peek().unwrap().0.amount);
                buy.peek_mut().unwrap().amount -= amount;
                sell.peek_mut().unwrap().0.amount -= amount;
                if buy.peek().unwrap().amount == 0 { buy.pop(); }
                if sell.peek().unwrap().0.amount == 0 { sell.pop(); }
            }
        }
        let cnt = buy.iter().fold(0, |cnt, o| (cnt + o.amount) % m);
        sell.iter().fold(cnt, |cnt, o| (cnt + o.0.amount) % m)
    }
}

#[cfg(test)]
mod test{
    use super::Solution;

    #[test]
    fn test() {
        let orders = vec![vec![7,1000000000,1],vec![15,3,0],vec![5,999999995,0],vec![5,1,1]];
        let res = Solution::get_number_of_backlog_orders(orders);
        assert_eq!(res, 999999984);
    }
}