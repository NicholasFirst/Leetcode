struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn array_sign(nums: Vec<i32>) -> i32 {
        let mut ans = 1;
        for x in nums {
            if x == 0 {
                return 0;
            } else if x < 0 {
                ans *= -1;
            }
        }
        ans
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test() {
        let nums = vec![-1, -2, -3, -4, 3, 2, 1];
        let ans = Solution::array_sign(nums);
        assert_eq!(1, ans);
    }
}
