struct Solution;

impl Solution {

    // https://leetcode.cn/problems/counting-words-with-a-given-prefix/description/
    #[allow(dead_code)]
    pub fn prefix_count(words: Vec<String>, pref: String) -> i32 {
        words.iter().filter(|&s| s.starts_with(&pref)).count() as i32
    }
}

#[cfg(test)]
mod test{
    use super::Solution;

    #[test]
    fn test() {
        let words = vec!["pay".to_string(),"attention".to_owned(),"practice".to_owned(),"attend".to_owned()];
        let pref = "at".to_string();
        let sol = Solution::prefix_count(words, pref);
        assert_eq!(sol, 2);
    }
}
