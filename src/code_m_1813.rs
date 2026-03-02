struct Solution;

impl Solution {

    // https://leetcode.cn/problems/sentence-similarity-iii/
    #[allow(dead_code)]
    pub fn are_sentences_similar(sentence1: String, sentence2: String) -> bool {
        let s1 = sentence1.split_whitespace().collect::<Vec<_>>();
        let s2 = sentence2.split_whitespace().collect::<Vec<_>>();
        let s = s1.len().min(s2.len());
        
        let l = s1.iter().zip(s2.iter())
                .take_while(|(s1, s2)| s1 == s2)
                .count();

        let r = s1.iter().rev().zip(s2.iter().rev())
                .take(s-l)      // 只找剩余个数
                .take_while(|(s1, s2)| s1 == s2)
                .count();
        s == l + r
    }
}

#[cfg(test)]
mod test{
    use super::Solution;

    #[test]
    fn test() {
        let sentence1 = "My name is Haley".to_string();
        let sentence2 = "My Haley".to_string();
        let res = Solution::are_sentences_similar(sentence1, sentence2);
        assert!(res);
    }

    #[test]
    fn test1() {
        let sentence1 = "of".to_string();
        let sentence2 = "A lot of words".to_string();
        let res = Solution::are_sentences_similar(sentence1, sentence2);
        assert!(!res);
    }

    #[test]
    fn test2() {
        let sentence1 = "Eating right now".to_string();
        let sentence2 = "Eating".to_string();
        let res = Solution::are_sentences_similar(sentence1, sentence2);
        assert_eq!(true, res);
    }

    #[test]
    fn test3() {
        let sentence1 = "CwFfRo regR".to_string();
        let sentence2 = "CwFfRo H regR".to_string();
        let res = Solution::are_sentences_similar(sentence1, sentence2);
        assert_eq!(true, res);
    }
}