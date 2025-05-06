struct Solution;

impl Solution {
    // https://leetcode.cn/problems/find-subsequence-of-length-k-with-the-largest-sum/?envType=problem-list-v2&envId=sorting
    // 给你一个整数数组 nums 和一个整数 k 。你需要找到 nums 中长度为 k 的 子序列 ，且这个子序列的 和最大 。
    // 请你返回 任意 一个长度为 k 的整数子序列。
    // 子序列 定义为从一个数组里删除一些元素后，不改变剩下元素的顺序得到的数组。
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