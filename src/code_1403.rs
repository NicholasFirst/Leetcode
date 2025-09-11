struct Solution;

impl Solution {
    ///From: https://leetcode.cn/problems/minimum-subsequence-in-non-increasing-order/
    #[allow(dead_code)]
    pub fn min_subsequence(nums: Vec<i32>) -> Vec<i32> {
        let mut nums = nums;
        let half = nums.iter().sum::<i32>() / 2;    //对半分
        let mut acc = 0;
        nums.sort_by(|a, b| b.cmp(a));
        nums.into_iter().take_while(|x| {
            acc += x;
            acc -x <= half
        }).collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let nums = vec![4,3,10,9,8];
        let res = Solution::min_subsequence(nums);
        assert_eq!(vec![10, 9], res);
    }
}