struct Solution;

impl Solution {
    /// https://leetcode.cn/problems/string-matching-in-an-array/
    #[allow(dead_code)]
    pub fn string_matching(words: Vec<String>) -> Vec<String> {
        let sentence = words.join(" ");
        words.into_iter().filter(|word| sentence.matches(word).nth(1).is_some()).collect::<Vec<_>>()
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test() {
        let words = vec![
            "mass".to_string(),
            "as".to_string(),
            "hero".to_string(),
            "superhero".to_string(),
        ];
        let res = Solution::string_matching(words);
        assert!(res.contains(&"hero".to_string()));
        assert!(res.contains(&"as".to_string()));
    }
}
