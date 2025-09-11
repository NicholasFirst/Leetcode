struct Solution;


impl Solution {
    //https://leetcode.cn/problems/power-of-three/description/
    #[allow(dead_code)]
    pub fn is_power_of_three(mut n: i32) -> bool {
        while n != 0 && n % 3 == 0 {
            n /= 3;
        }
        n == 1
        //n > 0 && 1162261467 % n == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let n = 27;
        let result = Solution::is_power_of_three(n);
        assert_eq!(result, true);
        let n = 0;
        let result = Solution::is_power_of_three(n);
        assert_eq!(result, false);
        let n = 9;
        let result = Solution::is_power_of_three(n);
        assert_eq!(result, true);
        let n = 45;
        let result = Solution::is_power_of_three(n);
        assert_eq!(result, false);
    }
}