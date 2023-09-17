struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn rob(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        if n == 1 {
            return nums[0];
        }
        Self::sub_rob(&nums[1..]).max(Self::sub_rob(&nums[..n - 1]))
    }

    #[allow(dead_code)]
    pub fn sub_rob(nums: &[i32]) -> i32 {
        let mut dp = [0; 2];
        for num in nums {
            dp = [dp[1], (dp[0] + num).max(dp[1])];
        }
        dp[1]
    }
}

#[cfg(test)]
mod test{
    use super::Solution;

    #[test]
    fn test() {
        let nums = vec![2,3,2];
        let money = Solution::rob(nums);
        assert_eq!(3, money);
    }


    #[test]
    fn test1() {
        let nums = vec![1,2,3,1];
        let money = Solution::rob(nums);
        assert_eq!(4, money);
    }
}