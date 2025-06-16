use std::i32;

struct Solution;

impl Solution {
    // https://leetcode.cn/problems/maximum-difference-between-increasing-elements/description/?envType=daily-question&envId=2025-06-16
    #[allow(dead_code)]
    pub fn maximum_difference(nums: Vec<i32>) -> i32 {
        let mut ans = 0;
        let mut pre_min = i32::MAX;
        for x in nums {
            ans = ans.max(x - pre_min);
            pre_min = pre_min.min(x);
        }
        if ans > 0 { ans } else { -1 }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_maximum_difference() {
        assert_eq!(Solution::maximum_difference(vec![1, 2, 3, 4]), 3);
        assert_eq!(Solution::maximum_difference(vec![4, 3, 2, 1]), -1);
        assert_eq!(Solution::maximum_difference(vec![1, 5, 2, 10]), 9);
    }
}