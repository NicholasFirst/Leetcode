struct Solution;

impl Solution {
    // https://leetcode.cn/problems/number-of-bit-changes-to-make-two-integers-equal/?envType=daily-question&envId=2024-11-02
    #[allow(dead_code)]
    pub fn min_changes(n: i32, k: i32) -> i32 {
        let mut res = 0;
        let mut n = n;
        let mut k = k;
        while n > 0 || k > 0 {
            if (n & 1) == 0 && (k & 1) == 1 {
                return -1;
            }
            if (n & 1) == 1 && (k & 1) == 0 {
                res += 1;
            }
            n >>= 1;
            k >>= 1;
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_3226() {
        assert_eq!(Solution::min_changes(13, 4), 2);
        assert_eq!(Solution::min_changes(21, 21), 0);
        assert_eq!(Solution::min_changes(14, 13), -1);
    }
}