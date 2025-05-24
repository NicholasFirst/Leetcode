struct Solution;

impl Solution {
    // https://leetcode.cn/problems/find-words-containing-character/?envType=daily-question&envId=2025-05-24
    #[allow(dead_code)]
    pub fn find_words_containing(words: Vec<String>, x: char) -> Vec<i32> {
        // words.iter().enumerate().filter(
        //     |(_, s)| s.contains(x)
        // ).map(|(i, _)| i as i32).collect::<Vec<i32>>()
        words.iter().enumerate().filter_map(|(i, s)| {
            if s.contains(x) {
                Some(i as i32)
            } else {
                None
            }
        }).collect::<Vec<i32>>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(Solution::find_words_containing(vec!["hello".to_string(), "world".to_string(), "leetcode".to_string()], 'e'), vec![0, 2]);
        assert_eq!(Solution::find_words_containing(vec!["leetcode".to_string(), "hello".to_string(), "world".to_string(), "leetcode".to_string()], 'e'), vec![0, 1, 3]);
    }
}