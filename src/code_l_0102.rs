struct Solution;

impl Solution {
    // https://leetcode.cn/problems/check-permutation-lcci/
    #[allow(dead_code)]
    pub fn check_permutation(s1: String, s2: String) -> bool {
        if s1.len() != s2.len() {
            return false;
        }
        let mut x: Vec<char> = s1.chars().collect();
        let mut y: Vec<char> = s2.chars().collect();
        x.sort();
        y.sort();
        x == y
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test() {
        let s1 = "abc".to_string();
        let s2 = "bac".to_string();
        let res = Solution::check_permutation(s1, s2);
        assert!(res);
    }

    #[test]
    fn test1() {
        let s1 = "abc".to_string();
        let s2 = "bad".to_string();
        let res = Solution::check_permutation(s1, s2);
        assert!(!res);
    }
}