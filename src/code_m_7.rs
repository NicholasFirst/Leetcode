struct Solution;

impl Solution {

    // https://leetcode.cn/problems/reverse-integer/
    #[allow(dead_code)]
    pub fn reverse(x: i32) -> i32 {
        fn helper(mut n: i32) -> Option<i32> {
            let mut res = 0i32;
            while n.abs() != 0 {
                res = res.checked_mul(10)?.checked_add(n % 10)?;
                n /= 10;
            }
            Some(res)
        }
        helper(x).unwrap_or_default()
    }
}

#[cfg(test)]
mod test{
    use super::Solution;

    #[test]
    fn test() {
        let x = -2147483648;
        let ans = Solution::reverse(x);
        assert_eq!(0, ans);
    }
}