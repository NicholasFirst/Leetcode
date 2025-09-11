struct Solution;

impl Solution {
    // https://leetcode.cn/problems/divisible-and-non-divisible-sums-difference/?envType=daily-question&envId=2025-05-27
    #[allow(dead_code)]
    pub fn difference_of_sum(n: i32, m: i32) -> i32 {
        let mut ans = 0;
        for i in 1..=n {
            if i % m == 0 {
                ans -= i;
            } else {
                ans += i;
            }
        }
        ans
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2894() {
        assert_eq!(Solution::difference_of_sum(10, 3), 19);
        assert_eq!(Solution::difference_of_sum(5, 6), 15);
        assert_eq!(Solution::difference_of_sum(5, 1), -15);
    }
}