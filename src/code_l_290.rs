struct Solution;

use std::collections::HashMap;

impl Solution {
    // https://leetcode.cn/problems/word-pattern/
    #[allow(dead_code)]
    pub fn word_pattern(pattern: String, s: String) -> bool {
        let s = s.split_whitespace().collect::<Vec<_>>();
        if s.len() != pattern.len() {
            return false;
        }
        let p = pattern.chars().collect::<Vec<_>>();
        let mut sp = HashMap::new();
        let mut ps = HashMap::new();
        for i in 0..s.len() {
            if let Some(ss) = ps.insert(p[i], s[i]) {
                if ss != s[i] {
                    return false;
                }
            }
            if let Some(c) = sp.insert(s[i], p[i]) {
                if c != p[i] {
                    return false;
                }
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(Solution::word_pattern("abba".to_string(), "dog cat cat dog".to_string()), true);
        assert_eq!(Solution::word_pattern("abba".to_string(), "dog cat cat fish".to_string()), false);
        assert_eq!(Solution::word_pattern("aaaa".to_string(), "dog cat cat dog".to_string()), false);
        assert_eq!(Solution::word_pattern("abba".to_string(), "dog dog dog dog".to_string()), false);
    }
}