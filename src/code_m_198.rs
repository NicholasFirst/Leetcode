struct Solution;

impl Solution {
 
    // https://leetcode.cn/problems/house-robber/
    #[allow(dead_code)]
    pub fn rob(nums: Vec<i32>) -> i32 {
        if nums.len() == 0 {
            return 0;
        }
        let mut dp = vec![0; nums.len() + 1];
        dp[1] = nums[0];
        for i in 2..=nums.len() {
            dp[i] = dp[i - 1].max(nums[i - 1] + dp[i - 2]);
        }
        dp[nums.len()]
    }
}

#[cfg(test)]
mod test{
    use super::Solution;

    #[test]
    fn test() {
        let nums = vec![1,2,3,1];
        let ans = Solution::rob(nums);
        assert_eq!(4, ans);
    }

    #[test]
    fn test1() {
        let nums = vec![2,7,9,3,1];
        let ans = Solution::rob(nums);
        assert_eq!(12, ans);
    }
}