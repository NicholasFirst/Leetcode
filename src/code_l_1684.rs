struct Solution;

impl Solution {
    // https://leetcode.cn/problems/count-the-number-of-consistent-strings/
    #[allow(dead_code)]
    pub fn count_consistent_strings(allowed: String, words: Vec<String>) -> i32 {
        use std::collections::HashSet;

        let allowed_set: HashSet<_> = allowed.chars().collect();
        let mut ans = 0;
        for s in words {
            let sc: HashSet<_> = s.chars().collect();
            if sc.is_subset(&allowed_set) {
                ans += 1;
            }
        }
        ans
    }
}

#[cfg(test)]
mod test{
    use super::Solution;

    #[test]
    fn test() {
        let allowed = "ab".to_string();
        let words = vec!["ad".to_string(),"bd".to_string(),"aaab".to_string(),"baa".to_string(),"badab".to_string()];
        let res = Solution::count_consistent_strings(allowed, words);
        assert_eq!(2, res);
    }
}