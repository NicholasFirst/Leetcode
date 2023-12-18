struct Solution;

//https://leetcode.cn/problems/climbing-stairs/?envType=daily-question&envId=2023-12-10
impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        if n <= 2 {
            return n;
        };
        let mut a = 1;
        let mut b = 2;
        for _ in 3..=n {
            let c = a + b;
            a = b;
            b = c;
        }
        b
    }
}

#[cfg(test)]
mod tests{
use super::*;

    #[test]
    fn test_climb_stairs() {
        assert_eq!(Solution::climb_stairs(2), 2);
        assert_eq!(Solution::climb_stairs(3), 3);
        assert_eq!(Solution::climb_stairs(4), 5);
        assert_eq!(Solution::climb_stairs(5), 8);
        assert_eq!(Solution::climb_stairs(6), 13);
        assert_eq!(Solution::climb_stairs(7), 21);
    }
}