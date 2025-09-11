struct Solution;

impl Solution {

    // https://leetcode.cn/problems/find-the-lexicographically-largest-string-from-the-box-i/description/?envType=daily-question&envId=2025-06-04
    #[allow(dead_code)]
    pub fn answer_string(word: String, num_friends: i32) -> String {
        if num_friends == 1 {
            return word;
        }
        let n = word.len();
        let mut res = String::new();
        for i in 0 ..= n - 1 {
            let s = &word[i .. std::cmp::min(i + n - (num_friends as usize) + 1, n)];
            if res < s.to_string() {
                res = s.to_string();
            }
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_3403() {
        assert_eq!(Solution::answer_string("dbca".to_string(), 2), "dbc".to_string());
        assert_eq!(Solution::answer_string("gggg".to_string(), 4), "g".to_string());
    }
}