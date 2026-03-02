use std::collections::HashSet;

struct Solution;

impl Solution {
 
    // https://leetcode.cn/problems/two-out-of-three/
    #[allow(dead_code)]
    pub fn two_out_of_three(nums1: Vec<i32>, nums2: Vec<i32>, nums3: Vec<i32>) -> Vec<i32> {
        let s1 = nums1.into_iter().collect::<HashSet<_>>();
        let s2 = nums2.into_iter().collect::<HashSet<_>>();
        let s3 = nums3.into_iter().collect::<HashSet<_>>();
        let mut res = HashSet::new();

        for &x in s1.intersection(&s2) {
            res.insert(x);
        }

        for &x in s2.intersection(&s3) {
            res.insert(x);
        }

        for &x in s3.intersection(&s1) {
            res.insert(x);
        }

        res.into_iter().collect()
    }
}

#[cfg(test)]
mod test{
    use super::Solution;

    #[test]
    fn test() {
        let nums1 = vec![1,1,3,2];
        let nums2 = vec![2,3];
        let nums3 = vec![3];
        let mut res = Solution::two_out_of_three(nums1, nums2, nums3);
        res.sort();
        assert_eq!(vec![2, 3], res);
    }


    #[test]
    fn test_vec() {
        let a = vec![1, 2, 3];
        let mut b = vec![3, 2, 1];
        b.sort();
        assert_eq!(a, b)
    }
}