struct Solution;

impl Solution {
    // https://leetcode.cn/problems/first-letter-to-appear-twice/description/
    #[allow(dead_code)]
    pub fn repeated_character(s: String) -> char {
        use std::collections::HashSet;
        let mut set = HashSet::with_capacity(s.len());
        for c in s.as_bytes().iter() {
            if set.contains(c) {
                return *c as char;
            }else{
                set.insert(c);
            }
        }
        panic!("Error");
    }
}

#[cfg(test)]
mod test{
    use super::Solution;

    #[test]
    fn test() {
        let s = "abccbaacz".to_string();
        let c = Solution::repeated_character(s);
        assert_eq!(c, 'c');
    }
}