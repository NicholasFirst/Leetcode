use std::mem::swap;

struct Solution;

impl Solution {
    // https://leetcode.cn/problems/equal-sum-arrays-with-minimum-number-of-operations/
    #[allow(dead_code)]
    pub fn min_operations(mut nums1: Vec<i32>, mut nums2: Vec<i32>) -> i32 {
        if 6*nums1.len() < nums2.len() || 6*nums2.len() < nums1.len() {
            return -1;
        }
        let mut diff: i32 = nums1.iter().sum::<i32>() - nums2.iter().sum::<i32>();
        if diff == 0 {
            return 0;
        }
        if diff < 0 {
            diff = -diff;
            swap(&mut nums1, &mut nums2);
        }
        let mut changes = [0;6];
        for n1 in nums1.into_iter() {
            changes[(n1-1) as usize] += 1;
        }
        for n2 in nums2.into_iter() {
            changes[(6-n2) as usize] += 1;
        }
        let mut ans = 0; 
        for (i, &c) in changes.iter().enumerate().rev() {
            let i = i as i32;
            if diff <= i*c {
                return ans + (diff+i-1)/i;
            }
            ans += c;
            diff -= i*c;
        }
        return ans;
    }
}

#[cfg(test)]
mod test{
    use super::Solution;

    #[test]
    fn test() {
        let nums1 = vec![1,2,3,4,5,6];
        let nums2 = vec![1,1,2,2,2,2];
        let res = Solution::min_operations(nums1, nums2);
        assert_eq!(res, 3);
    }
}