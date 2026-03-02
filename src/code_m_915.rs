struct Solution;

impl Solution {
    // https://leetcode.cn/problems/partition-array-into-disjoint-intervals/
    #[allow(dead_code)]
    pub fn partition_disjoint(nums: Vec<i32>) -> i32 {
        let mut prev_max = nums[0];
        let mut ans = 0;
        let mut max_val = nums[0];
        for i in 1..nums.len() {
            max_val = max_val.max(nums[i]);
            if nums[i] < prev_max {
                prev_max = max_val;
                ans = i;
            }
        }
        (ans + 1) as i32
    }
}

#[cfg(test)]
mod test{
    use super::Solution;

    #[test]
    fn test() {
        let nums = vec![5,0,3,8,6];
        let ans = Solution::partition_disjoint(nums);
        assert_eq!(ans, 3);
    }
}