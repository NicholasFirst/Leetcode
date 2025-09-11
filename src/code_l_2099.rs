struct Solution;

impl Solution {
    // https://leetcode.cn/problems/find-subsequence-of-length-k-with-the-largest-sum/?envType=problem-list-v2&envId=sorting
    #[allow(dead_code)]
    pub fn max_subsequence(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut a = Vec::with_capacity(nums.len());
        nums.into_iter().enumerate().for_each(|(i, x)| a.push((x, i)));
        a.sort_unstable_by(|x, y| y.0.cmp(&x.0));
        a.truncate(k as usize);
        a.sort_unstable_by_key(|x| x.1);
        a.into_iter().map(|x| x.0).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_subsequence() {
        assert_eq!(Solution::max_subsequence(vec![2,1,3,3], 2), vec![3,3]);
        assert_eq!(Solution::max_subsequence(vec![-1,-2,3,4], 3), vec![-1,3,4]);
        assert_eq!(Solution::max_subsequence(vec![3,4,3,3], 2), vec![3,4]);
    }
}