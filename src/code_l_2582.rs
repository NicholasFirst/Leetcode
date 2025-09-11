struct Solution;

impl Solution {
    #[allow(dead_code)]
    // https://leetcode.cn/problems/pass-the-pillow/?envType=daily-question&envId=2024-01-19
    pub fn pass_the_pillow(n: i32, time: i32) -> i32 {
        let k = time / (n - 1);
        let mut _mod = time % (n - 1);
        if k & 1 == 1 {
            return n - _mod
        }
        _mod + 1
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pass_the_pillow() {
        let n = 4; 
        let time = 5;
        let result = Solution::pass_the_pillow(n, time);
        assert_eq!(result, 2);
    }

    #[test]
    fn test_pass_the_pillow_2() {
        let n = 3;
        let time = 2;
        let result = Solution::pass_the_pillow(n, time);
        assert_eq!(result, 3);
    }
}