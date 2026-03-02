struct Solution;

impl Solution {

    // https://leetcode.cn/problems/minimum-amount-of-time-to-fill-cups/
    #[allow(dead_code)]
    pub fn fill_cups(mut amount: Vec<i32>) -> i32 {
        amount.sort();
        match amount.as_slice() {
            [a, b, c] if a + b <= *c => *c,
            [a, b, c] => (a + b + c + 1) / 2,
            _ => unreachable!(),
        }
    }
}


#[cfg(test)]
mod test{
    use super::Solution;

    #[test]
    fn test() {
        let amount = vec![1,4,2];
        let ans = Solution::fill_cups(amount);
        assert_eq!(4, ans);
    }
}