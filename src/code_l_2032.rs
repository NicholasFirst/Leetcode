struct Solution;

impl Solution {
 
    // https://leetcode.cn/problems/two-out-of-three/
    #[allow(dead_code)]
    pub fn two_out_of_three(nums1: Vec<i32>, nums2: Vec<i32>, nums3: Vec<i32>) -> Vec<i32> {
        let a = nums1.iter().filter(|&x| nums2.contains(x)).collect::<Vec<_>>();
        let b = nums3.iter().filter(|&x| nums2.contains(x)).collect::<Vec<_>>();
        let union = a.iter().filter(|&x| !b.contains(x)).chain(&b).collect::<Vec<_>>();
        let union = union.iter().map(|x| ***x).collect::<Vec<_>>();
        return union;
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