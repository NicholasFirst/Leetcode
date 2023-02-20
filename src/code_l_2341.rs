struct Solution;

impl Solution {

    // https://leetcode.cn/problems/maximum-number-of-pairs-in-array/
    #[allow(dead_code)]
    pub fn number_of_pairs(mut nums: Vec<i32>) -> Vec<i32> {
        nums.sort_unstable();
        let mut skip = false;
        let mut ans = 0;
        for a in nums.windows(2) {
            if skip {
                skip = false;
            } else if a[0] == a[1] {
                skip = true;
                ans += 1;
            }
        }
        vec![ans, nums.len() as i32 - ans * 2]
    }
}


#[cfg(test)]
mod test{
    use super::Solution;

    #[test]
    fn test() {
        let nums = vec![1,1];
        let ans = Solution::number_of_pairs(nums);
        assert_eq!(ans[0], 1);
        assert_eq!(ans[1], 0);

        let nums1 = vec![0];
        let ans = Solution::number_of_pairs(nums);
        assert_eq!(ans[0], 0);
        assert_eq!(ans[1], 1);
    }
}