struct Solution;

use std::collections::HashSet;

impl Solution {
    // https://leetcode.cn/problems/check-if-the-sentence-is-pangram/
    #[allow(dead_code)]
    pub fn check_if_pangram(sentence: String) -> bool {
        if sentence.len() < 26 {
            return false;
        }
        let mut set = HashSet::with_capacity(26);
        sentence.as_bytes().iter().for_each(|x| {
            set.insert(x);
        });
        set.len() == 26
    }
}

#[cfg(test)]
mod test{
    use super::Solution;

    #[test]
    fn test() {
        let sentence = "thequickbrownfoxjumpsoverthelazydog".to_string();
        let ans = Solution::check_if_pangram(sentence);
        assert!(ans);
    }
}