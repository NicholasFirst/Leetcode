#[allow(dead_code)]
struct Solution;

impl Solution { 
    /// From: https://leetcode.cn/problems/ugly-number/
    #[allow(dead_code)]
    pub fn is_ugly(n: i32) -> bool {
        if n < 1 {
            return false;
        }
        if n <= 6 {
            return true;
        }
        if n % 2 == 0 {
            return Self::is_ugly( n / 2);
        }
        if n % 3 == 0 {
            return Self::is_ugly(n / 3);
        }
        if n % 5 == 0 {
            return Self::is_ugly(n / 5);
        }
        return false;
    }

}

#[cfg(test)]
mod test{
    use super::*;

    #[test]
    fn test() {
        let res = Solution::is_ugly(4199040);
        assert!(res);
    }
}