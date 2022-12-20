struct Solution;

impl Solution {
    // https://leetcode.cn/problems/check-if-two-string-arrays-are-equivalent/
    #[allow(dead_code)]
    pub fn array_strings_are_equal(word1: Vec<String>, word2: Vec<String>) -> bool {
        // word1.join("") == word2.join("")
        // word1.iter().fold("".to_string(), |mut ans1, w| { ans1.push_str(&*w); ans1 }) == word2.iter().fold("".to_string(), |mut ans2, w| { ans2.push_str(&*w); ans2 })
        word1.concat() == word2.concat()
    }
}

#[cfg(test)]
mod test{
    use super::Solution;

    #[test]
    fn test() {
        let word1 = vec!["a".to_string(), "bc".to_string()];
        let word2 = vec!["ab".to_string(),"c".to_string()];
        let eq = Solution::array_strings_are_equal(word1, word2);
        assert!(eq);
    }
}