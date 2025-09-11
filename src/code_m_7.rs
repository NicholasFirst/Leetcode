struct Solution;

impl Solution {

    // https://leetcode.cn/problems/reverse-integer/
    #[allow(dead_code)]
    pub fn reverse(x: i32) -> i32 {
        let mut x = x;
        let mut ans = 0;

        while x != 0 {
            if ans > i32::MAX / 10 || ans < i32::MIN / 10 {
                return 0;
            }
            ans = ans * 10 + x % 10;
            x /= 10;
        }
        ans
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