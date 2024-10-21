struct Solution;

impl Solution {
    // https://leetcode.cn/problems/smallest-range-ii/description/?envType=daily-question&envId=2024-10-21
    #[allow(dead_code)]
    pub fn smallest_range_ii(mut nums: Vec<i32>, k: i32) -> i32 {
        nums.sort();
        let (mi, ma) = (nums[0], nums[nums.len() - 1]);
        let mut res = ma - mi;
        for i in 0..nums.len() - 1 {
            let (a, b) = (nums[i], nums[i + 1]);
            res = res.min((ma - k).max(a + k) - (mi + k).min(b - k));
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(Solution::smallest_range_ii(vec![1], 0), 0);
        assert_eq!(Solution::smallest_range_ii(vec![0, 10], 2), 6);
        assert_eq!(Solution::smallest_range_ii(vec![1, 3, 6], 3), 3);
    }
}