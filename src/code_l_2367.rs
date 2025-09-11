struct Solution;

impl Solution {
    // https://leetcode.cn/problems/number-of-arithmetic-triplets/
    #[allow(dead_code)]
    pub fn arithmetic_triplets(nums: Vec<i32>, diff: i32) -> i32 {
        let (mut i, mut j, mut k, mut cnt) = (0, 1, 2, 0);
        while k < nums.len() {
            while j < k && nums[j] + diff < nums[k] { j += 1; }
            if nums[j] + diff == nums[k] {
                while i < j && nums[i] + diff < nums[j] { i += 1; }
                if nums[i] + diff == nums[j] { cnt += 1; }
            }
            k += 1;
        }
        cnt
    }
}

#[cfg(test)]
mod test{
    use super::Solution;

    #[test]
    fn test() {
        let nums = vec![0,1,4,6,7,10];
        let diff = 3;
        let ans = Solution::arithmetic_triplets(nums, diff);
        assert_eq!(2, ans);
    }
}