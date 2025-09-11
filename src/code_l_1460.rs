
struct Solution;

impl Solution {
    /// https://leetcode.cn/problems/make-two-arrays-equal-by-reversing-sub-arrays/
    #[allow(dead_code)]
    pub fn can_be_equal(target: Vec<i32>, arr: Vec<i32>) -> bool {
        let mut target = target;
        let mut arr = arr;
        target.sort();
        arr.sort();
        target == arr
    }
}

#[cfg(test)]
mod test{
    use super::Solution;

    #[test]
    fn test() {
        let target = vec![1,2,3,4];
        let arr = vec![2,4,1,3];
        let res = Solution::can_be_equal(target, arr);
        assert!(res);
    }

    #[test]
    fn test1() {
        let target = vec![3,7,9];
        let arr = vec![3,7,11];
        let res = Solution::can_be_equal(target, arr);
        assert!(!res);
    }
}