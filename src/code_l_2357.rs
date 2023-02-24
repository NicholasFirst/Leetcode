struct Solution;

impl Solution {
    // https://leetcode.cn/problems/make-array-zero-by-subtracting-equal-amounts/
    #[allow(dead_code)]
    pub fn minimum_operations(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        nums.sort();
        let mut last = -1;
        let mut ans = 0;
        for val in nums {
           if val != 0 && val != last {
               ans += 1;
               last = val;
               continue;
           }
           last = val;
        }
        ans
    }
}

#[cfg(test)]
mod test{
    use super::Solution;

    #[test]
    fn test() {
        let nums = vec![1,5,0,3,5];
        let ans = Solution::minimum_operations(nums);
        assert_eq!(ans, 3);
    }

    #[test]
    fn test1() {
        let nums = vec![0, 5, 3];
        let ans = Solution::minimum_operations(nums);
        assert_eq!(ans, 2);
    }
}