
struct Solution;

impl Solution {
    /// https://leetcode.cn/problems/maximum-product-of-two-elements-in-an-array/
    #[allow(dead_code)]
    pub fn max_product(nums: Vec<i32>) -> i32 {
        // let mut nums = nums;
        // nums.sort();
        // (nums[nums.len() - 1] - 1) * (nums[nums.len() - 2] - 1)
        let (mut first, mut second) = (0, 0);
        nums.iter().for_each(|&x| {
            if x > first {
                second = first;
                first = x;
            } else if x > second { second = x; }
        });
        (first - 1) * (second - 1)
    }
}

#[cfg(test)]
mod test{

    use super::Solution;
    #[test]
    fn test() {
        let nums = vec![3,4,5,2];
        let res = Solution::max_product(nums);
        assert_eq!(12, res);
    }
}