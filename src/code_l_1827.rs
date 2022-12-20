struct Solution;

impl Solution {
    // https://leetcode.cn/problems/minimum-operations-to-make-the-array-increasing/
    #[allow(dead_code)]
    pub fn min_operations(nums: Vec<i32>) -> i32 {
        let mut ans = 0;
        let mut start = nums[0];
        nums.iter().skip(1).for_each(|e| {
            if e > &start {
                start = *e;
            }else {
                start += 1;
                ans += start - e;
            }
        });
        ans
    }
}

#[cfg(test)]
mod test{
    use super::Solution;

    #[test]
    fn test() {
        let nums = vec![1,5,2,4,1];
        let res = Solution::min_operations(nums);
        assert_eq!(res, 14);
    }

    #[test]
    fn test1() {
        let nums = vec![4];
        let res = Solution::min_operations(nums);
        assert_eq!(res, 0);
    }
}