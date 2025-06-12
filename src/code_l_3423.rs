use std::{os::windows, u32::MAX};

struct Solution;

impl Solution {
    // https://leetcode.cn/problems/maximum-difference-between-adjacent-elements-in-a-circular-array/description/?envType=daily-question&envId=2025-06-12
    #[allow(dead_code)]
    pub fn max_adjacent_distance(nums: Vec<i32>) -> i32 {
        if nums.len() < 2 {
            return 0;
        }
        let mut max_distance = (nums[0] - nums[nums.len() - 1]).abs().max(
            (nums[nums.len() - 1] - nums[0]).abs(),
        );
        nums.windows(2).for_each(|window| {
            let [x1, x2] = window else {
                unreachable!();
            };
            max_distance = max_distance.max((x1 - x2).abs()).max((x2 - x1).abs());
        });
        max_distance as i32
    }
}

#[cfg(test)]
mod tests {
    
    use super::*;

    #[test]
    fn test_max_adjacent_distance() {
        assert_eq!(Solution::max_adjacent_distance(vec![1, 3, 2, 5]), 4);
        assert_eq!(Solution::max_adjacent_distance(vec![4, 1, 2, 3]), 3);
        assert_eq!(Solution::max_adjacent_distance(vec![10]), 0);
        assert_eq!(Solution::max_adjacent_distance(vec![1, 2, 3, 4]), 3);
    }
}