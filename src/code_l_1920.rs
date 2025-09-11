struct Solution;

impl Solution {
    // https://leetcode.cn/problems/build-array-from-permutation/?envType=daily-question&envId=2025-05-06
    #[allow(dead_code)]
    pub fn build_array(nums: Vec<i32>) -> Vec<i32> {
        nums.iter().map(|&x| nums[x as usize]).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1920() {
        assert_eq!(Solution::build_array(vec![0, 2, 1, 5, 3, 4]), vec![0, 1, 2, 4, 5, 3]);
        assert_eq!(Solution::build_array(vec![5, 0, 1, 2, 3, 4]), vec![4, 5, 0, 1, 2, 3]);
    }
}