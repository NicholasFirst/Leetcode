struct Solution;

impl Solution {
    // https://leetcode.cn/problems/count-prefixes-of-a-given-string/description/?envType=daily-question&envId=2025-03-24
    #[allow(dead_code)]
    fn count_prefixes(words: Vec<String>, s: String) -> i32 {
        let mut count = 0;
        for word in words {
            if s.starts_with(&word) {
                count += 1;
            }
        }
        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2255() {
        assert_eq!(Solution::count_prefixes(vec!["a".to_string(), "b".to_string(), "c".to_string(), "ab".to_string(), "bc".to_string(), "abc".to_string()], "abc".to_string()), 3);
        assert_eq!(Solution::count_prefixes(vec!["a".to_string(),"a".to_string()], "aa".to_string()), 2);
    }
}