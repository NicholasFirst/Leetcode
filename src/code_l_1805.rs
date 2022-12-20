struct Solution;

impl Solution {
    // https://leetcode.cn/problems/number-of-different-integers-in-a-string/
    #[allow(dead_code)]
    pub fn num_different_integers(word: String) -> i32 {
        word
            .split(|c: char| !c.is_ascii_digit())
            .filter(|s| !s.is_empty())
            .map(|s| s.trim_start_matches('0'))
            .collect::<std::collections::HashSet<_>>()
            .len() as i32
    }
}


#[cfg(test)]
mod test{
    use super::Solution;

    #[test]
    fn test() {
        let word = "a123bc34d8ef34".to_string();
        let res = Solution::num_different_integers(word);
        assert_eq!(3, res);
    }

    #[test]
    fn test1() {
        let word = "167278959591294".to_string();
        let res = Solution::num_different_integers(word);
        assert_eq!(1, res);
    }
}