struct Solution;

impl Solution {

    // https://leetcode.cn/problems/check-if-it-is-a-good-array/description/
    #[allow(dead_code)]
    pub fn is_good_array(nums: Vec<i32>) -> bool {
        nums.into_iter().fold(0, gcd) == 1
    }
}

fn gcd(x: i32, y: i32) -> i32 {
    if x == 0 {
        y
    } else {
        gcd(y % x, x)
    }
}

#[cfg(test)]
mod test{
    use super::Solution;

    #[test]
    fn test() {
        let nums = vec![12,5,7,23];
        let res = Solution::is_good_array(nums);
        assert!(res);
    }

    #[test]
    fn test1() {
        let nums = vec![3,6];
        let res = Solution::is_good_array(nums);
        assert!(!res);
    }
}