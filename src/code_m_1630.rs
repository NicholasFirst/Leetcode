struct Solution;

impl Solution {
    // https://leetcode.cn/problems/arithmetic-subarrays/solutions/
    #[allow(dead_code)]
    pub fn check_arithmetic_subarrays(nums: Vec<i32>, l: Vec<i32>, r: Vec<i32>) -> Vec<bool> {
        (0..l.len()).map(|i| Self::check_subarrays(nums[l[i] as usize..r[i] as usize + 1].to_vec())).collect::<Vec<_>>()
    }

    fn check_subarrays(mut nums: Vec<i32>) -> bool {
        if nums.len() < 2 { return false; }
        if nums.len() == 2 { return true; }
        nums.sort();
        for i in 1..nums.len() {
            if nums[1] - nums[0] != nums[i] - nums[i - 1] { return false; }
        }
        true
    }
}


#[cfg(test)]
mod test{
    use super::Solution;

    #[test]
    fn test() {
        let nums = vec![4,6,5,9,3,7];
        let l = vec![0,0,2];
        let r = vec![2,3,5];
        let res = Solution::check_arithmetic_subarrays(nums, l, r);
        assert_eq!(vec![true,false,true], res);
    }
}