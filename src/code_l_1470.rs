
struct Solution;

impl Solution {

    /// https://leetcode.cn/problems/shuffle-the-array/
    #[allow(dead_code)]
    pub fn shuffle(nums: Vec<i32>, n: i32) -> Vec<i32> {
        (0..n as usize).flat_map(|i| vec![nums[i], nums[i + n as usize]]).collect()
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test() {
        let nums = vec![1,2,3,4,4,3,2,1];
        let n = 4;
        let res = Solution::shuffle(nums, n);
        assert_eq!(vec![1,4,2,3,3,2,4,1], res);
    }

}